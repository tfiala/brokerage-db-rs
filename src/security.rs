use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SecurityType {
    Stock,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Security {
    pub _id: bson::oid::ObjectId,
    pub listing_exchange: String,
    pub security_type: SecurityType,
    pub ticker: String,
    pub ibkr_conid: Option<u32>,
}

impl Security {
    pub const COLLECTION_NAME: &'static str = "securities";
}
