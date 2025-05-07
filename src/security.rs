use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SecurityType {
    Stock,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Security {
    pub _id: bson::oid::ObjectId,
    pub listing_exchange: String,
    pub security_type: SecurityType,
    pub ticker: String,
}
