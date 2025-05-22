use anyhow::Result;
use brokerage_db::{
    db_connection::DbConnection, db_connection_factory::DbConnectionFactory, dynamo,
};
use rstest::{fixture, rstest};
use testcontainers::core::IntoContainerPort;
use testcontainers_modules::{
    dynamodb_local::DynamoDb,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct TestDbConnection {
    pub node: ContainerAsync<DynamoDb>,
    pub db_conn: Box<dyn DbConnection>,
}

impl TestDbConnection {
    pub async fn new() -> Result<Self> {
        let node = DynamoDb::default().start().await?;

        let host = node.get_host().await?;
        let host_port = node.get_host_port_ipv4(8000.tcp()).await?;
        let endpoint_url = format!("http://{host}:{host_port}");

        let access_key_id = "fakeKey";
        let access_key_secret = "fakeSecret";
        let factory = dynamo::DynamoDbConnectionFactory::new(
            access_key_id,
            access_key_secret,
            Some(endpoint_url),
        );

        let db_conn = factory.create().await?;

        Ok(Self { db_conn, node })
    }
}

#[fixture]
async fn empty_test_db_conn() -> Result<TestDbConnection> {
    TestDbConnection::new().await
}

#[fixture]
async fn test_db_conn() -> Result<TestDbConnection> {
    // Create the dynamodb connection.
    let test_db_conn = TestDbConnection::new().await?;

    // Run the migrations.
    test_db_conn.db_conn.run_migrations().await?;

    Ok(test_db_conn)
}

#[tokio::test]
async fn test_succeeds() -> Result<()> {
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn empty_dynamodb_connection_succeeds(
    #[future] empty_test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let _test_db_conn = empty_test_db_conn.unwrap();
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn migrations_succeed(#[future] test_db_conn: Result<TestDbConnection>) -> Result<()> {
    let _test_db_conn = test_db_conn.unwrap();
    Ok(())
}
