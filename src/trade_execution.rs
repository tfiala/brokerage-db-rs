use crate::{account::BrokerageAccount, security::Security};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TradeSide {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeExecution {
    pub _id: bson::oid::ObjectId,
    pub brokerage_account: BrokerageAccount,
    pub commission: f64,
    pub quantity: u64,
    pub price: f64,
    pub security: Security,
    pub side: TradeSide,
}
