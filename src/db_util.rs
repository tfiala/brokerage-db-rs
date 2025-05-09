use anyhow::Result;
use mongodb::{ClientSession, Database};
use serde::Serialize;
use std::{any::type_name, fmt::Debug, sync::Arc};
use tokio::sync::Mutex;

pub async fn insert<T>(
    t: &T,
    db: &Database,
    collection_name: &str,
    session: Option<Arc<Mutex<ClientSession>>>,
) -> Result<()>
where
    T: Serialize + Send + Sync + Debug,
{
    let collection = db.collection::<T>(collection_name);

    let result = if session.is_some() {
        let session_am = session.unwrap();
        collection
            .insert_one(t)
            .session(&mut *session_am.lock().await)
            .await?
    } else {
        collection.insert_one(t).await?
    };

    tracing::info!(
        "inserted {} {:?}, _id: {}",
        type_name::<T>(),
        t,
        result.inserted_id
    );
    Ok(())
}
