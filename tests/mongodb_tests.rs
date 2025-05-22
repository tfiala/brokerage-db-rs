use anyhow::Result;
use brokerage_db::{
    account::{BrokerageAccount, IBrokerageAccount},
    db_connection::DbConnection,
    db_connection_factory::DbConnectionFactory,
    initialize, mongo, remove_data,
    security::{Security, SecurityType},
    trade_execution::{self, TradeExecution, TradeSide},
};
use bson::oid::ObjectId;
use mongodb::{
    Client, Database,
    error::{Error, ErrorKind, WriteFailure},
};
use rstest::{fixture, rstest};
use testcontainers_modules::{
    mongo::Mongo,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
// use tracing_test::traced_test;

struct TestDbConnection {
    db_conn: Box<dyn DbConnection<ObjectId>>,
    _node: ContainerAsync<Mongo>,
}

impl TestDbConnection {
    pub async fn new(db_name: &str) -> Result<Self> {
        let node = Mongo::default().start().await?;
        let host_port = node.get_host_port_ipv4(27017).await?;

        let uri = format!("mongodb://localhost:{}/", host_port);
        let factory = mongo::MdbDbConnectionFactory::new(&uri, db_name);
        let db_conn = factory.create().await?;

        Ok(Self {
            db_conn,
            _node: node,
        })
    }
}

#[fixture]
async fn empty_test_db_conn() -> Result<TestDbConnection> {
    TestDbConnection::new("test").await
}

#[fixture]
async fn test_db_conn() -> Result<TestDbConnection> {
    let test_db_conn = empty_test_db_conn().await?;
    test_db_conn.db_conn.run_migrations().await?;

    Ok(test_db_conn)
}

pub struct DbConnectionOld {
    pub client: Client,
    pub db: Database,
    pub node: ContainerAsync<Mongo>,
}

impl DbConnectionOld {
    pub async fn new(db_name: &str) -> Result<Self> {
        let node = Mongo::default().start().await?;
        let host_port = node.get_host_port_ipv4(27017).await?;

        let url = format!("mongodb://localhost:{}/", host_port);
        let client = mongodb::Client::with_uri_str(url).await?;
        let db = client.database(db_name);

        Ok(DbConnectionOld { client, db, node })
    }
}

struct TradeExecutionDesc {
    pub security: Security,
    pub brokerage_account: BrokerageAccount,
    pub trade_execution: TradeExecution,
}

#[fixture]
async fn empty_test_db_conn_old() -> Result<DbConnectionOld> {
    DbConnectionOld::new("test").await
}

#[fixture]
async fn test_db_conn_old() -> Result<DbConnectionOld> {
    let db_conn = DbConnectionOld::new("test").await?;
    initialize(&db_conn.db).await?;
    Ok(db_conn)
}

#[fixture]
async fn admin_db_conn() -> Result<DbConnectionOld> {
    DbConnectionOld::new("admin").await
}

const BROKERAGE_ID: &str = "batch-brokers";
const BROKERAGE_ACCOUNT_ID: &str = "A1234567";

const BROKERAGE_ID_2: &str = "another-broker";
const BROKERAGE_ACCOUNT_ID_2: &str = "DA7654321";

fn brokerage_account(
    db_conn: Box<dyn DbConnection<ObjectId>>,
) -> Box<dyn IBrokerageAccount<ObjectId> + Send> {
    db_conn.new_brokerage_account(BROKERAGE_ACCOUNT_ID, BROKERAGE_ID)
}

fn brokerage_account_2(
    db_conn: Box<dyn DbConnection<ObjectId>>,
) -> Box<dyn IBrokerageAccount<ObjectId> + Send> {
    db_conn.new_brokerage_account(BROKERAGE_ACCOUNT_ID_2, BROKERAGE_ID_2)
}

#[fixture]
fn brokerage_account_old() -> BrokerageAccount {
    BrokerageAccount::new(BROKERAGE_ID, BROKERAGE_ACCOUNT_ID)
}

#[fixture]
fn brokerage_account_2_old() -> BrokerageAccount {
    BrokerageAccount::new(BROKERAGE_ID_2, BROKERAGE_ACCOUNT_ID_2)
}

#[fixture]
fn security() -> Security {
    Security::new(SecurityType::Stock, "AAPL", "NASDAQ", None)
}

#[fixture]
fn security_with_conid() -> Security {
    Security::new(SecurityType::Stock, "AAPL", "NASDAQ", Some(12345678))
}

#[fixture]
fn trade_execution_desc() -> TradeExecutionDesc {
    let security = Security::new(SecurityType::Stock, "AAPL", "NASDAQ", None);
    let brokerage_account = BrokerageAccount::new("batch-brokers", "A1234567");
    let trade_execution = TradeExecution::builder()
        .brokerage_account_id(brokerage_account.id())
        .brokerage_execution_id("abc-123-def")
        .commission(0.0)
        .execution_timestamp_ms(1746665451000)
        .quantity(100.0)
        .price(150.0)
        .security_id(security.id())
        .side(TradeSide::Buy)
        .build()
        .expect("Failed to build TradeExecution");

    TradeExecutionDesc {
        security,
        brokerage_account,
        trade_execution,
    }
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_mongodb_container_connection(
    #[future] admin_db_conn: Result<DbConnectionOld>,
) -> Result<()> {
    // Ping the server to check if the connection is successful
    let result = admin_db_conn
        .unwrap()
        .db
        .run_command(bson::doc! { "ping": 1 })
        .await;
    assert!(result.is_ok());
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_migration_succeeds(
    #[future] empty_test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    empty_test_db_conn.unwrap().db_conn.run_migrations().await
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_migration_succeeds_old(
    #[future] empty_test_db_conn_old: Result<DbConnectionOld>,
) -> Result<()> {
    initialize(&empty_test_db_conn_old.unwrap().db).await?;
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_down_migration_succeeds_old(
    #[future] empty_test_db_conn_old: Result<DbConnectionOld>,
) -> Result<()> {
    let dbc = empty_test_db_conn_old?;

    initialize(&dbc.db).await?;
    remove_data(&dbc.db).await?;
    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_down_migration_succeeds(
    #[future] test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let test_db_conn = test_db_conn.unwrap();
    test_db_conn.db_conn.remove_migrations().await
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_brokerage_account_works(
    #[future] test_db_conn: Result<TestDbConnection>,
) -> Result<()> {
    let test_db_conn = test_db_conn?;
    let brokerage_account = brokerage_account(test_db_conn.db_conn);
    test_db_conn.db_conn.insert_bacct(brokerage_account).await?;

    let found_account = test_db_conn
        .db_conn
        .find_bacct_by_brokerage_and_account_id(BROKERAGE_ID, BROKERAGE_ACCOUNT_ID)
        .await?;

    assert!(found_account.is_some());
    assert_eq!(brokerage_account, found_account.unwrap());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_brokerage_account_works_old(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    brokerage_account_old: BrokerageAccount,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    brokerage_account_old.insert(&dbc.db, None).await?;

    let found_account = dbc
        .db
        .collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME)
        .find_one(bson::doc! {
        "brokerage_id": brokerage_account_old.brokerage_id(),
        "account_id": brokerage_account_old.account_id() })
        .await?
        .ok_or_else(|| anyhow::anyhow!("Brokerage account not found"))?;

    assert_eq!(brokerage_account_old, found_account);

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_all_brokerage_accounts_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    brokerage_account_old: BrokerageAccount,
    brokerage_account_2_old: BrokerageAccount,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    brokerage_account_old.insert(&dbc.db, None).await?;
    brokerage_account_2_old.insert(&dbc.db, None).await?;

    let found_accounts = BrokerageAccount::find(&dbc.db).await?;

    assert!(found_accounts.contains(&brokerage_account_old));
    assert!(found_accounts.contains(&brokerage_account_2_old));

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_duplicate_brokerage_account_fails(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    brokerage_account: BrokerageAccount,
) -> Result<()> {
    let dbc = test_db_conn_old?;

    // Insert it once.
    brokerage_account.insert(&dbc.db, None).await?;
    // And again.
    let result = brokerage_account.insert(&dbc.db, None).await;

    assert!(result.is_err());

    let expected_error = result.unwrap_err().downcast::<Error>();
    assert!(expected_error.is_ok());
    let kind = expected_error.unwrap().kind;
    match *kind {
        ErrorKind::Write(WriteFailure::WriteError(write_error)) => {
            assert_eq!(write_error.code, 11000);
        }
        _ => panic!("Expected a WriteError with code 11000"),
    }

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_security_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security.insert(&dbc.db, None).await?;

    let found_security = Security::find_by_ticker_and_exchange(
        &dbc.db,
        security.ticker(),
        security.listing_exchange(),
    )
    .await?;

    assert!(found_security.is_some());
    assert_eq!(security, found_security.unwrap());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_security_with_conid_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security_with_conid: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security_with_conid.insert(&dbc.db, None).await?;

    let found_security = dbc
        .db
        .collection::<Security>(Security::COLLECTION_NAME)
        .find_one(bson::doc! {
        "ticker": security_with_conid.ticker(),
        "listing_exchange": security_with_conid.listing_exchange() })
        .await?
        .ok_or_else(|| anyhow::anyhow!("Security not found"))?;

    assert_eq!(security_with_conid, found_security);

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_non_extant_security_fails(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    let result = Security::find_by_ticker_and_exchange(
        &dbc.db,
        security.ticker(),
        security.listing_exchange(),
    )
    .await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_security_with_ticker_and_exchange_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security.insert(&dbc.db, None).await?;

    let result = Security::find_by_ticker_and_exchange(
        &dbc.db,
        security.ticker(),
        security.listing_exchange(),
    )
    .await;
    assert!(result.is_ok());

    let found_security = result.unwrap();
    assert!(found_security.is_some());
    assert_eq!(security, found_security.unwrap());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_security_with_ticker_and_one_match_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security.insert(&dbc.db, None).await?;

    let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
    assert!(result.is_ok());

    let found_securities = result.unwrap();
    assert_eq!(found_securities.len(), 1);
    assert_eq!(security, found_securities[0]);

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_non_extant_security_with_ticker_returns_zero_elements(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;

    let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
    assert!(result.is_ok());

    let found_securities = result.unwrap();
    assert_eq!(found_securities.len(), 0);

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_security_with_ticker_and_two_match_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security.insert(&dbc.db, None).await?;

    let security2 = Security::new(SecurityType::Stock, security.ticker(), "NYSE", None);
    security2.insert(&dbc.db, None).await?;

    let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
    assert!(result.is_ok());

    let found_securities = result.unwrap();
    assert_eq!(found_securities.len(), 2);
    assert!(found_securities.contains(&security));
    assert!(found_securities.contains(&security2));

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_security_by_conid_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security_with_conid: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;
    security_with_conid.insert(&dbc.db, None).await?;

    let result = Security::find_by_conid(&dbc.db, security_with_conid.ibkr_conid().unwrap()).await;
    assert!(result.is_ok());

    let found_security = result.unwrap();
    assert!(found_security.is_some());
    assert_eq!(security_with_conid, found_security.unwrap());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn find_non_extant_security_by_conid_fails(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    security_with_conid: Security,
) -> Result<()> {
    let dbc = test_db_conn_old?;

    let result = Security::find_by_conid(&dbc.db, security_with_conid.ibkr_conid().unwrap()).await;
    assert!(result.is_ok());

    let found_security = result.unwrap();
    assert!(found_security.is_none());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_trade_execution_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    trade_execution_desc: TradeExecutionDesc,
) -> Result<()> {
    let dbc = test_db_conn_old?;

    // Insert the brokerage account and security first.
    trade_execution_desc
        .brokerage_account
        .insert(&dbc.db, None)
        .await?;
    trade_execution_desc.security.insert(&dbc.db, None).await?;

    // Now insert the trade execution.
    trade_execution_desc
        .trade_execution
        .insert(&dbc.db, None)
        .await?;

    let found_trade_execution =
        TradeExecution::find_by_id(&dbc.db, trade_execution_desc.trade_execution.id()).await?;
    assert!(found_trade_execution.is_some());
    assert_eq!(
        trade_execution_desc.trade_execution,
        found_trade_execution.unwrap()
    );

    // Check that the trade execution is linked to the correct brokerage account and security.
    let found_brokerage_account = BrokerageAccount::find_by_id(
        &dbc.db,
        trade_execution_desc.trade_execution.brokerage_account_id(),
    )
    .await?;
    assert!(found_brokerage_account.is_some());
    assert_eq!(
        trade_execution_desc.brokerage_account,
        found_brokerage_account.unwrap()
    );
    let found_security =
        Security::find_by_id(&dbc.db, trade_execution_desc.trade_execution.security_id()).await?;
    assert!(found_security.is_some());
    assert_eq!(trade_execution_desc.security, found_security.unwrap());

    Ok(())
}

#[rstest]
#[awt]
#[tokio::test]
async fn insert_two_trade_executions_works(
    #[future] test_db_conn_old: Result<DbConnectionOld>,
    trade_execution_desc: TradeExecutionDesc,
) -> Result<()> {
    let dbc = test_db_conn_old?;

    // Insert the brokerage account and security first.
    trade_execution_desc
        .brokerage_account
        .insert(&dbc.db, None)
        .await?;
    trade_execution_desc.security.insert(&dbc.db, None).await?;

    // Now insert the trade execution.
    trade_execution_desc
        .trade_execution
        .insert(&dbc.db, None)
        .await?;

    // Insert a similar trade execution.
    let trade_execution_2 =
        trade_execution::Builder::from_trade_execution(&trade_execution_desc.trade_execution)
            .brokerage_execution_id("abc-123-def-2")
            .execution_timestamp_ms(1746665452000)
            .build()?;

    trade_execution_2.insert(&dbc.db, None).await?;

    let found_trade_execution =
        TradeExecution::find_by_id(&dbc.db, trade_execution_desc.trade_execution.id()).await?;
    assert!(found_trade_execution.is_some());
    assert_eq!(
        trade_execution_desc.trade_execution,
        found_trade_execution.unwrap()
    );

    let found_trade_execution_2 =
        TradeExecution::find_by_id(&dbc.db, trade_execution_2.id()).await?;
    assert!(found_trade_execution_2.is_some());
    assert_eq!(trade_execution_2, found_trade_execution_2.unwrap());

    Ok(())
}
