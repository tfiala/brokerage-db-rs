use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BrokerageAccount {
    pub _id: bson::oid::ObjectId,
    pub brokerage_id: String,
    pub account_id: String,
}

impl BrokerageAccount {
    pub const COLLECTION_NAME: &'static str = "brokerage_accounts";
}
