use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{any::Any, fmt::Debug};

use crate::security::{ISecurity, SecurityType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MdbSecurity {
    _id: ObjectId,
    ibkr_conid: Option<u32>,
    listing_exchange: String,
    security_type: SecurityType,
    ticker: String,
}

impl<'a> MdbSecurity {
    pub const COLLECTION_NAME: &'static str = "securities";

    pub fn new(
        security_type: SecurityType,
        ticker: &str,
        listing_exchange: &str,
        ibkr_conid: Option<u32>,
    ) -> Self {
        Self {
            _id: ObjectId::new(),
            ibkr_conid,
            listing_exchange: listing_exchange.to_owned(),
            security_type: security_type,
            ticker: ticker.to_owned(),
        }
    }
}

impl<'a> ISecurity<'a> for MdbSecurity {
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
