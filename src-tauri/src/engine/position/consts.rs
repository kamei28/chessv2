/** Constants used for rank */
pub const RANK_SHIFT:   u8 =  8;
pub const RANK_INDEX:   u8 =  8;
pub const RANK_UP:      i8 =  8;
pub const RANK_DOWN:    i8 = -8;

/** Constants used for piece move generation */
pub const PAWN_CENTER:  u8 =  1;
pub const SHIFT_BASE:   u32 = (PAWN_CENTER + 8) as u32;
pub const KNIGHT_CENTER:u8 = 18;
