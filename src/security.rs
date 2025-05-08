use anyhow::Result;
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};

use crate::db_util;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SecurityType {
    Stock,
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

    pub fn get_id(&self) -> ObjectId {
        self._id
    }

    pub fn get_listing_exchange(&self) -> &str {
        &self.listing_exchange
    }

    pub fn get_security_type(&self) -> &SecurityType {
        &self.security_type
    }

    pub fn get_ticker(&self) -> &str {
        &self.ticker
    }

    pub fn get_ibkr_conid(&self) -> Option<u32> {
        self.ibkr_conid
    }

    pub async fn insert(&self, db: &Database, session: Option<&mut ClientSession>) -> Result<()> {
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
