use anyhow::Result;
use async_trait::async_trait;
use aws_config::{BehaviorVersion, SdkConfig, meta::region::RegionProviderChain};
use aws_sdk_dynamodb::{Client, config::Credentials};

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

    async fn insert_bacct(&self, _bacct: &dyn IBrokerageAccount) -> Result<()> {
        // let mdb_bacct = bacct
        //     .as_any()
        //     .downcast_ref::<DynamoDbBrokerageAccount>()
        //     .unwrap();
        todo!();
        // db_util::insert(
        //     mdb_bacct,
        //     &self.db,
        //     MdbBrokerageAccount::COLLECTION_NAME,
        //     session,
        // )
        // .await?;
        // Ok(())
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
        _brokerage_id: &str,
        _account_id: &str,
    ) -> Result<Option<Box<dyn IBrokerageAccount>>> {
        todo!()
        // let result = self
        //     .db
        //     .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
        //     .find_one(bson::doc! {
        //     "brokerage_id": brokerage_id,
        //     "account_id": account_id})
        //     .await?;
        // if let Some(bacct) = result {
        //     Ok(Some(Box::new(bacct)))
        // } else {
        //     Ok(None)
        // }
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
