use crate::{security::Security, trade_execution::TradeExecution};
use anyhow::Result;
use async_trait::async_trait;
use bson::doc;
use mongodb::{IndexModel, options::IndexOptions};
use tfiala_mongodb_migrator::migrator::Env;

pub struct Migration003 {}

const TRADE_EXECUTIONS_UNIQUE_INDEX_NAME: &str = "trade_executions_unique_idx";
const TRADE_EXECUTIONS_BY_ACCOUNT_SECURITY_TIMESTAMP_INDEX_NAME: &str =
    "trade_executions_by_account_security_timestamp_idx";

#[async_trait]
impl tfiala_mongodb_migrator::migration::Migration for Migration003 {
    async fn up(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();

        //
        // Create initial trade execution collection
        //
        db.create_collection(TradeExecution::COLLECTION_NAME)
            .await?;

        let collection = db.collection::<TradeExecution>(TradeExecution::COLLECTION_NAME);
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "brokerage_account_id": 1, "brokerage_execution_id": 1 })
                .options(
                    IndexOptions::builder()
                        .name(Some(TRADE_EXECUTIONS_UNIQUE_INDEX_NAME.to_owned()))
                        .unique(true)
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(
                    doc! { "brokerage_account_id": 1, "security_id": 1, "execution_timestamp": 1 },
                )
                .options(
                    IndexOptions::builder()
                        .name(Some(
                            TRADE_EXECUTIONS_BY_ACCOUNT_SECURITY_TIMESTAMP_INDEX_NAME.to_owned(),
                        ))
                        .build(),
                )
                .build(),
        ];

        collection.create_indexes(indexes).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();
        let collection = db.collection::<Security>(TradeExecution::COLLECTION_NAME);

        collection
            .drop_index(TRADE_EXECUTIONS_BY_ACCOUNT_SECURITY_TIMESTAMP_INDEX_NAME)
            .await?;

        collection
            .drop_index(TRADE_EXECUTIONS_UNIQUE_INDEX_NAME)
            .await?;

        collection.drop().await?;

        Ok(())
    }
}
