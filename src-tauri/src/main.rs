// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::invoke_handlers::download_video::download_video;

pub mod invoke_handlers;

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
