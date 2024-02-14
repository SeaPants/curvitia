use crate::lib::mongo_driver::{fetch_items, get_collection};
use serde_json::json;
use tauri::command;

#[command]
pub async fn fetch_items_command() -> Result<serde_json::Value, String> {
    let collection = get_collection("experiences")
        .await
        .map_err(|e| e.to_string())?;
    let items = fetch_items(&collection).await.map_err(|e| e.to_string())?;
    Ok(json!(items))
}
