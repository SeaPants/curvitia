use crate::lib::mongo_driver;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    mongo_driver::add_something(name).unwrap();
    format!("Hello, {}! You've been greeted from Rust!", name)
}
