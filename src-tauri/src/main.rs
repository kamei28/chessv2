// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tauri_cmd;
mod engine {
    pub mod position;
}

use std::sync::{Arc, Mutex};
use crate::engine::position::*;

/** アプリの起動、ボードデータの共有化 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut board = GameState::default();
    board.reset();

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(board)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tauri_cmd::view, 
            tauri_cmd::reset, 
            tauri_cmd::get_valid_moves, 
            tauri_cmd::move_piece, 
            tauri_cmd::test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
