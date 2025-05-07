// Public modules.
pub mod account;
pub mod security;
pub mod trade_execution;

// Internal modules.
mod migrations;

use account::BrokerageAccount;
use anyhow::Result;
use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::Database;
use security::Security;
use tfiala_mongodb_migrator::migrator::default::DefaultMigrator;

pub async fn run_migrations(db: Database) -> Result<()> {
    DefaultMigrator::new()
        .with_conn(db.clone())
        .with_migrations_vec(migrations::get_migrations())
        .up()
        .await?;
    Ok(())
}

pub async fn insert_brokerage_account(
    db: &Database,
    brokerage_account: &BrokerageAccount,
) -> Result<()> {
    let result = db
        .collection::<BrokerageAccount>(BrokerageAccount::COLLECTION_NAME)
        .insert_one(brokerage_account)
        .await?;
    tracing::info!(
        "inserted brokerage-account {:?}: (reported insert id: {:?})",
        brokerage_account,
        result.inserted_id
    );
    Ok(())
}

pub async fn insert_security(db: &Database, security: &Security) -> Result<()> {
    let result = db
        .collection::<Security>(Security::COLLECTION_NAME)
        .insert_one(security)
        .await?;
    tracing::info!(
        "inserted security {:?}: (reported insert id: {:?})",
        security,
        result.inserted_id
    );
    Ok(())
}

pub async fn find_security(
    db: &Database,
    ticker: &str,
    listing_exchange: &str,
) -> Result<Option<Security>> {
    Ok(db
        .collection::<Security>(Security::COLLECTION_NAME)
        .find_one(doc! { "ticker": ticker, "listing_exchange": listing_exchange })
        .await?)
}

pub async fn find_security_by_conid(db: &Database, ibkr_conid: u32) -> Result<Option<Security>> {
    Ok(db
        .collection::<Security>(Security::COLLECTION_NAME)
        .find_one(doc! { "ibkr_conid": ibkr_conid })
        .await?)
}

pub async fn find_security_by_ticker(db: &Database, ticker: &str) -> Result<Vec<Security>> {
    let result = db
        .collection::<Security>(Security::COLLECTION_NAME)
        .find(doc! { "ticker": ticker })
        .await?;

    // Convert the cursor to a vector of Security objects.
    Ok(result.try_collect().await?)
}
