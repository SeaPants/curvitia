use crate::lib::mongo_driver::{
    create_item, delete_item, fetch_items, get_collection, read_item, update_item,
};
use mongodb::bson;
use serde_json::{json, Value};
use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn create_item_command(collection_name: &str, item: Value) -> Result<(), String> {
    let collection = get_collection(collection_name)
        .await
        .map_err(|e| e.to_string())?;
    let document = bson::to_document(&item).map_err(|e| e.to_string())?;
    create_item(&collection, document)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command(rename_all = "snake_case")]
pub async fn read_item_command(collection_name: &str, item_id: &str) -> Result<Value, String> {
    let collection = get_collection(collection_name)
        .await
        .map_err(|e| e.to_string())?;
    let item = read_item(&collection, item_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(json!(item))
}

#[command(rename_all = "snake_case")]
pub async fn update_item_command(
    collection_name: &str,
    item_id: &str,
    item: Value,
) -> Result<(), String> {
    let collection = get_collection(collection_name)
        .await
        .map_err(|e| e.to_string())?;
    let document = bson::to_document(&item).map_err(|e| e.to_string())?;
    update_item(&collection, item_id, document)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command(rename_all = "snake_case")]
pub async fn delete_item_command(collection_name: &str, item_id: &str) -> Result<(), String> {
    let collection = get_collection(collection_name)
        .await
        .map_err(|e| e.to_string())?;
    delete_item(&collection, item_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command(rename_all = "snake_case")]
pub async fn fetch_items_command(collection_name: &str) -> Result<Value, String> {
    let collection = get_collection(collection_name)
        .await
        .map_err(|e| e.to_string())?;
    let items = fetch_items(&collection).await.map_err(|e| e.to_string())?;
    Ok(json!(items))
}
