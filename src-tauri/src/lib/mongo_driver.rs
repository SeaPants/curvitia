use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::ClientOptions,
    Client, Collection,
};
use std::env;

pub async fn create_item(collection: &Collection<Document>, item: Document) -> Result<()> {
    collection.insert_one(item, None).await?;
    Ok(())
}

pub async fn read_item(collection: &Collection<Document>, item_id: &str) -> Result<Document> {
    let object_id = ObjectId::parse_str(&item_id)?;
    let item = collection.find_one(doc! { "_id": object_id }, None).await?;
    match item {
        Some(item) => Ok(item),
        None => Err(anyhow::anyhow!("Item not found")),
    }
}

pub async fn update_item(
    collection: &Collection<Document>,
    item_id: &str,
    item: Document,
) -> Result<()> {
    let object_id = ObjectId::parse_str(&item_id)?;
    collection
        .update_one(doc! { "_id": object_id }, doc! { "$set": item }, None)
        .await?;
    Ok(())
}

pub async fn delete_item(collection: &Collection<Document>, item_id: &str) -> Result<()> {
    let object_id = ObjectId::parse_str(&item_id)?;
    collection
        .delete_one(doc! { "_id": object_id }, None)
        .await?;
    Ok(())
}

pub async fn fetch_items(collection: &Collection<Document>) -> Result<Vec<Document>> {
    let cursor = collection.find(doc! {}, None).await?;
    let items: Vec<Document> = cursor.try_collect::<Vec<_>>().await?;
    Ok(items)
}

pub async fn get_collection(name: &str) -> Result<Collection<Document>> {
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("curvitia-db");
    Ok(db.collection(name))
}
