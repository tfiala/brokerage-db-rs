use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    Client, Error,
    types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
};

use crate::dynamo::account::DynamoBrokerageAccount;

use super::Migration;

pub struct MigrationV001 {}

#[async_trait]
impl Migration for MigrationV001 {
    fn id(&self) -> &'static str {
        "v001_add_brokerage_account"
    }

    async fn up(&self, client: &Client) -> Result<()> {
        let account_id_attr = AttributeDefinition::builder()
            .attribute_name(DynamoBrokerageAccount::ACCOUNT_ID_COL_NAME)
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let brokerage_id_attr = AttributeDefinition::builder()
            .attribute_name(DynamoBrokerageAccount::BROKERAGE_ID_COL_NAME)
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let key_schema_1 = KeySchemaElement::builder()
            .attribute_name(DynamoBrokerageAccount::ACCOUNT_ID_COL_NAME.to_owned())
            .key_type(KeyType::Hash)
            .build()?;

        let key_schema_2 = KeySchemaElement::builder()
            .attribute_name(DynamoBrokerageAccount::BROKERAGE_ID_COL_NAME.to_owned())
            .key_type(KeyType::Hash)
            .build()?;

        let create_table_result = client
            .create_table()
            .table_name(DynamoBrokerageAccount::TABLE_NAME)
            .set_key_schema(Some(vec![key_schema_1, key_schema_2]))
            .set_attribute_definitions(Some(vec![account_id_attr, brokerage_id_attr]))
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
