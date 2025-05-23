use anyhow::Result;
use brokerage_db::{
    account::IBrokerageAccount, db_connection::DbConnection,
    db_connection_factory::DbConnectionFactory, dynamo,
};
use rstest::{fixture, rstest};
use testcontainers::core::IntoContainerPort;
use testcontainers_modules::{
    dynamodb_local::DynamoDb,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tracing_test::traced_test;

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

const BROKERAGE_ID: &str = "batch-brokers";
const BROKERAGE_ACCOUNT_ID: &str = "A1234567";

const BROKERAGE_ID_2: &str = "another-broker";
const BROKERAGE_ACCOUNT_ID_2: &str = "DA7654321";

fn brokerage_account(db_conn: &dyn DbConnection) -> Box<dyn IBrokerageAccount> {
    db_conn.new_brokerage_account(BROKERAGE_ACCOUNT_ID, BROKERAGE_ID)
}

fn brokerage_account_2(db_conn: &dyn DbConnection) -> Box<dyn IBrokerageAccount> {
    db_conn.new_brokerage_account(BROKERAGE_ACCOUNT_ID_2, BROKERAGE_ID_2)
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
#[traced_test]
async fn migrations_succeed(#[future] test_db_conn: Result<TestDbConnection>) -> Result<()> {
    let _test_db_conn = test_db_conn.unwrap();
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
#[traced_test]
async fn insert_brokerage_account_works(
    #[future] test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let test_db_conn = test_db_conn?;
    let brokerage_account = brokerage_account(test_db_conn.db_conn.as_ref());
    test_db_conn
        .db_conn
        .insert_bacct(brokerage_account.as_ref())
        .await?;

    let found_account = test_db_conn
        .db_conn
        .find_bacct_by_brokerage_and_account_id(BROKERAGE_ID, BROKERAGE_ACCOUNT_ID)
        .await?;
    assert!(found_account.is_some());
    let found_account = found_account.unwrap();

    assert_eq!(brokerage_account.account_id(), found_account.account_id());
    assert_eq!(
        brokerage_account.brokerage_id(),
        found_account.brokerage_id()
    );

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_duplicate_brokerage_account_fails(
    #[future] test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let test_db_conn = test_db_conn?;
    let brokerage_account = brokerage_account(test_db_conn.db_conn.as_ref());

    // Insert it once.
    test_db_conn
        .db_conn
        .insert_bacct(brokerage_account.as_ref())
        .await?;

    // Insert it again.
    let result = test_db_conn
        .db_conn
        .insert_bacct(brokerage_account.as_ref())
        .await;

    assert!(result.is_err());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_all_brokerage_accounts_works(
    #[future] test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let test_db_conn = test_db_conn.unwrap();

    let brokerage_account = brokerage_account(test_db_conn.db_conn.as_ref());
    test_db_conn
        .db_conn
        .insert_bacct(brokerage_account.as_ref())
        .await?;

    let brokerage_account_2 = brokerage_account_2(test_db_conn.db_conn.as_ref());
    test_db_conn
        .db_conn
        .insert_bacct(brokerage_account_2.as_ref())
        .await?;

    let found_accounts = test_db_conn.db_conn.find_bacct_all().await?;

    assert_eq!(found_accounts.len(), 2);

    assert!(
        brokerage_account.account_id() == found_accounts[0].account_id()
            || brokerage_account.account_id() == found_accounts[1].account_id()
    );
    assert!(
        brokerage_account.brokerage_id() == found_accounts[0].brokerage_id()
            || brokerage_account.brokerage_id() == found_accounts[1].brokerage_id()
    );

    assert!(
        brokerage_account_2.account_id() == found_accounts[0].account_id()
            || brokerage_account_2.account_id() == found_accounts[1].account_id()
    );
    assert!(
        brokerage_account_2.brokerage_id() == found_accounts[0].brokerage_id()
            || brokerage_account_2.brokerage_id() == found_accounts[1].brokerage_id()
    );
    Ok(())
}
