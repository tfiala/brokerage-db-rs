use anyhow::Result;
use async_trait::async_trait;

use crate::account::IBrokerageAccount;

#[async_trait]
pub trait DbConnection {
    //
    // Migrations
    //

    async fn run_migrations(&self) -> Result<()>;
    async fn remove_migrations(&self) -> Result<()>;

    //
    // Brokerage Accounts
    //
    fn new_brokerage_account(
        &self,
        account_id: &str,
        brokerage_id: &str,
    ) -> Box<dyn IBrokerageAccount + Send>;
    async fn insert_bacct(&self, bacct: &dyn IBrokerageAccount) -> Result<()>;
    async fn update_bacct(&self, bacct: &dyn IBrokerageAccount) -> Result<()>;

    async fn find_bacct_all(&self) -> Result<Vec<Box<dyn IBrokerageAccount>>>;
    async fn find_bacct_by_brokerage_and_account_id(
        &self,
        brokerage_id: &str,
        account_id: &str,
    ) -> Result<Option<Box<dyn IBrokerageAccount>>>;
    // async fn find_bacct_by_dbid(&self, dbid: &I) -> Result<Option<Box<dyn IBrokerageAccount>>>;
}
