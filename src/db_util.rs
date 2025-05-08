use anyhow::Result;
use mongodb::{ClientSession, Database, action::InsertOne};
use serde::Serialize;
use std::{any::type_name, fmt::Debug};

fn maybe_add_session<'a>(
    insert_one: InsertOne<'a>,
    session: Option<&'a mut ClientSession>,
) -> InsertOne<'a> {
    match session {
        Some(session) => insert_one.session(session),
        None => insert_one,
    }
}

pub async fn insert<T>(
    t: &T,
    db: &Database,
    collection_name: &str,
    session: Option<&mut ClientSession>,
) -> Result<()>
where
    T: Serialize + Send + Sync + Debug,
{
    let collection = db.collection::<T>(collection_name);
    let db_op = collection.insert_one(t);
    let result = maybe_add_session(db_op, session).await?;

    tracing::info!(
        "inserted {} {:?}: (reported insert id: {:?})",
        type_name::<T>(),
        t,
        result.inserted_id
    );
    Ok(())
}
