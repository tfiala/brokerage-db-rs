use crate::security::Security;
use anyhow::Result;
use async_trait::async_trait;
use bson::doc;
use mongodb::{IndexModel, options::IndexOptions};
use tfiala_mongodb_migrator::migrator::Env;

pub struct Migration002 {}

const SECURITIES_UNIQUE_INDEX_NAME: &str = "securities_unique_idx";
const SECURITIES_IBKR_CONID_INDEX_NAME: &str = "securities_conid_idx";

#[async_trait]
impl tfiala_mongodb_migrator::migration::Migration for Migration002 {
    async fn up(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();

        //
        // Create initial security (stock, bond, option, etc.) collection
        //
        db.create_collection(Security::COLLECTION_NAME).await?;

        let collection = db.collection::<Security>(Security::COLLECTION_NAME);
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "ticker": 1, "listing_exchange": 1 })
                .options(
                    IndexOptions::builder()
                        .name(Some(SECURITIES_UNIQUE_INDEX_NAME.to_owned()))
                        .unique(true)
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "ibkr_conid": 1 })
                .options(
                    IndexOptions::builder()
                        .name(Some(SECURITIES_IBKR_CONID_INDEX_NAME.to_owned()))
                        .build(),
                )
                .build(),
        ];

        collection.create_indexes(indexes).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();
        let collection = db.collection::<Security>(Security::COLLECTION_NAME);

        collection
            .drop_index(SECURITIES_IBKR_CONID_INDEX_NAME)
            .await?;

        collection.drop_index(SECURITIES_UNIQUE_INDEX_NAME).await?;

        collection.drop().await?;

        Ok(())
    }
}
