use crate::{account::BrokerageAccount, db_util, security::Security};
use anyhow::Result;
use bson::oid::ObjectId;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TradeExecution {
    pub _id: bson::oid::ObjectId,
    pub brokerage_account_id: bson::oid::ObjectId,
    pub brokerage_execution_id: String,
    pub execution_timestamp_ms: i64,
    pub commission: f64,
    pub quantity: u64,
    pub price: f64,
    pub security_id: bson::oid::ObjectId,
    pub side: TradeSide,
}

impl TradeExecution {
    pub const COLLECTION_NAME: &'static str = "trade_executions";

    pub async fn insert(&self, db: &Database, session: Option<&mut ClientSession>) -> Result<()> {
        db_util::insert(self, db, Self::COLLECTION_NAME, session).await
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"_id": id})
            .await?;

        Ok(result)
    }

    pub async fn brokerage_account(&self, db: &Database) -> Result<BrokerageAccount> {
        Ok(BrokerageAccount::find_by_id(db, self.brokerage_account_id)
            .await?
            .unwrap())
    }

    pub async fn security(&self, db: &Database) -> Result<Security> {
        Ok(Security::find_by_id(db, self.security_id).await?.unwrap())
    }
}
