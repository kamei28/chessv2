use crate::engine::position::game_state::GameState;

impl GameState {
    // legal king moves
    #[inline(always)]
    pub fn generate_king_moves(&self, loc: u8) -> u64 {
        println!("king");

        0x0
    }
}
