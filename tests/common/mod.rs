use anyhow::Result;
use brokerage_db::{account::IBrokerageAccount, db_connection::DbConnection};
use testcontainers::{ContainerAsync, Image};

pub struct CommonTests<T: Image> {
    db_conn: Box<dyn DbConnection>,
    _test_container: ContainerAsync<T>,
}

impl<T: Image> CommonTests<T> {
    const BROKERAGE_ID: &str = "batch-brokers";
    const BROKERAGE_ACCOUNT_ID: &str = "A1234567";

    const BROKERAGE_ID_2: &str = "another-broker";
    const BROKERAGE_ACCOUNT_ID_2: &str = "DA7654321";

    pub fn new(db_conn: Box<dyn DbConnection>, test_container: ContainerAsync<T>) -> Self {
        Self {
            db_conn,
            _test_container: test_container,
        }
    }

    fn brokerage_account(&self) -> Box<dyn IBrokerageAccount> {
        self.db_conn
            .new_brokerage_account(Self::BROKERAGE_ACCOUNT_ID, Self::BROKERAGE_ID)
    }

    fn brokerage_account_2(&self) -> Box<dyn IBrokerageAccount> {
        self.db_conn
            .new_brokerage_account(Self::BROKERAGE_ACCOUNT_ID_2, Self::BROKERAGE_ID_2)
    }

    pub async fn insert_brokerage_account_works(&self) -> Result<()> {
        let brokerage_account = self.brokerage_account();
        self.db_conn
            .insert_bacct(brokerage_account.as_ref())
            .await?;

        let found_account = self
            .db_conn
            .find_bacct_by_brokerage_and_account_id(Self::BROKERAGE_ID, Self::BROKERAGE_ACCOUNT_ID)
            .await?;
        assert!(found_account.is_some());
        let found_account = found_account.unwrap();

        assert_eq!(brokerage_account.account_id(), found_account.account_id());
        assert_eq!(
            brokerage_account.brokerage_id(),
            found_account.brokerage_id()
        );

        Ok(())
    }

    pub async fn insert_duplicate_brokerage_account_fails(&self) -> Result<()> {
        let brokerage_account = self.brokerage_account();

        // Insert it once.
        self.db_conn
            .insert_bacct(brokerage_account.as_ref())
            .await?;

        // Insert it again.
        let result = self.db_conn.insert_bacct(brokerage_account.as_ref()).await;

        assert!(result.is_err());

        Ok(())
    }

    pub async fn find_all_brokerage_accounts_works(&self) -> Result<()> {
        let brokerage_account = self.brokerage_account();
        self.db_conn
            .insert_bacct(brokerage_account.as_ref())
            .await?;

        let brokerage_account_2 = self.brokerage_account_2();
        self.db_conn
            .insert_bacct(brokerage_account_2.as_ref())
            .await?;

        let found_accounts = self.db_conn.find_bacct_all().await?;

        assert_eq!(found_accounts.len(), 2);

        assert!(
            brokerage_account.account_id() == found_accounts[0].account_id()
                || brokerage_account.account_id() == found_accounts[1].account_id()
        );
        assert!(
            brokerage_account.brokerage_id() == found_accounts[0].brokerage_id()
                || brokerage_account.brokerage_id() == found_accounts[1].brokerage_id()
        );

        assert!(
            brokerage_account_2.account_id() == found_accounts[0].account_id()
                || brokerage_account_2.account_id() == found_accounts[1].account_id()
        );
        assert!(
            brokerage_account_2.brokerage_id() == found_accounts[0].brokerage_id()
                || brokerage_account_2.brokerage_id() == found_accounts[1].brokerage_id()
        );
        Ok(())
    }
}
