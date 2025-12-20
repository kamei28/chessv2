use crate::engine::position::game_state::GameState;

impl GameState {
    // legal bishop moves
    #[inline(always)]
    pub fn generate_bishop_moves(&self, loc: u8) -> u64 {
        println!("bishop");
        
        0x0
    }
}
