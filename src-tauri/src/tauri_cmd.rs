use core::arch::x86_64::_rdtsc;
use std::io::{stdout, Write};
use std::{i8, sync::{Arc, Mutex}, time::Instant};
use rayon::prelude::*;
use tauri::State;

use crate::engine::position::game_state::*;
use crate::engine::position::consts::*;

#[tauri::command]
/** 駒出力、並列処理テスト */
pub fn view(state: State<Arc<Mutex<GameState>>>) {
    let maps = state.lock().unwrap();
    let fields = [
        ("white", maps.white),
        ("black", maps.black),
        ("pawn", maps.pawn),
        ("rook", maps.rook),
        ("knight", maps.knight),
        ("bishop", maps.bishop),
        ("qeen", maps.queen),
        ("king", maps.king),
    ];

    // Rayon の parallel iterator で並列処理
    fields.par_iter().for_each(|(name, val)| {
        let bits = format!("{:064b}", val);
        let formatted: String = bits
            .as_bytes()
            .chunks(8)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}:\t{}", name, formatted);
    });
}

#[tauri::command]
/** 戦局をリセット */
pub fn reset(state: State<Arc<Mutex<GameState>>>) {
    let mut maps = state.lock().unwrap();

    maps.reset();
    println!("The board was reset.");
}

#[tauri::command]
/** 駒の可動範囲を取得 */
pub fn get_valid_moves(loc: u8, state: State<Arc<Mutex<GameState>>>) -> Vec<u8> {
    let maps = state.lock().unwrap();

    let start_instant = Instant::now();
    let start = unsafe { _rdtsc() };

    for _i in 0..=10000 {

    maps.get_valid_moves(loc);

    }

    let end = unsafe { _rdtsc() };
    println!("{:?}", start_instant.elapsed());
    println!("cycles: {}", end - start);
    stdout().flush().unwrap();
    
    ret_loc(maps.get_valid_moves(loc))
}

#[tauri::command]
/** 駒の移動処理(アプリ用) */
pub fn move_piece(from: u8, to: u8, state: State<Arc<Mutex<GameState>>>) -> Option<i8> {
    let mut maps = state.lock().unwrap();

    // アンパッサンを反映
    let en = if maps.en_passant == to {
        if maps.white & (1 << from) == 0 {
            Some(maps.en_passant as i8 + RANK_STEP)
        } else {
            Some(maps.en_passant as i8 - RANK_STEP)
        }
    } else {
        None
    };

    // 駒の移動処理
    println!("move from {from} to {to}");
    maps.move_piece(from, to);

    // 削除する駒の位置を返す
    en
}

#[tauri::command]
/** debag */
pub fn test(loc: u8, state: State<Arc<Mutex<GameState>>>) -> u64 {
    let maps = state.lock().unwrap();
    let start = unsafe { _rdtsc() };

    maps.get_valid_moves(loc);

    let end = unsafe { _rdtsc() };
    end - start
}

/** 1の場所を返す */
fn ret_loc(mut board: u64) -> Vec<u8> {
    let mut ret = Vec::new();
    while board != 0 {
        ret.push(board.trailing_zeros() as u8);
        board &= board - 1;
    }
    ret
}
