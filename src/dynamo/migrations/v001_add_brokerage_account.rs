use crate::dynamo::account::DynamoBrokerageAccount;
use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    Client,
    types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
};
use tracing::debug;

use super::Migration;

pub struct MigrationV001 {}

#[async_trait]
impl Migration for MigrationV001 {
    fn id(&self) -> &'static str {
        "v001_add_brokerage_account"
    }

    async fn up(&self, client: &Client) -> Result<()> {
        debug!(
            "dynamodb: creating table: {}",
            DynamoBrokerageAccount::TABLE_NAME
        );

        let pk_attr = AttributeDefinition::builder()
            .attribute_name(DynamoBrokerageAccount::PK_COL_NAME)
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let key_schema = KeySchemaElement::builder()
            .attribute_name(DynamoBrokerageAccount::PK_COL_NAME.to_owned())
            .key_type(KeyType::Hash)
            .build()?;

        let create_table_result = client
            .create_table()
            .table_name(DynamoBrokerageAccount::TABLE_NAME)
            .key_schema(key_schema)
            .set_attribute_definitions(Some(vec![pk_attr]))
            .set_billing_mode(Some(BillingMode::PayPerRequest))
            .send()
            .await?;

        tracing::info!("dynamodb: created table, result: {:?}", create_table_result);

        Ok(())
    }

    async fn down(&self, _client: &Client) -> Result<()> {
        todo!()
    }
}
