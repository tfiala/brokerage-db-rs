use std::{any::Any, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::Mutex;

use crate::db_util;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SecurityType {
    Stock,
}

impl From<&str> for SecurityType {
    fn from(value: &str) -> Self {
        match value {
            "stock" => SecurityType::Stock,
            _ => panic!("unknown security type {}", value),
        }
    }
}

impl From<&String> for SecurityType {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "stock" => SecurityType::Stock,
            _ => panic!("unknown security type {}", value),
        }
    }
}

impl Into<&'static str> for SecurityType {
    fn into(self) -> &'static str {
        match self {
            SecurityType::Stock => "stock",
        }
    }
}

impl Into<String> for SecurityType {
    fn into(self) -> String {
        let st_str: &str = self.into();
        st_str.to_owned()
    }
}

#[async_trait]
pub trait ISecurity<'a>: Send + Sync + fmt::Debug {
    fn as_any(&self) -> &dyn Any;

    fn ibkr_conid(&self) -> Option<u32>;
    fn listing_exchange(&'a self) -> &'a str;
    fn security_type(&'a self) -> &'a SecurityType;
    fn ticker(&'a self) -> &'a str;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Security {
    _id: bson::oid::ObjectId,
    listing_exchange: String,
    security_type: SecurityType,
    ticker: String,
    ibkr_conid: Option<u32>,
}

impl Security {
    pub const COLLECTION_NAME: &'static str = "securities";

    pub fn new(
        security_type: SecurityType,
        ticker: &str,
        listing_exchange: &str,
        ibkr_conid: Option<u32>,
    ) -> Self {
        Self {
            _id: ObjectId::new(),
            listing_exchange: listing_exchange.to_owned(),
            security_type,
            ticker: ticker.to_owned(),
            ibkr_conid,
        }
    }

    pub fn id(&self) -> ObjectId {
        self._id
    }

    pub fn listing_exchange(&self) -> &str {
        &self.listing_exchange
    }

    pub fn security_type(&self) -> &SecurityType {
        &self.security_type
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn ibkr_conid(&self) -> Option<u32> {
        self.ibkr_conid
    }

    pub async fn insert(
        &self,
        db: &Database,
        session: Option<Arc<Mutex<ClientSession>>>,
    ) -> Result<()> {
        db_util::insert(self, db, Self::COLLECTION_NAME, session).await
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"_id": id})
            .await?;

        Ok(result)
    }

    pub async fn find_by_ticker_and_exchange(
        db: &Database,
        ticker: &str,
        listing_exchange: &str,
    ) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"ticker": ticker, "listing_exchange": listing_exchange})
            .await?;

        Ok(result)
    }

    pub async fn find_by_conid(db: &Database, ibkr_conid: u32) -> Result<Option<Self>> {
        Ok(db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(doc! { "ibkr_conid": ibkr_conid })
            .await?)
    }

    pub async fn find_by_ticker(db: &Database, ticker: &str) -> Result<Vec<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find(doc! { "ticker": ticker })
            .await?;

        // Convert the cursor to a vector of Security objects.
        Ok(result.try_collect().await?)
    }
}
