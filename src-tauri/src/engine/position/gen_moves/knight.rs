use crate::engine::position::game_state::GameState;
use crate::engine::position::consts::*;

impl GameState {
    #[inline(always)]
    pub fn generate_knight_moves(&self, loc: u8) -> u64 {
        
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

        knight_moves
    }
}
