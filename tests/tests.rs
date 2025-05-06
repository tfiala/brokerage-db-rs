use anyhow::Result;
use brokerage_db::run_migrations;
use mongodb::{Client, Database};
use testcontainers_modules::{
    mongo::Mongo,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct TestDb {
    pub client: Client,
    pub db: Database,
    pub node: ContainerAsync<Mongo>,
}

impl TestDb {
    pub async fn new_with_db(db_name: &str) -> Result<Self> {
        let node = Mongo::default().start().await?;
        let host_port = node.get_host_port_ipv4(27017).await?;
        let url = format!("mongodb://localhost:{}/", host_port);

        let client = mongodb::Client::with_uri_str(url).await?;
        let db = client.database(db_name);

        Ok(Self { client, db, node })
    }

    pub async fn new() -> Result<Self> {
        Self::new_with_db("test").await
    }
}

#[tokio::test]
async fn test_mongodb_container_connection() -> Result<()> {
    let test = TestDb::new_with_db("admin").await?;

    // Ping the server to check if the connection is successful
    let result = test.db.run_command(bson::doc! { "ping": 1 }).await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_migration_up_and_down_succeeds() -> Result<()> {
    let test = TestDb::new_with_db("admin").await?;
    run_migrations(test.db.clone()).await?;
    Ok(())
}
