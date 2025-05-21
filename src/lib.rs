// Public modules.
pub mod account;
pub mod db_connection;
pub mod dynamo;
pub mod eod_summary;
pub mod mongo;
pub mod security;
pub mod trade_execution;

// Internal modules.
mod db_util;
mod migrations;

use anyhow::Result;
use mongodb::Database;

pub async fn initialize(db: &Database) -> Result<()> {
    migrations::run_migrations(db).await
}

pub async fn remove_data(db: &Database) -> Result<()> {
    migrations::run_down_migrations(db).await
}
