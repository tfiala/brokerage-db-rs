use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{any::Any, fmt::Debug};

use crate::account::IBrokerageAccount;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MdbBrokerageAccount {
    pub _id: ObjectId,
    pub brokerage_id: String,
    pub account_id: String,
}

impl MdbBrokerageAccount {
    pub const COLLECTION_NAME: &'static str = "brokerage_accounts";
}

impl IBrokerageAccount<ObjectId> for MdbBrokerageAccount {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dbid(&self) -> &ObjectId {
        &self._id
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn brokerage_id(&self) -> &str {
        &self.brokerage_id
    }
}
