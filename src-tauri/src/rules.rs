use core::arch::x86_64::_rdtsc;
use std::io::{stdout, Write};
use std::{i8, sync::{Arc, Mutex}, time::Instant};
use rayon::prelude::*;
use tauri::State;

use crate::{EnPassant, GameState};

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
    fn get_valid_moves(&self, loc: u8) -> Vec<u8> {
        let bit_mask = 1u64 << loc;

        if self.pawn & bit_mask != 0 { self.generate_pawn_moves(loc) }
        else if self.knight & bit_mask != 0 { self.generate_knight_moves(loc) }
        else if self.bishop & bit_mask != 0 { self.generate_bishop_moves(loc) }
        else if self.rook & bit_mask != 0 { self.generate_rook_moves(loc) }
        else if self.queen & bit_mask != 0 { self.generate_queen_moves(loc) }
        else if self.king & bit_mask != 0 { self.generate_king_moves(loc) }
        else { vec![] }
    }

    // legal pawn moves
    fn generate_pawn_moves(&self, loc: u8) -> Vec<u8> {
        let start_instant = Instant::now();
        let start = unsafe { _rdtsc() };

        // for _i in 0..=10000000 {

        // // 前方3マスを適応
        // let g1 = loc + 7;
        // let g2 = 0b10u64 << g1;
        // let g3 = loc - 8;
        // let g4 = 1u64 << g3;
        // let g5 = self.white | self.black;

        // // 前進(1~2)
        // let v1 = (self.white >> loc & 1) * !0;
        // let v2 = g2 & !g5 & ((loc & !7 == 8) as u64).wrapping_neg();
        // let v2 = ((v2 != 0) as u64).wrapping_neg() & (g2 << 8);
        // let v3 = g4 & !g5 & ((loc & !7 == 48) as u64).wrapping_neg();
        // let v3 = ((v3 != 0) as u64).wrapping_neg() & (g4 >> 8);
        // let mut pw_mask = (v1 & (g2 | v2)) | (!v1 & (g4 | v3));
        // pw_mask &= !g5;
        
        // let v1 = ((self.white >> loc & 1)as u64).wrapping_neg();
        let g1 = (self.black >> loc & 1) as u8;
        // let g2 = (g1 as u64).wrapping_neg();
        let g3 = loc & !7;
        let g4 = g1 << 4;

        let mut pw_mask = 0;
        let mut v1 = self.white | self.black;
        v1 ^= 0x100 << loc >> g4;
        v1 &= 0x100 << loc >> g4;

        println!("::{}", g4);
        println!("::{:064b}", 1u64 << (loc + 8));
        println!("::{:064b}", 0x100u64 << loc >> g4);
        println!("::{:064b}", self.white | self.black);
        println!("::{:064b}", (0x100u64 << loc >> g4) ^ (self.white | self.black));

        // println!("{:064b}\n{:064b}", self.white | self.black, 0x100 << loc >> g4);

        
        // println!("{v1}, {}", v1/v1);
        // v1 ^= 0x1001*(v1/v1) << loc >> g4;

        let mut v2 = if g1 != 0 { self.white } else { self.black };
        v2 ^= 0x280 << loc >> g4;
        v2 &= 0x280 << loc >> g4;
        pw_mask |= v1 | v2;
        // pw_mask &= 0x10381 << loc >> g4;
        // println!("{pw_mask:064b}");


        // // 前方を確認
        // let mut pw_mask = self.white | self.black;
        // let v1 = loc as i8 + if g1 == 0 { 8 } else { -8 };
        // pw_mask ^= 0x101 << loc >> (g4 >> 1);

        // pw_mask &= 0x10381 << loc >> g4;


        // let is_black = (self.black >> loc & 1) as u8;
        // let mut v1 = 0x100 << loc >> (is_black << 3);
        // v1 ^= self.white | self.black;
        // v1 ^= 0x10001*((v1 != 0) as u64) >> (is_black << 4);
        // let mut pw_mask = if is_black == 0 { self.white } else { self.white };
        // pw_mask |= v1;
        // pw_mask &= 0x10381 << loc >> (is_black << 4);
        // pw_mask |= 0x10381 << loc >> g4;
        // pw_mask ^= (0b101 << g4 >> g1);
        // pw_mask &= 0xff00 << g3;

        // // 2マスチェック
        // let v2 = v1 & (0b10 << g4 >> g1);
        // pw_mask |= v2 << (2*g4 + 1) >> 2*g1;

        // let v5 = loc & !7;
        // let pw_mask = if self.white >> loc & 1 != 0 {
        //     let v2 = loc + 7;
        //     let mut v3 = !self.black | self.white;
        //     let mut v4 = ((v5 == 8 && (self.white | self.black) & (0b10 << v2) == 0) as u64).wrapping_neg();
        //     v4 = (v4 & 0x207) | (!v4 & 0b111);
        //     v3 ^= 0b101 << v2;
        //     v3 &= v4 << v2;
        //     v3 & 0xff00 << v5
        //     // v3
        // } else {
        //     let v2 = loc - 8;
        //     let mut v3 = !self.white | self.black;
        //     let mut v4 = ((v5 == 48 && (self.white | self.black) & (0b1 << v2) == 0) as u64).wrapping_neg();
        //     v4 = (v4 & 0x702) | (!v4 & 0x700);
        //     v3 ^= 0x500 << v2 >> 9;
        //     v3 &= v4 << v2 >> 9;
        //     v3 & 0xff << v5 - 8
        //     // v3
        // };
        
        
        // let v2 = 0b111 << loc >> PAWN_CENTER;
        // let pw_mask = (v1 & v2) | (!v1 & v3);

        // }
        // let pw_mask = (v1 & (v2 << 8) & v3) | (!v1 & !(v2 >> 8) & !v3);
        // let pw_mask = v3 & 0b111 << loc << 8 >> PAWN_CENTER;


        // let is_white = self.white & (1u64 << loc) != 0;
        // let board = self.white | self.black;
        // let rank_mask = 0xffu64 << (loc & !7);
        // let mut forward_mask = (0b111u64 << (loc - PAWN_CENTER)) & rank_mask;
        // let mut attack_mask = 0x101u64 << loc;

        // // ret: 全体ボードを0x101でXOR後、mask: 0b111 & 0xffでANDして可動範囲を求める(ベース)
        // let (ret, mask): (u64, u64) = if is_white {
        //     // 白ポーンの処理
        //     // 2マス飛び可能か判定
        //     if loc & !7 == RANK_INDEX && board & (attack_mask << RANK_SHIFT) == 0 {
        //         forward_mask |= 1u64 << loc << RANK_SHIFT;

        //     // アンパッサン可能か判定
        //     } else if let Some(turn) = self.en_passant.valid_turn {
        //         if self.move_count == turn
        //             && forward_mask & (1u64 << self.en_passant.place >> 2 * RANK_SHIFT) != 0
        //         {
        //             attack_mask |= 1u64 << self.en_passant.place >> 2 * RANK_SHIFT;
        //         }
        //     }

        //     // 全体ボードと可動範囲マスクを返す
        //     (
        //         board ^ attack_mask << RANK_SHIFT,
        //         forward_mask << RANK_SHIFT,
        //     )
        // } else {
        //     // 黒ポーンの処理
        //     // 2マス飛び可能か判定
        //     if loc & !7 == 6 * RANK_INDEX && board & (attack_mask >> 2 * RANK_SHIFT) == 0 {
        //         forward_mask |= 1u64 << loc >> RANK_SHIFT;

        //     // アンパッサン可能か判定
        //     } else if let Some(turn) = self.en_passant.valid_turn {
        //         if self.move_count == turn
        //             && forward_mask & (1u64 << self.en_passant.place << 2 * RANK_SHIFT) != 0
        //         {
        //             attack_mask |= 1u64 << self.en_passant.place << 3 * RANK_SHIFT;
        //         }
        //     }

        //     // 全体ボードと可動範囲マスクを返す
        //     (
        //         board ^ attack_mask >> 2 * RANK_SHIFT,
        //         forward_mask >> RANK_SHIFT,
        //     )
        // };

        let end = unsafe { _rdtsc() };
        println!("{:?}", start_instant.elapsed());
        println!("cycles: {}", end - start);

        // ret_loc(ret & mask)
        ret_loc(pw_mask)
        // vec![]
    }

    // legal knight moves
    fn generate_knight_moves(&self, loc: u8) -> Vec<u8> {
        let start_instant = Instant::now();
        let start = unsafe { _rdtsc() };

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

        let end = unsafe { _rdtsc() };
        println!("{:?}", start_instant.elapsed());
        println!("cycles: {}", end - start);

        ret_loc(knight_moves)
        // vec![]
    }

    // let end = unsafe { _rdtsc() };
    // println!("{:?}", start_instant.elapsed());
    // println!("cycles: {}", end - start);

    // // 可動範囲がはみ出ないように制限
    // let mut knight_mask = if loc & 7 < 4 {
    //     0x3f3f3f3f3f3f3f3fu64   // 左2列除外
    // } else {
    //     0xfcfcfcfcfcfcfcfcu64   // 右2列除外
    // };

    // // ベース可動範囲を適応
    // knight_mask &= if loc < KNIGHT_CENTER {
    //     0xa1100110au64 >> (KNIGHT_CENTER - loc)
    // } else {
    //     0xa1100110au64 << (loc - KNIGHT_CENTER)
    // };

    // // ボード全体の可動マスクを作成
    // let board = if self.white & (1u64 << loc) != 0 {
    //     !self.white & (self.black | knight_mask)
    // } else {
    //     !self.black & (self.white | knight_mask)
    // };

    // ret_loc(knight_moves)

    // 可動範囲がはみ出ないように制限
    // let v1 = ((loc & 7 < 4) as u64).wrapping_neg();
    // let mut knight_moves = (v1 & 0x3f3f3f3f3f3f3f3fu64) | (!v1 & 0xfcfcfcfcfcfcfcfcu64);

    // // ベース可動範囲を適応
    // let v1 = ((loc < KNIGHT_CENTER) as u64).wrapping_neg();
    // let v2 = 0xa1100110au64 >> KNIGHT_CENTER.abs_diff(loc);
    // let v3 = 0xa1100110au64 << KNIGHT_CENTER.abs_diff(loc);
    // knight_moves &= (v1 & v2) | (!v1 & v3);

    // // ボード全体の可動マスクを作成
    // let v1 = ((self.white >> loc) & 1).wrapping_neg();
    // let v2 = !self.white & (self.black | knight_moves);
    // let v3 = !self.black & (self.white | knight_moves);
    // knight_moves &= (v1 & v2) | (!v1 & v3);
    // vec![]

    // legal bishop moves
    fn generate_bishop_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // legal rook moves
    fn generate_rook_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // legal queen moves
    fn generate_queen_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // legal kings moves
    fn generate_king_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
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

    for _i in 0..=10 {

    maps.get_valid_moves(loc);

    }

    let ret = maps.get_valid_moves(loc);

    stdout().flush().unwrap();
    ret
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
