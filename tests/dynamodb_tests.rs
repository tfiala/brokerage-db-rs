use anyhow::Result;
use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_dynamodb::{
    Client,
    config::Credentials,
    // types::{
    //     AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    // },
};
use std::fmt::Display;
use testcontainers::core::IntoContainerPort;
use testcontainers_modules::{
    dynamodb_local::DynamoDb,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct DbConnection {
    pub node: ContainerAsync<DynamoDb>,
    pub client: Client,
}

impl DbConnection {
    pub async fn new() -> Result<Self> {
        let node = DynamoDb::default().start().await?;
        let host = node.get_host().await?;
        let host_port = node.get_host_port_ipv4(8000.tcp()).await?;

        let client = Self::build_dynamodb_client(host, host_port).await;

        Ok(Self { client, node })
    }

    async fn build_dynamodb_client(host: impl Display, host_port: u16) -> Client {
        let endpoint_uri = format!("http://{host}:{host_port}");
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let creds = Credentials::new("fakeKey", "fakeSecret", None, None, "test");

        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .endpoint_url(endpoint_uri)
            .credentials_provider(creds)
            .load()
            .await;

        Client::new(&shared_config)
    }
}

#[tokio::test]
async fn test_succeeds() -> Result<()> {
    Ok(())
}

#[tokio::test]
async fn empty_dynamodb_has_no_tables() -> Result<()> {
    let connection = DbConnection::new().await?;

    let req = connection.client.list_tables();
    let list_tables_result = req.send().await.unwrap();

    assert_eq!(list_tables_result.table_names().len(), 0);

    Ok(())
}
