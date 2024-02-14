use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use std::env;

#[tokio::main]
pub async fn add_something(name: &str) -> mongodb::error::Result<()> {
    let title: String;

    if name.is_empty() {
        title = format!("Who are you?")
    } else {
        title = format!("Your name is {name}!")
    }

    let new_doc = doc! {
        "title": title,
        "content": "This is content",
        "name": name
    };

    let collection = get_collection("experiences").await?;
    collection.insert_one(new_doc, None).await?;

    Ok(())
}

async fn get_collection(name: &str) -> Result<Collection<Document>, mongodb::error::Error> {
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("curvitia-db");
    let collection = db.collection::<Document>(name);
    Ok(collection)
}
