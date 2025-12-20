use crate::engine::position::game_state::GameState;

impl GameState {
    #[inline(always)]
    pub fn generate_pawn_moves(&self, loc: u8) -> u64 {

        // ベース定義
        let g1 = (self.black >> loc & 1) as u8;
        let g3 = loc & !7;
        let g2 = (!g1 & (g3 == 8) as u8) | (g1 & (g3 == 48) as u8);
        let g4 = g1 << 4;

        // 前方確認
        let mut v1 = self.white | self.black;
        let mut v2 = 0x100 << loc >> g4;

        // 2マス
        let v3 = g2 * ((v1 ^ v2) & v2 != 0) as u8;
        v2 |= 0x10001*(v3 as u64) << loc >> g4;
        v1 ^= v2;
        v1 &= v2;

        // 斜め前確認
        let mut v2 = if g1 != 0 { self.white } else { self.black };
        v2 &= 0x280 << loc >> g4;

        // 合法手作成
        (v1 | v2) & !(0xff << g3)
    }
}
