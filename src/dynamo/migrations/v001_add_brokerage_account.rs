use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::{Client, types::KeySchemaElement};

use crate::dynamo::account::DynamoBrokerageAccount;

use super::Migration;

pub struct MigrationV001 {}

#[async_trait]
impl Migration for MigrationV001 {
    fn id(&self) -> &'static str {
        "v001_add_brokerage_account"
    }

    async fn up(&self, client: &Client) -> Result<()> {
        let key_schema_1 = KeySchemaElement::builder()
            .attribute_name(DynamoBrokerageAccount::ACCOUNT_ID_COL_NAME.to_owned())
            .attribute_type(KeyType::Hash)
            .build()?;

        let key_schema_2 = KeySchemaElement::builder()
            .attribute_name(DynamoBrokerageAccount::BROKERAGE_ID_COL_NAME.to_owned())
            .attribute_type(KeyType::Hash)
            .build()?;

        let create_table_result = client
            .create_table()
            .table_name(DynamoBrokerageAccount::TABLE_NAME);

        Ok(())
    }

    async fn down(&self, _client: &Client) -> Result<()> {
        todo!()
    }
}
