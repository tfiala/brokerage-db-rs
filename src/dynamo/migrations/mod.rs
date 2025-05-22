mod runner;
mod v001_add_brokerage_account;

use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use runner::Runner;
use v001_add_brokerage_account::*;

#[async_trait]
pub trait Migration: Send + Sync {
    fn id(&self) -> &'static str;
    async fn up(&self, client: &Client) -> Result<()>;
    async fn down(&self, client: &Client) -> Result<()>;
}

pub async fn run_migrations(client: &Client) -> Result<()> {
    let runner = Runner::new(vec![Box::new(MigrationV001 {})]);
    runner.up(client).await
}

pub async fn remove_migrations(client: &Client) -> Result<()> {
    let runner = Runner::new(vec![Box::new(MigrationV001 {})]);
    runner.down(client).await
}
