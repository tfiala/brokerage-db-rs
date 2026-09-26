use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    Client,
    types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
};
use tracing::debug;

use super::Migration;
use crate::dynamo::security::DynamoSecurity;

pub struct MigrationV002 {}

#[async_trait]
impl Migration for MigrationV002 {
    fn id(&self) -> &'static str {
        "v002_add_security"
    }

    async fn up(&self, client: &Client) -> Result<()> {
        debug!("dynamodb: creating table: {}", DynamoSecurity::TABLE_NAME);

        let pk_attr = AttributeDefinition::builder()
            .attribute_name(DynamoSecurity::PK_COL_NAME)
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let key_schema = KeySchemaElement::builder()
            .attribute_name(DynamoSecurity::PK_COL_NAME.to_owned())
            .key_type(KeyType::Hash)
            .build()?;

        let create_table_result = client
            .create_table()
            .table_name(DynamoSecurity::TABLE_NAME)
            .key_schema(key_schema)
            .set_attribute_definitions(Some(vec![pk_attr]))
            .set_billing_mode(Some(BillingMode::PayPerRequest))
            .send()
            .await?;

        tracing::info!(
            "dynamodb: created security table, result: {:?}",
            create_table_result
        );

        Ok(())
    }

    async fn down(&self, _client: &Client) -> Result<()> {
        todo!()
    }
}
