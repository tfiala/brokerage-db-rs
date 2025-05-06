use crate::{account::BrokerageAccount, security::Security};

pub enum TradeSide {
    Buy,
    Sell,
}

pub struct TradeExecution {
    pub brokerage_account: BrokerageAccount,
    pub commission: f64,
    pub quantity: u64,
    pub price: f64,
    pub security: Security,
    pub side: TradeSide,
}
