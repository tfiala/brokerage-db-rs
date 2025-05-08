use anyhow::Result;
use mongodb::Database;
use tfiala_mongodb_migrator::{migration::Migration, migrator::default::DefaultMigrator};

mod v001_add_accounts;
mod v002_add_security;
mod v003_add_trade_executions;

fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(v001_add_accounts::Migration001 {}),
        Box::new(v002_add_security::Migration002 {}),
        Box::new(v003_add_trade_executions::Migration003 {}),
    ]
}

pub async fn run_migrations(db: &Database) -> Result<()> {
    DefaultMigrator::new()
        .with_conn(db.clone())
        .with_migrations_vec(get_migrations())
        .up()
        .await?;
    Ok(())
}
