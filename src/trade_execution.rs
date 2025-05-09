use crate::{account::BrokerageAccount, db_util, security::Security};
use anyhow::Result;
use bson::oid::ObjectId;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TradeExecution {
    _id: bson::oid::ObjectId,
    brokerage_account_id: bson::oid::ObjectId,
    brokerage_execution_id: String,
    execution_timestamp_ms: i64,
    commission: f64,
    quantity: u64,
    price: f64,
    security_id: bson::oid::ObjectId,
    side: TradeSide,
}

impl TradeExecution {
    pub const COLLECTION_NAME: &'static str = "trade_executions";

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn id(&self) -> ObjectId {
        self._id
    }

    pub fn brokerage_account_id(&self) -> ObjectId {
        self.brokerage_account_id
    }

    pub fn brokerage_execution_id(&self) -> &str {
        &self.brokerage_execution_id
    }

    pub fn execution_timestamp_ms(&self) -> i64 {
        self.execution_timestamp_ms
    }

    pub fn commission(&self) -> f64 {
        self.commission
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn security_id(&self) -> ObjectId {
        self.security_id
    }

    pub fn side(&self) -> &TradeSide {
        &self.side
    }

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

    pub async fn find_by_brokerage_execution_id(
        db: &Database,
        execution_id: &str,
    ) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"brokerage_execution_id": execution_id})
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

pub struct Builder {
    _id: bson::oid::ObjectId,
    brokerage_account_id: Option<bson::oid::ObjectId>,
    brokerage_execution_id: Option<String>,
    execution_timestamp_ms: Option<i64>,
    commission: Option<f64>,
    quantity: Option<u64>,
    price: Option<f64>,
    security_id: Option<bson::oid::ObjectId>,
    side: Option<TradeSide>,
}

impl Builder {
    fn new() -> Self {
        Self {
            _id: ObjectId::new(),
            brokerage_account_id: None,
            brokerage_execution_id: None,
            execution_timestamp_ms: None,
            commission: None,
            quantity: None,
            price: None,
            security_id: None,
            side: None,
        }
    }

    pub fn from_trade_execution(trade_execution: &TradeExecution) -> Self {
        Self {
            _id: ObjectId::new(),
            brokerage_account_id: Some(trade_execution.brokerage_account_id),
            brokerage_execution_id: Some(trade_execution.brokerage_execution_id.clone()),
            execution_timestamp_ms: Some(trade_execution.execution_timestamp_ms),
            commission: Some(trade_execution.commission),
            quantity: Some(trade_execution.quantity),
            price: Some(trade_execution.price),
            security_id: Some(trade_execution.security_id),
            side: Some(trade_execution.side.clone()),
        }
    }

    pub fn brokerage_account_id(mut self, id: bson::oid::ObjectId) -> Self {
        self.brokerage_account_id = Some(id);
        self
    }
    pub fn brokerage_execution_id(mut self, id: &str) -> Self {
        self.brokerage_execution_id = Some(id.to_owned());
        self
    }
    pub fn execution_timestamp_ms(mut self, timestamp: i64) -> Self {
        self.execution_timestamp_ms = Some(timestamp);
        self
    }
    pub fn commission(mut self, commission: f64) -> Self {
        self.commission = Some(commission);
        self
    }

    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn security_id(mut self, id: bson::oid::ObjectId) -> Self {
        self.security_id = Some(id);
        self
    }

    pub fn side(mut self, side: TradeSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn build(self) -> Result<TradeExecution> {
        Ok(TradeExecution {
            _id: self._id,
            brokerage_account_id: self.brokerage_account_id.unwrap(),
            brokerage_execution_id: self.brokerage_execution_id.unwrap(),
            execution_timestamp_ms: self.execution_timestamp_ms.unwrap(),
            commission: self.commission.unwrap(),
            quantity: self.quantity.unwrap(),
            price: self.price.unwrap(),
            security_id: self.security_id.unwrap(),
            side: self.side.unwrap(),
        })
    }
}
