use anyhow::Result;
use brokerage_db::run_migrations;
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
async fn test_db_conn() -> Result<DbConnection> {
    DbConnection::new("test").await
}

#[fixture]
async fn admin_db_conn() -> Result<DbConnection> {
    DbConnection::new("admin").await
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
async fn test_migration_succeeds(#[future] test_db_conn: Result<DbConnection>) -> Result<()> {
    run_migrations(test_db_conn.unwrap().db.clone()).await?;
    Ok(())
}
