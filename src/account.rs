use anyhow::Result;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

use crate::db_util;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BrokerageAccount {
    _id: ObjectId,
    brokerage_id: String,
    account_id: String,
}

impl BrokerageAccount {
    pub const COLLECTION_NAME: &'static str = "brokerage_accounts";

    pub fn new(brokerage_id: &str, account_id: &str) -> Self {
        Self {
            _id: ObjectId::new(),
            brokerage_id: brokerage_id.to_owned(),
            account_id: account_id.to_owned(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self._id
    }

    pub fn brokerage_id(&self) -> &str {
        &self.brokerage_id
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub async fn insert(
        &self,
        db: &Database,
        session: Option<Arc<Mutex<ClientSession>>>,
    ) -> Result<()> {
        // Option<&mut ClientSession>
        db_util::insert(self, db, Self::COLLECTION_NAME, session).await
    }

    pub async fn find(db: &Database) -> Result<Vec<Self>> {
        Ok(db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find(bson::doc! {})
            .await?
            .try_collect()
            .await?)
    }

    pub async fn find_by_brokerage_and_account_id(
        db: &Database,
        brokerage_id: &str,
        account_id: &str,
    ) -> Result<Option<Self>> {
        let result = db
            .collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME)
            .find_one(bson::doc! {
            "brokerage_id": brokerage_id,
            "account_id": account_id})
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"_id": id})
            .await?;

        Ok(result)
    }
}
