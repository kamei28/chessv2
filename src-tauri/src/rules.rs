use std::{i8, sync::{Arc, Mutex}};
use rayon::prelude::*;
use tauri::State;

use crate::{EnPassant, GameState};

/** メモ
 * マスは0~63
 * シフトは1~
 */ 

/** 各駒の合法手を作成する関数を追加 */
impl GameState {
    /** 駒の移動処理 */
    fn mvoe_piece(&mut self, from: u8, to: u8) {
        let from_mask = 1 << from;
        let to_mask = 1 << to;
        let mut board: &mut u64 = &mut self.error;

        // 駒別移動処理: ポーン
        if self.pawn & from_mask != 0 {
            if (EnPassant {     // アンパッサンコマドリ
                place: (to as i8 + if self.move_count & 1 == 0 { 8 } else { -8 }) as u8, 
                valid_turn: self.move_count
            }) == self.en_passant {
                self.white &= !(1u64 << to << 8);
                self.black &= !(1u64 << to >> 8 );
            } else if (from as i8 - to as i8).abs() == 16 {
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
        if (self.white >> from) & 1 != 0 {
            self.white &= !from_mask;
            self.white |= to_mask;
            self.black &= !to_mask;
        } else {
            self.black &= !from_mask;
            self.black |= to_mask;
            self.white &= !to_mask;
        }

        // 駒の移動処理
        *board &= !from_mask;
        *board |= to_mask;

        self.move_count += 1;
    }

    /** インデックスから駒の可動範囲を調べる */
    fn get_valid_moves(&self, loc: u8) -> Vec<u8> {
        let bit_mask = 1 << loc;
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
        let mut mask_b = (0b111 << (loc as u64 - 1)) & (0xff << (loc & !7));
        let piece_a = self.white | self.black;
        let mut mask_c = 0x101 << loc;

        // ret: 全体ボードを0x101でXOR後、mask: 0b111 & 0xffでANDして可動範囲を求める
        let (ret, mask): (u64, u64) = if self.white & 1 << loc != 0 {   // white piece
            if loc & !7 == 8 && piece_a & (mask_c << 8) == 0 {
                mask_b |= 1 << loc << 8;
            } else if self.move_count == self.en_passant.valid_turn && mask_b & (1u64 << self.en_passant.place >> 16) != 0 {
                mask_c |= 1u64 << self.en_passant.place >> 16;
            }
            (piece_a ^ mask_c << 8, mask_b << 8)

        } else {
            if loc & !7 == 48 && piece_a & (mask_c >> 16) == 0 {    // black piece
                mask_b |= 1 << loc >> 8;
            } else if self.move_count == self.en_passant.valid_turn && mask_b & (1u64 << self.en_passant.place << 16) != 0 {
                mask_c |= 1u64 << self.en_passant.place << 24;
            }

            (piece_a ^ mask_c >> 16, mask_b >> 8)
        };

        // println!("{mask:064b}\n{ret:064b}\n");

        ret_loc(ret & mask)
    }
    // fn generate_pawn_moves(&self, mut loc: u8) -> Vec<u8> {
    //     let cmp_color = self.white & (1 << loc) != 0;
    //     let board = self.white | self.black;
    //     let mut skip_mask: u64= 0;
    //     let basic_mask = if loc%8 == 0 { 0b110 }    // adjustment for the a and h files
    //         else if loc%8 == 7 { 0b011 }
    //         else { 0b111 };

    //     // skip_mask: 2マス、mask: 基本マス
    //     let (ret, mask) = if cmp_color {
    //         let bit_mask = 1 << (loc+8);    // white
    //         let mut ret_mask = 0b010;
            
    //         if loc/8 == 1 && board & bit_mask == 0 {
    //             skip_mask = 0x1000000 << loc%8;
    //             ret_mask = 0x202;   // 0x200 + 0b010
    //         }
    //         loc += 7;   // white, up 1 line -3bit//2
    //         (self.black ^ ret_mask << loc, basic_mask << loc)

    //     } else {
    //         let bit_mask = 1 << (loc-8);    // black
    //         if loc/8 == 6 && board & bit_mask == 0 {
    //             skip_mask = 0x100000000 << loc%8;
    //             (self.white ^ 0x202 << (loc - 17), 0b111 << (loc - 9))

    //         } else {
    //             if loc != 8 {
    //                 loc -= 9;   // black, down 1 line -3bit//2
    //                 (self.white ^ 0b010 << loc, basic_mask << loc)
    //             } else {
    //                 loc -= 8;   // black, down 1 line -3bit//2
    //                 (self.white ^ 0b01 << loc, 0b11 << loc)
    //             }
    //         }
    //     };

    //     // println!("{ret:064b}\n{:064b}", mask | skip_mask);

    //     // アンパッサン追加(retを1にする)
    //     // 2マス追加時にアンパッサン用変数用意

    //     ret_loc(ret & (mask | skip_mask))
    // }

    //knight
    fn generate_knight_moves(&self, loc: u8) -> Vec<u8> {
        // 可動領域
        let mut mask_b = if loc & 7 < 4 { 0x3f3f3f3f3f3f3f3f } else { 0xfcfcfcfcfcfcfcfc };

        // ベース可動範囲を適応
        mask_b &= if loc < 18 { 0xa1100110a >> (18 - loc) } else { 0xa1100110a << (loc - 18) };

        // 全体ボード
        let ret = if self.white & (1 << loc) != 0 {
            !self.white & (self.black | mask_b)
        } else {
            !self.black & (self.white | mask_b)
        };

        println!("knihgt");
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
    if maps.pawn & (1u64 << to) != 0 && (maps.en_passant.place as i8 - to as i8).abs() == 8 && maps.move_count - 1 == maps.en_passant.valid_turn {
        (maps.en_passant.place as i8 + if maps.en_passant.valid_turn & 1 != 0 { 16 } else { -16 }).try_into().unwrap()
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

// /** キング以外の合法手を求める(解析用) */
// fn get_valid_moves(board: &u64, piece: &u64) -> Vec<u8> {
//     let mut valid = !board & piece;
//     let mut ret = Vec::new();

//     while valid != 0 {
//         ret.push(valid.trailing_zeros() as u8);
//         valid &= valid - 1;
//     }
//     ret
// }

// /** 駒の可動範囲を求める */
// fn get_move_range(loc: &u8) {

// }

/** 1の場所を返す */
 fn ret_loc(mut board: u64) -> Vec<u8> {
    let mut ret = Vec::new();

    while board != 0 {
        ret.push(board.trailing_zeros() as u8);
        board &= board - 1;
    }
    ret
}
