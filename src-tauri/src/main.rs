// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
mod rules;

#[derive(PartialEq, Eq, Debug, Default)]
struct EnPassant {
    place: u8, 
    valid_turn: u32
}

#[derive(Debug, Default)]
/** ボードの構造体を作成 */
struct GameState {
    move_count: u32, 
    en_passant: EnPassant, 
    white:  u64, 
    black:  u64, 
    pawn:   u64, 
    rook:   u64, 
    knight: u64, 
    bishop: u64, 
    queen:  u64, 
    king:   u64, 
    error:  u64
}

/** reset: ボードを初期化 */
impl GameState {
    fn reset(&mut self) {
        self.move_count = 0;
        self.en_passant = EnPassant { place: 0, valid_turn: 0 };
        self.white  = 0xffff;
        self.black  = 0xffff << 0x30;
        self.pawn   = 0xff << 0x30 | 0xff00;
        self.rook   = 0x81 << 0x38 | 0x81;
        self.knight = 0x42 << 0x38 | 0x42;
        self.bishop = 0x24 << 0x38 | 0x24;
        self.queen  = 0x10 << 0x38 | 0x10;
        self.king   = 0x08 << 0x38 | 0x08;
        self.error  = 0;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/** アプリを起動、ボードを共有 */
fn main() {
    let mut board = GameState::default();
    board.reset();

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(board)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            rules::view, 
            rules::reset, 
            rules::get_valid_moves, 
            rules::mvoe_piece, 
            rules::test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
