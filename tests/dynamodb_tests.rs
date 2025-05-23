use anyhow::Result;
use brokerage_db::{db_connection_factory::DbConnectionFactory, dynamo};
use common::CommonTests;
use rstest::{fixture, rstest};
use testcontainers::core::IntoContainerPort;
use testcontainers_modules::{dynamodb_local::DynamoDb, testcontainers::runners::AsyncRunner};
use tracing_test::traced_test;

mod common;

#[fixture]
async fn common_tests() -> Result<CommonTests<DynamoDb>> {
    // Create the test container.
    let node = DynamoDb::default().start().await?;

    // Set up AWS endpoint.
    let host = node.get_host().await?;
    let host_port = node.get_host_port_ipv4(8000.tcp()).await?;
    let endpoint_url = format!("http://{host}:{host_port}");

    // Get the DynamoDB db connection factory.
    let access_key_id = "fakeKey";
    let access_key_secret = "fakeSecret";
    let factory = dynamo::DynamoDbConnectionFactory::new(
        access_key_id,
        access_key_secret,
        Some(endpoint_url),
    );

    // Create the db connection.
    let db_conn = factory.create().await?;

    // Run migrations.
    db_conn.run_migrations().await?;

    Ok(CommonTests::new(db_conn, node))
}

#[rstest]
#[awt]
#[tokio::test]
#[traced_test]
async fn migrations_succeed(#[future] common_tests: Result<CommonTests<DynamoDb>>) -> Result<()> {
    let _common_tests = common_tests.unwrap();
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
#[traced_test]
async fn insert_brokerage_account_works(
    #[future] common_tests: Result<CommonTests<DynamoDb>>,
) -> Result<()> {
    common_tests
        .unwrap()
        .insert_brokerage_account_works()
        .await?;

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_duplicate_brokerage_account_fails(
    #[future] common_tests: Result<CommonTests<DynamoDb>>,
) -> Result<()> {
    common_tests
        .unwrap()
        .insert_duplicate_brokerage_account_fails()
        .await?;

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_all_brokerage_accounts_works(
    #[future] common_tests: Result<CommonTests<DynamoDb>>,
) -> Result<()> {
    common_tests
        .unwrap()
        .find_all_brokerage_accounts_works()
        .await?;

    Ok(())
}
