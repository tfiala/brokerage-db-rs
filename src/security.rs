pub enum SecurityType {
    Stock,
}

pub struct Security {
    pub listing_exchange: String,
    pub security_type: SecurityType,
    pub ticker: String,
}
