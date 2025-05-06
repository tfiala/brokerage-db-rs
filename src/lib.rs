// Public modules.
pub mod account;
pub mod security;
pub mod trade_execution;

// Internal modules.
mod migrations;

use anyhow::Result;
use mongodb::Database;
use tfiala_mongodb_migrator::migrator::default::DefaultMigrator;

pub async fn run_migrations(db: Database) -> Result<()> {
    DefaultMigrator::new()
        .with_conn(db.clone())
        .with_migrations_vec(migrations::get_migrations())
        .up()
        .await?;

    Ok(())
}
