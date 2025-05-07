use crate::account::BrokerageAccount;
use anyhow::Result;
use async_trait::async_trait;
use bson::doc;
use mongodb::{IndexModel, options::IndexOptions};
use tfiala_mongodb_migrator::migrator::Env;

pub struct Migration {}

#[async_trait]
impl tfiala_mongodb_migrator::migration::Migration for Migration {
    async fn up(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();

        //
        // Create initial brokerage-accounts indexes.
        //
        let account_collection =
            db.collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME);
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "brokerage_id": 1, "account_id": 1 })
                .options(
                    IndexOptions::builder()
                        .name(Some("brokerage-account-unique-idx".to_owned()))
                        .unique(true)
                        .build(),
                )
                .build(),
        ];

        let _result = account_collection.create_indexes(indexes).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> Result<()> {
        let db = env.db.unwrap();
        let account_collection =
            db.collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME);

        let _result = account_collection
            .drop_index("brokerage-account-unique-idx")
            .await?;

        Ok(())
    }
}
