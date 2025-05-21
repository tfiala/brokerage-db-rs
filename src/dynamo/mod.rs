use anyhow::Result;
use async_trait::async_trait;

use crate::{account::IBrokerageAccount, db_connection::DbConnection};
use account::DynamoDbBrokerageAccount;

mod account;

pub struct DynamoDbConnection {}

#[async_trait]
impl DbConnection<String> for DynamoDbConnection {
    //
    // Migrations
    //

    async fn run_migrations(&self) -> Result<()> {
        todo!()
    }

    async fn remove_migrations(&self) -> Result<()> {
        todo!()
    }

    //
    // Brokerage Accounts
    //
    fn new_brokerage_account(
        &self,
        account_id: &str,
        brokerage_id: &str,
    ) -> Box<dyn IBrokerageAccount<String>> {
        Box::new(DynamoDbBrokerageAccount::new(brokerage_id, account_id))
    }

    async fn insert_bacct(&self, _bacct: Box<dyn IBrokerageAccount<String> + Send>) -> Result<()> {
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

    async fn update_bacct(&self, _bacct: Box<dyn IBrokerageAccount<String> + Send>) -> Result<()> {
        todo!()
    }

    async fn find_bacct_all(&self) -> Result<Vec<Box<dyn IBrokerageAccount<String>>>> {
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
    ) -> Result<Option<Box<dyn IBrokerageAccount<String>>>> {
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

    async fn find_bacct_by_dbid(
        &self,
        _dbid: &String,
    ) -> Result<Option<Box<dyn IBrokerageAccount<String>>>> {
        todo!()
        // let result = self
        //     .db
        //     .collection::<MdbBrokerageAccount>(MdbBrokerageAccount::COLLECTION_NAME)
        //     .find_one(bson::doc! {"_id": dbid})
        //     .await?;

        // if let Some(bacct) = result {
        //     Ok(Some(Box::new(bacct)))
        // } else {
        //     Ok(None)
        // }
    }
}
