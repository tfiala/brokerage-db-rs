// Public modules.
pub mod account;
pub mod security;
pub mod trade_execution;

// Internal modules.
mod migrations;

use anyhow::Result;
use mongodb::Database;

pub async fn initialize(db: &Database) -> Result<()> {
    migrations::run_migrations(db).await
}
