use crate::engine::position::consts::*;

/** ゲームデータの管理 */
#[derive(Debug, Default)]
pub struct GameState {
    pub move_count: u32, 
    pub en_passant: u8, 
    pub white:  u64, 
    pub black:  u64, 
    pub pawn:   u64, 
    pub rook:   u64, 
    pub knight: u64, 
    pub bishop: u64, 
    pub queen:  u64, 
    pub king:   u64, 
    pub error:  u64
}

/** 駒処理を実装 */
impl GameState {
    /** ゲームデータをリセット */
    pub fn reset(&mut self) {
        self.move_count = 0;
        self.en_passant = 0;
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
    
    /** 駒の移動処理 */
    #[inline(always)]
    pub fn move_piece(&mut self, from: u8, to: u8) {
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

    /** アンパッサンの判定 */
    pub fn handle_en_passant(&mut self, from: u8, to: u8) {

        // 有効なら移動して削除
        if self.en_passant == to {
            self.white &= !(1u64 << to << RANK_SHIFT);
            self.black &= !(1u64 << to >> RANK_SHIFT);
            self.en_passant = 0;

        // 2マス飛びなら有効化
        } else if from.abs_diff(to) == 2 * RANK_SHIFT {
            self.en_passant = (from + to)/2;
        
        // 無効なら削除
        } else {
            self.en_passant = 0;
        }
    }
    
    /** インデックスから駒の可動範囲を調べる */
    #[inline(always)]
    pub fn get_valid_moves(&self, loc: u8) -> u64 {
        let bit_mask = 1u64 << loc;

        // ポーンだけ白黒で処理分け
        if self.pawn & bit_mask != 0 {
            if self.white & bit_mask != 0 {
                self.generate_wpawn_moves(loc)
            } else { 
                self.generate_bpawn_moves(loc)
            }
        }
        else if self.knight & bit_mask  != 0 { self.generate_knight_moves(loc)  }
        else if self.bishop & bit_mask  != 0 { self.generate_bishop_moves(loc)  }
        else if self.queen  & bit_mask  != 0 { self.generate_queen_moves(loc)   }
        else if self.rook   & bit_mask  != 0 { self.generate_rook_moves(loc)    }
        else if self.king   & bit_mask  != 0 { self.generate_king_moves(loc)    }
        else { 0x0 }
    }

    // gen_moves::*
}
