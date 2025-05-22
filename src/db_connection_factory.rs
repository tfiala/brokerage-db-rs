use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use crate::db_connection::DbConnection;

#[async_trait]
pub trait DbConnectionFactory {
    fn id(&self) -> &'static str;
    async fn create(&self) -> Result<Box<dyn DbConnection>>;
}

pub struct DbConnectionFactoryManager {
    factories: HashMap<&'static str, Box<dyn DbConnectionFactory>>,
}

impl DbConnectionFactoryManager {
    pub fn register_factory(&mut self, factory: Box<dyn DbConnectionFactory>) {
        self.factories.insert(factory.id(), factory);
    }
}
