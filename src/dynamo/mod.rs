use anyhow::Result;
use async_trait::async_trait;
use aws_config::{BehaviorVersion, SdkConfig, meta::region::RegionProviderChain};
use aws_sdk_dynamodb::{Client, config::Credentials, types::AttributeValue};
use tracing::{debug, info};

use crate::{
    account::IBrokerageAccount, db_connection::DbConnection,
    db_connection_factory::DbConnectionFactory,
};
use account::DynamoBrokerageAccount;

mod account;
mod migrations;

pub struct DynamoDbConnectionFactory {
    endpoint_url: Option<String>,
    access_key_id: String,
    access_key_secret: String,
}

impl DynamoDbConnectionFactory {
    pub fn new(access_key_id: &str, access_key_secret: &str, endpoint_url: Option<String>) -> Self {
        DynamoDbConnectionFactory {
            access_key_id: access_key_id.to_owned(),
            access_key_secret: access_key_secret.to_owned(),
            endpoint_url,
        }
    }

    async fn sdk_config(&self) -> SdkConfig {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let creds = Credentials::new(
            &self.access_key_id,
            &self.access_key_secret,
            None,
            None,
            "test",
        );

        let mut config_loader = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .credentials_provider(creds);

        if let Some(endpoint_url) = &self.endpoint_url {
            config_loader = config_loader.endpoint_url(endpoint_url);
        }

        config_loader.load().await
    }
}

#[async_trait]
impl DbConnectionFactory for DynamoDbConnectionFactory {
    fn id(&self) -> &'static str {
        "dynamodb"
    }

    async fn create(&self) -> Result<Box<dyn DbConnection>> {
        let sdk_config = self.sdk_config().await;
        let client = Client::new(&sdk_config);
        Ok(Box::new(DynamoDbConnection { client }))
    }
}

pub struct DynamoDbConnection {
    client: Client,
}

#[async_trait]
impl DbConnection for DynamoDbConnection {
    //
    // Migrations
    //

    async fn run_migrations(&self) -> Result<()> {
        info!("dynamodb: running migrations");
        migrations::run_migrations(&self.client).await
    }

    async fn remove_migrations(&self) -> Result<()> {
        migrations::remove_migrations(&self.client).await
    }

    //
    // Brokerage Accounts
    //
    fn new_brokerage_account(
        &self,
        account_id: &str,
        brokerage_id: &str,
    ) -> Box<dyn IBrokerageAccount + Send> {
        Box::new(DynamoBrokerageAccount::new(brokerage_id, account_id))
    }

    async fn insert_bacct(&self, bacct: &dyn IBrokerageAccount) -> Result<()> {
        let mdb_bacct = bacct
            .as_any()
            .downcast_ref::<DynamoBrokerageAccount>()
            .unwrap();

        let pk_av = AttributeValue::S(mdb_bacct.pk.clone());
        let account_id_av = AttributeValue::S(mdb_bacct.account_id.clone());
        let brokerage_id_av = AttributeValue::S(mdb_bacct.brokerage_id.clone());

        let request = self
            .client
            .put_item()
            .table_name(DynamoBrokerageAccount::TABLE_NAME)
            .item(DynamoBrokerageAccount::PK_COL_NAME, pk_av)
            .item(DynamoBrokerageAccount::ACCOUNT_ID_COL_NAME, account_id_av)
            .item(
                DynamoBrokerageAccount::BROKERAGE_ID_COL_NAME,
                brokerage_id_av,
            )
            .condition_expression(format!(
                "attribute_not_exists({})",
                DynamoBrokerageAccount::PK_COL_NAME
            ));

        debug!("dynamodb: inserting brokerage account: {:?}", request);

        let response = request.send().await?;
        info!("dynamodb: insert brokerage account result: {:?}", response);

        Ok(())
    }

    async fn update_bacct(&self, _bacct: &dyn IBrokerageAccount) -> Result<()> {
        todo!()
    }

    async fn find_bacct_all(&self) -> Result<Vec<Box<dyn IBrokerageAccount>>> {
        todo!()
        // let mdb_baccts = self
        //     .db
        //     .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
        //     .find(bson::doc! {})
        //     .await?
        //     .try_collect::<Vec<MdbBrokerageAccount>>()
        //     .await?;
        // let boxed_baccts = mdb_baccts
        //     .into_iter()
        //     .map(|bacct| {
        //         let boxed_bacct: Box<dyn IBrokerageAccount<ObjectId>> = Box::new(bacct);
        //         boxed_bacct
        //     })
        //     .collect();
        // Ok(boxed_baccts)
    }

    async fn find_bacct_by_brokerage_and_account_id(
        &self,
        brokerage_id: &str,
        account_id: &str,
    ) -> Result<Option<Box<dyn IBrokerageAccount>>> {
        let results = self
            .client
            .query()
            .table_name(DynamoBrokerageAccount::TABLE_NAME)
            .key_condition_expression(format!(
                "#{} = :{}",
                DynamoBrokerageAccount::PK_COL_NAME,
                DynamoBrokerageAccount::PK_COL_NAME,
            ))
            .expression_attribute_names(
                format!("#{}", DynamoBrokerageAccount::PK_COL_NAME),
                DynamoBrokerageAccount::PK_COL_NAME,
            )
            .expression_attribute_names(
                format!("#{}", DynamoBrokerageAccount::PK_COL_NAME),
                DynamoBrokerageAccount::PK_COL_NAME,
            )
            .expression_attribute_values(
                format!(":{}", DynamoBrokerageAccount::PK_COL_NAME),
                AttributeValue::S(DynamoBrokerageAccount::pk(brokerage_id, account_id)),
            )
            .send()
            .await?;

        if let Some(items) = results.items {
            let baccts: Vec<DynamoBrokerageAccount> = items.iter().map(|v| v.into()).collect();
            if baccts.len() == 1 {
                Ok(Some(Box::new(baccts[0].clone())))
            } else if baccts.len() > 1 {
                Err(anyhow::anyhow!(
                    "dynamodb: found multiple brokerage accounts for account_id: {}, brokerage_id: {}",
                    account_id,
                    brokerage_id
                ))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    // async fn find_bacct_by_dbid(
    //     &self,
    //     _dbid: &String,
    // ) -> Result<Option<Box<dyn IBrokerageAccount<String>>>> {
    //     todo!()
    //     // let result = self
    //     //     .db
    //     //     .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
    //     //     .find_one(bson::doc! {"_id": dbid})
    //     //     .await?;

    //     // if let Some(bacct) = result {
    //     //     Ok(Some(Box::new(bacct)))
    //     // } else {
    //     //     Ok(None)
    //     // }
    // }
}
