// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::lib::tauri_commands;
use dotenv::dotenv;

mod lib {
    pub mod mongo_driver;
    pub mod tauri_commands;
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_commands::create_item_command,
            tauri_commands::read_item_command,
            tauri_commands::update_item_command,
            tauri_commands::delete_item_command,
            tauri_commands::fetch_items_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
