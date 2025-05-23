// Public modules.
pub mod account;
pub mod db_connection;
pub mod db_connection_factory;
pub mod dynamo;
pub mod eod_summary;
pub mod mongo;
pub mod security;
pub mod trade_execution;

// Internal modules.
mod db_util;

use anyhow::Result;
use mongodb::Database;

pub async fn initialize(db: &Database) -> Result<()> {
    // TODO fix me: register the database connection and use it to run migrations.
    mongo::migrations::run_migrations(db).await
}

pub async fn remove_data(db: &Database) -> Result<()> {
    // TODO fix me: register the database connection and use it to run migrations.
    mongo::migrations::run_down_migrations(db).await
}
