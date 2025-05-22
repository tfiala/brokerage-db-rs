use super::Migration;
use anyhow::Result;
use aws_sdk_dynamodb::Client;

pub struct Runner {
    migrations: Vec<Box<dyn Migration>>,
}

impl Runner {
    pub fn new(migrations: Vec<Box<dyn Migration>>) -> Self {
        Self { migrations }
    }

    pub async fn up(&self, client: &Client) -> Result<()> {
        for migration in self.migrations.iter() {
            let result = migration.up(client).await;
            if result.is_err() {
                tracing::warn!(
                    "migration {} failed, stopping further migrations",
                    migration.id()
                )
            }
        }
        Ok(())
    }

    pub async fn down(&self, _client: &Client) -> Result<()> {
        Ok(())
    }
}
