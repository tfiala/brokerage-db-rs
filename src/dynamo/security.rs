use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

use crate::security::{ISecurity, SecurityType};
use std::{any::Any, collections::HashMap};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DynamoSecurity {
    // account_id and brokerage_id are part of the compound primary key.
    pub pk: String,
    pub listing_exchange: String,
    pub security_type: SecurityType,
    pub ticker: String,
    pub ibkr_conid: Option<u32>,
}

impl<'a> ISecurity<'a> for DynamoSecurity {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn ibkr_conid(&self) -> Option<u32> {
        self.ibkr_conid
    }

    fn listing_exchange(&'a self) -> &'a str {
        &self.listing_exchange
    }

    fn security_type(&'a self) -> &'a SecurityType {
        &self.security_type
    }

    fn ticker(&'a self) -> &'a str {
        &self.ticker
    }
}

impl DynamoSecurity {
    pub const TABLE_NAME: &str = "securities";

    pub const PK_COL_NAME: &str = "PK";

    pub const LISTING_EXCHANGE_COL_NAME: &str = "listing_exchange";
    pub const SECURITY_TYPE_COL_NAME: &str = "security_type";
    pub const TICKER_COL_NAME: &str = "ticker";
    pub const IBKR_CONID_COL_NAME: &str = "ibkr_conid";

    pub fn new(
        security_type: SecurityType,
        ticker: &str,
        listing_exchange: &str,
        ibkr_conid: Option<u32>,
    ) -> Self {
        Self {
            pk: Self::pk(security_type.clone(), ticker, listing_exchange),
            ibkr_conid,
            listing_exchange: listing_exchange.to_owned(),
            security_type: security_type,
            ticker: ticker.to_owned(),
        }
    }

    pub fn pk(security_type: SecurityType, ticker: &str, listing_exchange: &str) -> String {
        let st_str: &str = security_type.into();
        format!("{}#{}#{}", ticker, listing_exchange, st_str)
    }
}

impl From<&HashMap<String, AttributeValue>> for DynamoSecurity {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let listing_exchange = value
            .get(Self::LISTING_EXCHANGE_COL_NAME)
            .unwrap()
            .as_s()
            .unwrap();

        let security_type_str = value
            .get(Self::SECURITY_TYPE_COL_NAME)
            .unwrap()
            .as_s()
            .unwrap();

        let security_type: SecurityType = security_type_str.into();

        let ticker = value.get(Self::TICKER_COL_NAME).unwrap().as_s().unwrap();

        let ibkr_conid = {
            match value.get(Self::IBKR_CONID_COL_NAME) {
                Some(attribute) => {
                    let conid: u32 = attribute.as_s().unwrap().parse().unwrap();
                    Some(conid)
                }
                None => None,
            }
        };

        DynamoSecurity::new(security_type, ticker, listing_exchange, ibkr_conid)
    }
}
