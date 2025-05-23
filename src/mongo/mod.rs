mod account;
pub mod migrations;

use crate::{
    account::IBrokerageAccount, db_connection::DbConnection,
    db_connection_factory::DbConnectionFactory, db_util,
};
use account::MdbBrokerageAccount;
use anyhow::Result;
use async_trait::async_trait;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::{Client, Database};

pub struct MdbConnectionFactory {
    uri: String,
    db_name: String,
}

impl MdbConnectionFactory {
    pub fn new(uri: &str, db_name: &str) -> Self {
        Self {
            db_name: db_name.to_owned(),
            uri: uri.to_owned(),
        }
    }
}

#[async_trait]
impl DbConnectionFactory for MdbConnectionFactory {
    fn id(&self) -> &'static str {
        "mongodb"
    }

    async fn create(&self) -> Result<Box<dyn DbConnection>> {
        let client = mongodb::Client::with_uri_str(self.uri.clone()).await?;
        let db = client.database(&self.db_name);

        Ok(Box::new(MdbDbConnection {
            _client: client,
            db,
        }))
    }
}

#[derive(Debug)]
pub struct MdbDbConnection {
    _client: Client,
    db: Database,
}

#[async_trait]
impl DbConnection for MdbDbConnection {
    //
    // migrations
    //

    async fn run_migrations(&self) -> Result<()> {
        migrations::run_migrations(&self.db).await
    }

    async fn remove_migrations(&self) -> Result<()> {
        migrations::run_down_migrations(&self.db).await
    }

    //
    // Brokerage Accounts
    //
    fn new_brokerage_account(
        &self,
        account_id: &str,
        brokerage_id: &str,
    ) -> Box<dyn IBrokerageAccount + Send> {
        Box::new(MdbBrokerageAccount {
            _id: ObjectId::new(),
            account_id: account_id.to_owned(),
            brokerage_id: brokerage_id.to_owned(),
        })
    }

    async fn insert_bacct(&self, bacct: &dyn IBrokerageAccount) -> Result<()> {
        let mdb_bacct = bacct
            .as_any()
            .downcast_ref::<MdbBrokerageAccount>()
            .unwrap();
        // TODO figure out session handling
        let session = None;
        db_util::insert(
            &mdb_bacct,
            &self.db,
            MdbBrokerageAccount::COLLECTION_NAME,
            session,
        )
        .await?;
        Ok(())
    }

    async fn update_bacct(&self, _bacct: &dyn IBrokerageAccount) -> Result<()> {
        todo!()
    }

    async fn find_bacct_all(&self) -> Result<Vec<Box<dyn IBrokerageAccount>>> {
        let mdb_baccts = self
            .db
            .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
            .find(bson::doc! {})
            .await?
            .try_collect::<Vec<MdbBrokerageAccount>>()
            .await?;
        let boxed_baccts = mdb_baccts
            .into_iter()
            .map(|bacct| {
                let boxed_bacct: Box<dyn IBrokerageAccount> = Box::new(bacct);
                boxed_bacct
            })
            .collect();
        Ok(boxed_baccts)
    }

    async fn find_bacct_by_brokerage_and_account_id(
        &self,
        brokerage_id: &str,
        account_id: &str,
    ) -> Result<Option<Box<dyn IBrokerageAccount>>> {
        let result = self
            .db
            .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
            .find_one(bson::doc! {
            "brokerage_id": brokerage_id,
            "account_id": account_id})
            .await?;
        if let Some(bacct) = result {
            Ok(Some(Box::new(bacct)))
        } else {
            Ok(None)
        }
    }

    // async fn find_bacct_by_dbid(
    //     &self,
    //     dbid: &ObjectId,
    // ) -> Result<Option<Box<dyn IBrokerageAccount<ObjectId>>>> {
    //     let result = self
    //         .db
    //         .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
    //         .find_one(bson::doc! {"_id": dbid})
    //         .await?;

    //     if let Some(bacct) = result {
    //         Ok(Some(Box::new(bacct)))
    //     } else {
    //         Ok(None)
    //     }
    // }
}
