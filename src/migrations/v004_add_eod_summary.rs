use crate::eod_summary::EODSummary;
use anyhow::Result;
use async_trait::async_trait;
use bson::doc;
use mongodb::{IndexModel, options::IndexOptions};
use tfiala_mongodb_migrator::migrator::Env;

pub struct Migration004 {}

const EOD_SUMMARIES_UNIQUE_INDEX_NAME: &str = "eod_summaries_unique_idx";

#[async_trait]
impl tfiala_mongodb_migrator::migration::Migration for Migration004 {
    async fn up(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();

        //
        // Create initial EOD summary collection
        //
        db.create_collection(EODSummary::COLLECTION_NAME).await?;

        let collection = db.collection::<EODSummary>(EODSummary::COLLECTION_NAME);
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "brokerage_account_id": 1, "end_timestamp_ms": 1 })
                .options(
                    IndexOptions::builder()
                        .name(Some(EOD_SUMMARIES_UNIQUE_INDEX_NAME.to_owned()))
                        .unique(true)
                        .build(),
                )
                .build(),
        ];

        collection.create_indexes(indexes).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();
        let collection = db.collection::<EODSummary>(EODSummary::COLLECTION_NAME);

        collection
            .drop_index(EOD_SUMMARIES_UNIQUE_INDEX_NAME)
            .await?;

        collection.drop().await?;

        Ok(())
    }
}
