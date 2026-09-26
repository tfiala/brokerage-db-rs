use anyhow::Result;
use brokerage_db::{
    account::IBrokerageAccount,
    db_connection::DbConnection,
    security::{ISecurity, SecurityType},
};
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

    // =======================
    // Brokerage Account Tests
    // =======================

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

    fn security(&self) -> Result<Box<dyn ISecurity>> {
        self.db_conn
            .new_security(SecurityType::Stock, "AAPL", "NASDAQ", None)
    }

    fn security_with_conid(&self) -> Result<Box<dyn ISecurity>> {
        self.db_conn
            .new_security(SecurityType::Stock, "AAPL", "NASDAQ", Some(12345678))
    }

    pub async fn insert_security_works(&self) -> Result<()> {
        let security = self.security()?;
        self.db_conn.insert_security(security.as_ref()).await?;

        let found_security = self
            .db_conn
            .find_security_by_ticker_and_exchange(
                security.security_type,
                security.ticker(),
                security.listing_exchange(),
            )
            .await?;

        assert!(found_security.is_some());
        assert_eq!(security, found_security.unwrap());

        Ok(())
    }

    pub async fn insert_security_with_conid_works(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security_with_conid: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        security_with_conid.insert(&dbc.db, None).await?;

        let found_security = dbc
            .db
            .collection::<Security>(Security::COLLECTION_NAME)
            .find_one(bson::doc! {
            "ticker": security_with_conid.ticker(),
            "listing_exchange": security_with_conid.listing_exchange() })
            .await?
            .ok_or_else(|| anyhow::anyhow!("Security not found"))?;

        assert_eq!(security_with_conid, found_security);

        Ok(())
    }

    pub async fn find_non_extant_security_fails(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        let result = Security::find_by_ticker_and_exchange(
            &dbc.db,
            security.ticker(),
            security.listing_exchange(),
        )
        .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        Ok(())
    }

    pub async fn find_security_with_ticker_and_exchange_works(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        security.insert(&dbc.db, None).await?;

        let result = Security::find_by_ticker_and_exchange(
            &dbc.db,
            security.ticker(),
            security.listing_exchange(),
        )
        .await;
        assert!(result.is_ok());

        let found_security = result.unwrap();
        assert!(found_security.is_some());
        assert_eq!(security, found_security.unwrap());

        Ok(())
    }

    pub async fn find_security_with_ticker_and_one_match_works(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        security.insert(&dbc.db, None).await?;

        let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
        assert!(result.is_ok());

        let found_securities = result.unwrap();
        assert_eq!(found_securities.len(), 1);
        assert_eq!(security, found_securities[0]);

        Ok(())
    }

    pub async fn find_non_extant_security_with_ticker_returns_zero_elements(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;

        let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
        assert!(result.is_ok());

        let found_securities = result.unwrap();
        assert_eq!(found_securities.len(), 0);

        Ok(())
    }

    pub async fn find_security_with_ticker_and_two_match_works(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        security.insert(&dbc.db, None).await?;

        let security2 = Security::new(SecurityType::Stock, security.ticker(), "NYSE", None);
        security2.insert(&dbc.db, None).await?;

        let result = Security::find_by_ticker(&dbc.db, security.ticker()).await;
        assert!(result.is_ok());

        let found_securities = result.unwrap();
        assert_eq!(found_securities.len(), 2);
        assert!(found_securities.contains(&security));
        assert!(found_securities.contains(&security2));

        Ok(())
    }

    pub async fn find_security_by_conid_works(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security_with_conid: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;
        security_with_conid.insert(&dbc.db, None).await?;

        let result =
            Security::find_by_conid(&dbc.db, security_with_conid.ibkr_conid().unwrap()).await;
        assert!(result.is_ok());

        let found_security = result.unwrap();
        assert!(found_security.is_some());
        assert_eq!(security_with_conid, found_security.unwrap());

        Ok(())
    }

    pub async fn find_non_extant_security_by_conid_fails(
        #[future] test_db_conn_old: Result<DbConnectionOld>,
        security_with_conid: Security,
    ) -> Result<()> {
        let dbc = test_db_conn_old?;

        let result =
            Security::find_by_conid(&dbc.db, security_with_conid.ibkr_conid().unwrap()).await;
        assert!(result.is_ok());

        let found_security = result.unwrap();
        assert!(found_security.is_none());

        Ok(())
    }
}
