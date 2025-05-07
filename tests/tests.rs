use anyhow::Result;
use brokerage_db::{account::BrokerageAccount, insert_brokerage_account, run_migrations};
use mongodb::{Client, Database};
use rstest::{fixture, rstest};
use testcontainers_modules::{
    mongo::Mongo,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tracing_test::traced_test;

pub struct DbConnection {
    pub client: Client,
    pub db: Database,
    pub node: ContainerAsync<Mongo>,
}

impl DbConnection {
    pub async fn new(db_name: &str) -> Result<Self> {
        let node = Mongo::default().start().await?;
        let host_port = node.get_host_port_ipv4(27017).await?;

        let url = format!("mongodb://localhost:{}/", host_port);
        let client = mongodb::Client::with_uri_str(url).await?;
        let db = client.database(db_name);

        Ok(DbConnection { client, db, node })
    }
}

#[fixture]
async fn empty_test_db_conn() -> Result<DbConnection> {
    DbConnection::new("test").await
}

#[fixture]
async fn test_db_conn() -> Result<DbConnection> {
    let db_conn = DbConnection::new("test").await?;
    run_migrations(db_conn.db.clone()).await?;
    Ok(db_conn)
}

#[fixture]
async fn admin_db_conn() -> Result<DbConnection> {
    DbConnection::new("admin").await
}

#[fixture]
fn brokerage_account() -> BrokerageAccount {
    BrokerageAccount {
        _id: bson::oid::ObjectId::new(),
        brokerage_id: "batch-brokers".to_string(),
        account_id: "A1234567".to_string(),
    }
}

#[rstest]
#[awt]
#[tokio::test]
async fn test_mongodb_container_connection(
    #[future] admin_db_conn: Result<DbConnection>,
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
#[traced_test]
#[tokio::test]
async fn test_migration_succeeds(#[future] empty_test_db_conn: Result<DbConnection>) -> Result<()> {
    run_migrations(empty_test_db_conn.unwrap().db.clone()).await?;
    Ok(())
}

#[rstest]
#[awt]
#[traced_test]
#[tokio::test]
async fn insert_one_brokerage_account_works(
    #[future] test_db_conn: Result<DbConnection>,
    brokerage_account: BrokerageAccount,
) -> Result<()> {
    let dbc = test_db_conn?;
    insert_brokerage_account(&dbc.db, &brokerage_account).await?;

    let found_account = dbc
        .db
        .collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME)
        .find_one(bson::doc! {
        "brokerage_id": brokerage_account.brokerage_id.clone(),
        "account_id": brokerage_account.account_id.clone() })
        .await?
        .ok_or_else(|| anyhow::anyhow!("Brokerage account not found"))?;

    // assert_eq!(found_account.brokerage_id, brokerage_account.brokerage_id);
    // assert_eq!(found_account.account_id, brokerage_account.account_id);
    // assert_eq!(found_account._id, brokerage_account._id);
    assert_eq!(brokerage_account, found_account);

    Ok(())
}
