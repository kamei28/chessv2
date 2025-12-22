use crate::engine::position::game_state::GameState;

impl GameState {
    #[inline(always)]
    pub fn generate_wpawn_moves(&self, loc: u8) -> u64 {

        // 前１マス目
        let v1 = self.white | self.black;
        let mut v2 = !v1 & (0x100 << loc);

        // 前2マス目
        v2 |= !v1 & (v2 << (((loc & !7 == 8) as u8) << 3));

        // アンパッサン
        let v3 = ((self.move_count%2 == 0) as u64) << self.en_passant;

        // 斜め前
        let g2 = loc & 7;
        let v4 = (((g2 != 7) as u64) << 9) | (((g2 != 0) as u64) << 7);
        
        v2 | (v4 << loc) & (self.black | v3)
    }
}
