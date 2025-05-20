use std::sync::Arc;

use anyhow::Result;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::{ClientSession, Database};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{account::BrokerageAccount, db_util};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EODSummary {
    _id: ObjectId,
    brokerage_account_id: ObjectId,
    start_timestamp_ms: i64,
    end_timestamp_ms: i64,

    starting_cash: f64,
    ending_cash: f64,

    commissions: f64,
    commissions_mtd: Option<f64>,
    commissions_ytd: Option<f64>,

    deposits: f64,
    deposits_mtd: Option<f64>,
    deposits_ytd: Option<f64>,

    dividends: f64,
    dividends_mtd: Option<f64>,
    dividends_ytd: Option<f64>,

    interest: f64,
    interest_mtd: Option<f64>,
    interest_ytd: Option<f64>,

    net_trade_purchases: f64,
    net_trade_sales: f64,

    other_fees: f64,
    other_fees_mtd: Option<f64>,
    other_fees_ytd: Option<f64>,

    withdrawals: f64,
    withdrawals_mtd: Option<f64>,
    withdrawals_ytd: Option<f64>,
}

pub struct Builder {
    _id: ObjectId,
    brokerage_account_id: Option<ObjectId>,
    start_timestamp_ms: Option<i64>,
    end_timestamp_ms: Option<i64>,

    starting_cash: Option<f64>,
    ending_cash: Option<f64>,

    commissions: Option<f64>,
    commissions_mtd: Option<f64>,
    commissions_ytd: Option<f64>,

    deposits: Option<f64>,
    deposits_mtd: Option<f64>,
    deposits_ytd: Option<f64>,

    dividends: Option<f64>,
    dividends_mtd: Option<f64>,
    dividends_ytd: Option<f64>,

    interest: Option<f64>,
    interest_mtd: Option<f64>,
    interest_ytd: Option<f64>,

    net_trade_purchases: Option<f64>,
    net_trade_sales: Option<f64>,

    other_fees: Option<f64>,
    other_fees_mtd: Option<f64>,
    other_fees_ytd: Option<f64>,

    withdrawals: Option<f64>,
    withdrawals_mtd: Option<f64>,
    withdrawals_ytd: Option<f64>,
}

impl EODSummary {
    pub const COLLECTION_NAME: &'static str = "eod_summaries";

    pub fn builder() -> Builder {
        Builder {
            _id: ObjectId::new(),
            brokerage_account_id: None,
            start_timestamp_ms: None,
            end_timestamp_ms: None,

            starting_cash: None,
            ending_cash: None,

            commissions: None,
            commissions_mtd: None,
            commissions_ytd: None,

            deposits: None,
            deposits_mtd: None,
            deposits_ytd: None,

            dividends: None,
            dividends_mtd: None,
            dividends_ytd: None,

            interest: None,
            interest_mtd: None,
            interest_ytd: None,

            net_trade_purchases: None,
            net_trade_sales: None,

            other_fees: None,
            other_fees_mtd: None,
            other_fees_ytd: None,

            withdrawals: None,
            withdrawals_mtd: None,
            withdrawals_ytd: None,
        }
    }

    pub fn id(&self) -> ObjectId {
        self._id
    }

    pub fn brokerage_account_id(&self) -> ObjectId {
        self.brokerage_account_id
    }

    pub fn start_timestamp_ms(&self) -> i64 {
        self.start_timestamp_ms
    }

    pub fn end_timestamp_ms(&self) -> i64 {
        self.end_timestamp_ms
    }

    pub fn starting_cash(&self) -> f64 {
        self.starting_cash
    }

    pub fn ending_cash(&self) -> f64 {
        self.ending_cash
    }

    pub fn commissions(&self) -> f64 {
        self.commissions
    }

    pub fn deposits(&self) -> f64 {
        self.deposits
    }

    pub fn dividends(&self) -> f64 {
        self.dividends
    }

    pub fn interest(&self) -> f64 {
        self.interest
    }

    pub fn net_trade_purchases(&self) -> f64 {
        self.net_trade_purchases
    }

    pub fn net_trade_sales(&self) -> f64 {
        self.net_trade_sales
    }

    pub fn other_fees(&self) -> f64 {
        self.other_fees
    }

    pub fn withdrawals(&self) -> f64 {
        self.withdrawals
    }

