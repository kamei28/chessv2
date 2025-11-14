use std::{i8, sync::{Arc, Mutex}};
use rayon::prelude::*;
use tauri::State;

use crate::{EnPassant, GameState};

const RANK_SHIFT    : u8 =  8;
const RANK_NUMBER   : u8 =  8;
const RANK_UP       : i8 =  8;
const RANK_DOWN     : i8 = -8;
const FILE_SHIFT    : u8 =  1;

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
            if (EnPassant {     // 移動先のアンパッサン判定
                place: ((to as i8) + if self.move_count & 0b1 == 0 { RANK_UP } else { RANK_DOWN }) as u8, 
                valid_turn: self.move_count
            }) == self.en_passant {
                self.white &= !(1u64 << to << RANK_SHIFT);  // アンパッサンコマドリ
                self.black &= !(1u64 << to >> RANK_SHIFT);

            } else if from.abs_diff(to) == 2*RANK_SHIFT {     // 2マス飛びならアンパッサン登録
                self.en_passant = EnPassant { place: from, valid_turn: self.move_count + 1 };
            }
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
        if self.rook   & from_mask != 0 {
            board = &mut self.rook;
        } else {
            self.rook &= !to_mask;
        }

        // 駒別移動処理: クイーン
        if self.queen  & from_mask != 0 {
            board = &mut self.queen;
        } else {
            self.queen &= !to_mask;
        }

        // 駒別移動処理: キング
        if self.king   & from_mask != 0 {
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

    /** インデックスから駒の可動範囲を調べる */
    fn get_valid_moves(&self, loc: u8) -> Vec<u8> {
        let bit_mask = 1u64 << loc;
        if self.pawn        & bit_mask != 0 { self.generate_pawn_moves(loc)    } 
        else if self.knight & bit_mask != 0 { self.generate_knight_moves(loc)  } 
        else if self.bishop & bit_mask != 0 { self.generate_bishop_moves(loc)  } 
        else if self.rook   & bit_mask != 0 { self.generate_rook_moves(loc)    } 
        else if self.queen  & bit_mask != 0 { self.generate_queen_moves(loc)   } 
        else if self.king   & bit_mask != 0 { self.generate_king_moves(loc)    } 
        else { vec![] }
    }

    // pawn
    fn generate_pawn_moves(&self, loc: u8) -> Vec<u8> {
        let mut mask_b = (0b111u64 << (loc - FILE_SHIFT)) & (0xffu64 << (loc & !7));
        let mut mask_c = 0x101u64 << loc;
        let piece_a = self.white | self.black;

        // ret: 全体ボードを0x101でXOR後、mask: 0b111 & 0xffでANDして可動範囲を求める
        let (ret, mask): (u64, u64) = if self.white & 0b1 << loc != 0 {   // white piece
            if loc & !7 == RANK_NUMBER && piece_a & (mask_c << RANK_SHIFT) == 0 {
                mask_b |= 1u64 << loc << RANK_SHIFT;
            } else if self.move_count == self.en_passant.valid_turn && mask_b & (1u64 << self.en_passant.place >> 2*RANK_SHIFT) != 0 {
                mask_c |= 1u64 << self.en_passant.place >> 2*RANK_SHIFT;
            }
            (piece_a ^ mask_c << RANK_SHIFT, mask_b << RANK_SHIFT)

        } else {
            if loc & !7 == 6*RANK_NUMBER && piece_a & (mask_c >> 2*RANK_SHIFT) == 0 {    // black piece
                mask_b |= 1u64 << loc >> RANK_SHIFT;
            } else if self.move_count == self.en_passant.valid_turn && mask_b & (1u64 << self.en_passant.place << 2*RANK_SHIFT) != 0 {
                mask_c |= 1u64 << self.en_passant.place << 3*RANK_SHIFT;
            }

            (piece_a ^ mask_c >> 2*RANK_SHIFT, mask_b >> RANK_SHIFT)
        };

        // println!("{mask:064b}\n{ret:064b}\n");

        ret_loc(ret & mask)
    }

    //knight
    fn generate_knight_moves(&self, loc: u8) -> Vec<u8> {
        // 可動領域
        // 0x3f3f3f3f3f3f3f3f: 左2列除外
        // 0xfcfcfcfcfcfcfcfc: 右2列除外
        let mut mask_b = if loc & 7 < 4 { 0x3f3f3f3f3f3f3f3fu64 } else { 0xfcfcfcfcfcfcfcfcu64 };

        // ベース可動範囲を適応
        // 18: 位置調整の境界値
        mask_b &= if loc < 18 { 0xa1100110a >> (18 - loc) } else { 0xa1100110a << (loc - 18) };

        // 全体ボード
        let ret = if self.white & (1u64 << loc) != 0 {
            !self.white & (self.black | mask_b)
        } else {
            !self.black & (self.white | mask_b)
        };

        ret_loc(ret & mask_b)
    }

    // bishop
    fn generate_bishop_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // rook
    fn generate_rook_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // queen
    fn generate_queen_moves(&self, loc: u8) -> Vec<u8> {
        vec![]
    }

    // kings
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
    
    maps.get_valid_moves(loc)
}

/** 駒の移動処理(アプリ用) */
#[tauri::command]
pub fn mvoe_piece(from: u8, to: u8, state: State<Arc<Mutex<GameState>>>) -> u8 {
    let mut maps = state.lock().unwrap();
    
    println!("move from {from} to {to}");
    maps.mvoe_piece(from, to);

    // 削除する駒の位置を返す
    if maps.pawn & (1u64 << to) != 0 && maps.en_passant.place.abs_diff(to) == RANK_NUMBER && maps.en_passant.valid_turn == (maps.move_count - 1) {
        let rank_shift = if maps.en_passant.valid_turn & 0b1 != 0 { RANK_UP } else { RANK_DOWN };
        ((maps.en_passant.place as i8) + 2*rank_shift).try_into().unwrap()
    } else {
        0
    }
}

/** debag */
#[tauri::command]
pub fn test(loc: u8, state: State<Arc<Mutex<GameState>>>) -> Vec<u8> {
    let maps = state.lock().unwrap();
    
    maps.generate_pawn_moves(loc)
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
