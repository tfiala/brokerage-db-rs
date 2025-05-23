use super::Migration;
use anyhow::Result;
use aws_sdk_dynamodb::Client;
use tracing::{debug, info, warn};

pub struct Runner {
    migrations: Vec<Box<dyn Migration>>,
}

impl Runner {
    pub fn new(migrations: Vec<Box<dyn Migration>>) -> Self {
        info!("Creating runner with {} migrations", migrations.len());
        Self { migrations }
    }

    pub async fn up(&self, client: &Client) -> Result<()> {
        debug!("Runner running {} migrations", self.migrations.len());
        for migration in self.migrations.iter() {
            migration.up(client).await?;
        }
        Ok(())
    }

    pub async fn down(&self, client: &Client) -> Result<()> {
        for migration in self.migrations.iter().rev() {
            let result = migration.down(client).await;
            if result.is_ok() {
                debug!("migration up {} succeeded", migration.id());
            } else {
                warn!("migration up {} failed", migration.id());
                break;
            }
        }
        Ok(())
    }
}
