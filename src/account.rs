use anyhow::Result;
use bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BrokerageAccount {
    pub _id: ObjectId,
    pub brokerage_id: String,
    pub account_id: String,
}

impl BrokerageAccount {
    pub const COLLECTION_NAME: &'static str = "brokerage_accounts";

    pub async fn insert(&self, db: &Database) -> Result<()> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .insert_one(self)
            .await?;
        tracing::info!(
            "inserted brokerage-account {:?}: (reported insert id: {:?})",
            self,
            result.inserted_id
        );
        Ok(())
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
