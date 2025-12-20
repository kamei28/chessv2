use crate::engine::position::{
    game_state::{EnPassant, GameState}, 
    consts::*
};

impl GameState {
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
}
