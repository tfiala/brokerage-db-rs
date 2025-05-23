use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

use crate::account::IBrokerageAccount;
use std::{any::Any, collections::HashMap};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DynamoBrokerageAccount {
    // account_id and brokerage_id are part of the compound primary key.
    pub pk: String,
    pub account_id: String,
    pub brokerage_id: String,
}

impl IBrokerageAccount for DynamoBrokerageAccount {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn brokerage_id(&self) -> &str {
        &self.brokerage_id
    }
}

impl DynamoBrokerageAccount {
    pub const TABLE_NAME: &str = "brokerage_accounts";

    pub const PK_COL_NAME: &str = "PK";
    pub const ACCOUNT_ID_COL_NAME: &str = "account_id";
    pub const BROKERAGE_ID_COL_NAME: &str = "brokerage_id";

    pub fn new(brokerage_id: &str, account_id: &str) -> Self {
        Self {
            pk: Self::pk(brokerage_id, account_id),
            account_id: account_id.to_owned(),
            brokerage_id: brokerage_id.to_owned(),
        }
    }

    pub fn pk(brokerage_id: &str, account_id: &str) -> String {
        format!("{}#{}", brokerage_id, account_id)
    }
}

impl From<&HashMap<String, AttributeValue>> for DynamoBrokerageAccount {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let account_id = value
            .get(Self::ACCOUNT_ID_COL_NAME)
            .unwrap()
            .as_s()
            .unwrap();

        let brokerage_id = value
            .get(Self::BROKERAGE_ID_COL_NAME)
            .unwrap()
            .as_s()
            .unwrap();

        DynamoBrokerageAccount::new(brokerage_id, account_id)
    }
}