    pub async fn insert(
        &self,
        db: &Database,
        session: Option<Arc<Mutex<ClientSession>>>,
    ) -> Result<()> {
        db_util::insert(self, db, Self::COLLECTION_NAME, session).await
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Option<Self>> {
        let result = db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find_one(bson::doc! {"_id": id})
            .await?;

        Ok(result)
    }

    pub async fn find_by_account_id(
        db: &Database,
        brokerage_account_id: ObjectId,
    ) -> Result<Vec<Self>> {
        Ok(db
            .collection::<Self>(Self::COLLECTION_NAME)
            .find(bson::doc! {"brokerage_account_id": brokerage_account_id })
            .await?
            .try_collect()
            .await?)
    }

    pub async fn brokerage_account(&self, db: &Database) -> Result<BrokerageAccount> {
        Ok(BrokerageAccount::find_by_id(db, self.brokerage_account_id)
            .await?
            .unwrap())
    }
}

impl Builder {
    pub fn build(self) -> Result<EODSummary> {
        Ok(EODSummary {
            _id: self._id,
            brokerage_account_id: self.brokerage_account_id.unwrap(),
            start_timestamp_ms: self.start_timestamp_ms.unwrap(),
            end_timestamp_ms: self.end_timestamp_ms.unwrap(),

            starting_cash: self.starting_cash.unwrap(),
            ending_cash: self.ending_cash.unwrap(),

            commissions: self.commissions.unwrap(),
            commissions_mtd: self.commissions_mtd,
            commissions_ytd: self.commissions_ytd,

            deposits: self.deposits.unwrap(),
            deposits_mtd: self.deposits_mtd,
            deposits_ytd: self.deposits_ytd,

            dividends: self.dividends.unwrap(),
            dividends_mtd: self.dividends_mtd,
            dividends_ytd: self.dividends_ytd,

            interest: self.interest.unwrap(),
            interest_mtd: self.interest_mtd,
            interest_ytd: self.interest_ytd,

            net_trade_purchases: self.net_trade_purchases.unwrap(),
            net_trade_sales: self.net_trade_sales.unwrap(),

            other_fees: self.other_fees.unwrap(),
            other_fees_mtd: self.other_fees_mtd,
            other_fees_ytd: self.other_fees_ytd,

            withdrawals: self.withdrawals.unwrap(),
            withdrawals_mtd: self.withdrawals_mtd,
            withdrawals_ytd: self.withdrawals_ytd,
        })
    }

    pub fn brokerage_account_id(mut self, brokerage_account_id: ObjectId) -> Self {
        self.brokerage_account_id = Some(brokerage_account_id);
        self
    }

    pub fn start_timestamp_ms(mut self, start_timestamp_ms: i64) -> Self {
        self.start_timestamp_ms = Some(start_timestamp_ms);
        self
    }

    pub fn end_timestamp_ms(mut self, end_timestamp_ms: i64) -> Self {
        self.end_timestamp_ms = Some(end_timestamp_ms);
        self
    }

    pub fn starting_cash(mut self, starting_cash: f64) -> Self {
        self.starting_cash = Some(starting_cash);
        self
    }

    pub fn ending_cash(mut self, ending_cash: f64) -> Self {
        self.ending_cash = Some(ending_cash);
        self
    }

    pub fn commissions(mut self, commissions: f64) -> Self {
        self.commissions = Some(commissions);
        self
    }

    pub fn deposits(mut self, deposits: f64) -> Self {
        self.deposits = Some(deposits);
        self
    }

    pub fn dividends(mut self, dividends: f64) -> Self {
        self.dividends = Some(dividends);
        self
    }

    pub fn interest(mut self, interest: f64) -> Self {
        self.interest = Some(interest);
        self
    }

    pub fn net_trade_purchases(mut self, net_trade_purchases: f64) -> Self {
        self.net_trade_purchases = Some(net_trade_purchases);
        self
    }

    pub fn net_trade_sales(mut self, net_trade_sales: f64) -> Self {
        self.net_trade_sales = Some(net_trade_sales);
        self
    }

    pub fn other_fees(mut self, other_fees: f64) -> Self {
        self.other_fees = Some(other_fees);
        self
    }

    pub fn withdrawals(mut self, withdrawals: f64) -> Self {
        self.withdrawals = Some(withdrawals);
        self
    }

    pub fn commissions_mtd(mut self, commissions_mtd: f64) -> Self {
        self.commissions_mtd = Some(commissions_mtd);
        self
    }

    pub fn commissions_ytd(mut self, commissions_ytd: f64) -> Self {
        self.commissions_ytd = Some(commissions_ytd);
        self
    }

    pub fn deposits_mtd(mut self, deposits_mtd: f64) -> Self {
        self.deposits_mtd = Some(deposits_mtd);
        self
    }

    pub fn deposits_ytd(mut self, deposits_ytd: f64) -> Self {
        self.deposits_ytd = Some(deposits_ytd);
        self
    }

    pub fn dividends_mtd(mut self, dividends_mtd: f64) -> Self {
        self.dividends_mtd = Some(dividends_mtd);
        self
    }

    pub fn dividends_ytd(mut self, dividends_ytd: f64) -> Self {
        self.dividends_ytd = Some(dividends_ytd);
        self
    }

    pub fn interest_mtd(mut self, interest_mtd: f64) -> Self {
        self.interest_mtd = Some(interest_mtd);
        self
    }

    pub fn interest_ytd(mut self, interest_ytd: f64) -> Self {
        self.interest_ytd = Some(interest_ytd);
        self
    }

    pub fn other_fees_mtd(mut self, other_fees_mtd: f64) -> Self {
        self.other_fees_mtd = Some(other_fees_mtd);
        self
    }

    pub fn other_fees_ytd(mut self, other_fees_ytd: f64) -> Self {
        self.other_fees_ytd = Some(other_fees_ytd);
        self
    }

    pub fn withdrawals_mtd(mut self, withdrawals_mtd: f64) -> Self {
        self.withdrawals_mtd = Some(withdrawals_mtd);
        self
    }

    pub fn withdrawals_ytd(mut self, withdrawals_ytd: f64) -> Self {
        self.withdrawals_ytd = Some(withdrawals_ytd);
        self
    }
}
