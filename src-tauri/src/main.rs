// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
mod rules;

/* 駒の初期配置 */
const START_POSITION: [Piece; 64] = [
    // 8段目
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
    Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
    // 7段目
    Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
    Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
    // 6〜3段目
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    // 2段目
    Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
    Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
    // 1段目
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
    Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/* 駒の種類を定義 */
pub enum Piece { WPawn, BPawn, Knight, Bishop, Rook, Queen, King, None }

#[derive(PartialEq, Eq, Debug, Default)]
/* アンパッサン構造体 */
struct EnPassant {
    place: u8, 
    valid_turn: Option<u32>
}

#[derive(Debug, Clone, Copy)]
/* 判別用ボードの作成 */
struct PieceType([Piece; 64]);

#[derive(Debug, Default)]
/* ゲームデータの管理 */
struct GameState {
    move_count: u32, 
    en_passant: EnPassant, 
    piecet: PieceType, 
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

/* defaultの手動設定 */
impl Default for PieceType {
    fn default() -> Self {
        PieceType(START_POSITION)
    }
}

/** reset: ボードを初期化 */
impl GameState {
    fn reset(&mut self) {
        self.move_count = 0;
        self.en_passant = EnPassant { place: 0, valid_turn: None };
        self.piecet = PieceType::default();
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
/** アプリの起動、ボードデータの共有化 */
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
