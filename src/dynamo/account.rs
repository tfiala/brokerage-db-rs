use serde::{Deserialize, Serialize};

use crate::account::IBrokerageAccount;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DynamoDbBrokerageAccount {
    pub dbid: String,
    pub account_id: String,
    pub brokerage_id: String,
}

impl IBrokerageAccount<String> for DynamoDbBrokerageAccount {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dbid(&self) -> &String {
        &self.dbid
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn brokerage_id(&self) -> &str {
        &self.brokerage_id
    }
}

impl DynamoDbBrokerageAccount {
    pub fn new(brokerage_id: &str, account_id: &str) -> Self {
        Self {
            dbid: format!("{}#{}", brokerage_id, account_id),
            account_id: account_id.to_owned(),
            brokerage_id: brokerage_id.to_owned(),
        }
    }
}
