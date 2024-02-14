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
        .invoke_handler(tauri::generate_handler![tauri_commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
