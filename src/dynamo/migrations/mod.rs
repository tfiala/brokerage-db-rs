mod v001_add_brokerage_account;

use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use v001_add_brokerage_account::*;

#[async_trait]
pub trait Migration {
    fn id(&self) -> &'static str;
    async fn up(&self, client: &Client) -> Result<()>;
    async fn down(&self, client: &Client) -> Result<()>;
}

const MIGRATIONS: Vec<Box<dyn Migration>> = vec![Box::new(MigrationV001::new())];

pub async fn run_migrations(client: &Client) -> Result<()> {
    Ok(())
}

pub async fn remove_migrations(client: &Client) -> Result<()> {
    Ok(())
}
