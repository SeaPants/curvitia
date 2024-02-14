use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::Collection;
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub title: String,
    pub content: String,
    pub name: String,
}

pub async fn fetch_items(collection: &Collection<Item>) -> Result<Vec<Item>> {
    let cursor = collection.find(doc! {}, None).await?;
    let items: Vec<Item> = cursor.try_collect::<Vec<_>>().await?;
    Ok(items)
}

pub async fn get_collection(name: &str) -> Result<Collection<Item>> {
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("curvitia-db");
    Ok(db.collection(name))
}
