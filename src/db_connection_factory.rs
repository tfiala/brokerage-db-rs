use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use crate::db_connection::DbConnection;

#[async_trait]
pub trait DbConnectionFactory<I> {
    fn id(&self) -> &'static str;
    async fn create(&self) -> Result<Box<dyn DbConnection<I>>>;
}

pub struct DbConnectionFactoryManager<I> {
    factories: HashMap<&'static str, Box<dyn DbConnectionFactory<I>>>,
}

impl<I> DbConnectionFactoryManager<I> {
    pub fn register_factory(&mut self, factory: Box<dyn DbConnectionFactory<I>>) {
        self.factories.insert(factory.id(), factory);
    }
}
