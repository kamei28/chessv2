use core::arch::x86_64::_rdtsc;
use std::io::{stdout, Write};
use std::{i8, sync::{Arc, Mutex}, time::Instant};
use rayon::prelude::*;
use tauri::State;

use crate::{EnPassant, GameState, /*Piece*/};

/** Constants used for rank */
const RANK_SHIFT:   u8 =  8;
const RANK_INDEX:   u8 =  8;
const RANK_UP:      i8 =  8;
const RANK_DOWN:    i8 = -8;

/** Constants used for piece move generation */
const PAWN_CENTER:  u8 =  1;
const SHIFT_BASE:   u32 = (PAWN_CENTER + 8) as u32;
const KNIGHT_CENTER:u8 = 18;

/** メモ
 * マスは0~63
 * シフトは1~
 */

/** 各駒の合法手を作成する関数を追加 */
impl GameState {
    /** 駒の移動処理 */
    fn mvoe_piece(&mut self, from: u8, to: u8) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;
        let mut board = &mut self.error;

        // 駒別移動処理: ポーン
        if self.pawn & from_mask != 0 {
            self.handle_en_passant(from, to);
            board = &mut self.pawn;
        } else {
            self.pawn &= !to_mask;
        }

        // 駒別移動処理: ナイト
        if self.knight & from_mask != 0 {
            board = &mut self.knight;
        } else {
            self.knight &= !to_mask;
        }

        // 駒別移動処理: ビショップ
        if self.bishop & from_mask != 0 {
            board = &mut self.bishop;
        } else {
            self.bishop &= !to_mask;
        }

        // 駒別移動処理: ルーク
        if self.rook & from_mask != 0 {
            board = &mut self.rook;
        } else {
            self.rook &= !to_mask;
        }

        // 駒別移動処理: クイーン
        if self.queen & from_mask != 0 {
            board = &mut self.queen;
        } else {
            self.queen &= !to_mask;
        }

        // 駒別移動処理: キング
        if self.king & from_mask != 0 {
            board = &mut self.king;
        } else {
            self.king &= !to_mask;
        }

        // 色別移動処理: ホワイト、ブラック
        if (self.white >> from) & 0b1 != 0 {
            self.white &= !from_mask;
            self.white |= to_mask;
            self.black &= !to_mask;
        } else {
            self.black &= !from_mask;
            self.black |= to_mask;
            self.white &= !to_mask;
        }

        *board &= !from_mask;
        *board |= to_mask;

