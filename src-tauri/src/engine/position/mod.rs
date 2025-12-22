pub mod consts;
pub mod game_state;

pub mod gen_moves {
    pub mod wpawn;
    pub mod bpawn;
    pub mod rook;
    pub mod knight;
    pub mod bishop;
    pub mod queen;
    pub mod king;
}

pub use game_state::*;