        self.move_count += 1;
    }

    fn handle_en_passant(&mut self, from: u8, to: u8) {
        // 移動先のアンパッサン構造体を定義
        let rank_shift = if self.move_count & 0b1 == 0 {
            RANK_UP
        } else {
            RANK_DOWN
        };
        let expected_en_passant = EnPassant {
            place: ((to as i8) + rank_shift) as u8,
            valid_turn: Some(self.move_count),
        };

        // アンパッサンの有効判定
        if expected_en_passant == self.en_passant {
            self.white &= !(1u64 << to << RANK_SHIFT);
            self.black &= !(1u64 << to >> RANK_SHIFT);

        // 2マス飛び判定
        } else if from.abs_diff(to) == 2 * RANK_SHIFT {
            // アンパッサンに登録
            self.en_passant = EnPassant {
                place: from,
                valid_turn: Some(self.move_count + 1),
            };
        }
    }

    /** インデックスから駒の可動範囲を調べる */
    #[inline(always)]
    fn get_valid_moves(&self, loc: u8) -> u64 {
        let bit_mask = 1u64 << loc;

        // if let Some(piece) = self.piecet.0.get(loc as usize) {
        //     // println!("{:?}", piece);
        // }

        // match self.piecet.0[loc as usize] {
        //     Piece::BPawn    =>  self.generate_pawn_moves(loc), 
        //     Piece::WPawn    =>  self.generate_pawn_moves(loc), 
        //     Piece::Knight   =>  self.generate_knight_moves(loc), 
        //     Piece::Bishop   =>  self.generate_bishop_moves(loc), 
        //     Piece::Rook     =>  self.generate_rook_moves(loc), 
        //     Piece::Queen    =>  self.generate_queen_moves(loc), 
        //     Piece::King     =>  self.generate_king_moves(loc), 
        //     _ => { 0x0 }
        // }

        if self.pawn & bit_mask != 0 { self.generate_pawn_moves(loc) }
        else if self.knight & bit_mask  != 0 { self.generate_knight_moves(loc)  }
        else if self.bishop & bit_mask  != 0 { self.generate_bishop_moves(loc)  }
        else if self.rook   & bit_mask  != 0 { self.generate_rook_moves(loc)    }
        else if self.queen  & bit_mask  != 0 { self.generate_queen_moves(loc)   }
        else if self.king   & bit_mask  != 0 { self.generate_king_moves(loc)    }
        else { 0x0 }
    }

    // legal pawn moves
    fn generate_pawn_moves(&self, loc: u8) -> u64 {
        // let start_instant = Instant::now();
        // let start = unsafe { _rdtsc() };

        // for _i in 0..=10000000 {

        // ベース定義
        let g1 = (self.black >> loc & 1) as u8;
        let g3 = loc & !7;
        let g2 = (!g1 & (g3 == 8) as u8) | (g1 & (g3 == 48) as u8);
        let g4 = g1 << 4;

        // 前方確認
        let mut v1 = self.white | self.black;
        let mut v2 = 0x100 << loc >> g4;
        // v1 ^= v2;

        // 2マス
        let v3 = g2 * ((v1 ^ v2) & v2 != 0) as u8;
        v2 |= 0x10001*(v3 as u64) << loc >> g4;
        v1 ^= v2;
        v1 &= v2;

        // 斜め前確認
        let mut v2 = if g1 != 0 { self.white } else { self.black };
        v2 &= 0x280 << loc >> g4;

        // 合法手作成
        // let pw_mask = (v1 | v2) & !(0xff << g3);
        (v1 | v2) & !(0xff << g3)
        
        // }

        // let end = unsafe { _rdtsc() };
        // println!("{:?}", start_instant.elapsed());
        // println!("cycles: {}", end - start);

        // ret_loc(ret & mask)
        // pw_mask
        // vec![]
    }

    // legal knight moves
    fn generate_knight_moves(&self, loc: u8) -> u64 {
        // let start_instant = Instant::now();
        // let start = unsafe { _rdtsc() };

        // for _i in 0..=10000000 {

        // 可動範囲がはみ出ないように制限
        let v1 = ((loc & 7 < 4) as u64).wrapping_neg();
        let mut knight_moves = (v1 & 0x3f3f3f3f3f3f3f3fu64) | (!v1 & 0xfcfcfcfcfcfcfcfcu64);

        // ベース可動範囲を適応
        knight_moves &= if loc < KNIGHT_CENTER {
            0xa1100110au64 >> (KNIGHT_CENTER - loc)
        } else {
            0xa1100110au64 << (loc - KNIGHT_CENTER)
        };

        // ボード全体の可動マスクを作成
        let v1 = ((self.white >> loc) & 1).wrapping_neg();
        let v2 = !self.white & (self.black | knight_moves);
        let v3 = !self.black & (self.white | knight_moves);
        knight_moves &= (v1 & v2) | (!v1 & v3);

        // black_box(knight_moves);

        // }

        // let end = unsafe { _rdtsc() };
        // println!("{:?}", start_instant.elapsed());
        // println!("cycles: {}", end - start);

        // ret_loc(knight_moves)
        knight_moves
        // vec![]
    }

    // legal bishop moves
    fn generate_bishop_moves(&self, loc: u8) -> u64 {
        // vec![]
        0x0
    }

    // legal rook moves
    fn generate_rook_moves(&self, loc: u8) -> u64 {
        // vec![]
        0x0
    }

    // legal queen moves
    fn generate_queen_moves(&self, loc: u8) -> u64 {
        // vec![]
        0x0
    }

    // legal kings moves
    fn generate_king_moves(&self, loc: u8) -> u64 {
        // vec![]
        0x0
    }
}

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

/** 駒の移動処理(アプリ用) */
#[tauri::command]
pub fn mvoe_piece(from: u8, to: u8, state: State<Arc<Mutex<GameState>>>) -> Option<i8> {
    let mut maps = state.lock().unwrap();

    // 駒の移動処理
    println!("move from {from} to {to}");
    maps.mvoe_piece(from, to);

    let is_pawn = maps.pawn & (1u64 << to) != 0;
    let loc_abs = maps.en_passant.place.abs_diff(to);

    // 削除する駒の位置を返す
    if let Some(turn) = maps.en_passant.valid_turn {
        // アンパッサンされたか判定
        if is_pawn && turn == (maps.move_count - 1) && loc_abs == RANK_INDEX {
            let rank_shift = if turn & 0b1 != 0 { RANK_UP } else { RANK_DOWN };

            // アンパッサンされた駒の位置を返す
            Some((maps.en_passant.place as i8) + 2 * rank_shift)
        } else {
            None
        }
    } else {
        None
    }
}

/** debag */
#[tauri::command]
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