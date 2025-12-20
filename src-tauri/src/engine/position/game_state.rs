/** 駒の種類ボード */
// use crate::piece_type::*;

#[derive(PartialEq, Eq, Debug, Default)]
/** アンパッサン構造体 */
pub struct EnPassant {
    pub place: u8, 
    pub valid_turn: Option<u32>
}

#[derive(Debug, Default)]
/** ゲームデータの管理 */
pub struct GameState {
    pub move_count: u32, 
    pub en_passant: EnPassant, 
    // pub piecet: PieceType, 
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
        self.en_passant = EnPassant { place: 0, valid_turn: None };
        // self.piecet = PieceType::default();
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
    // move_pieces
    // valid_moves
    // gen_moves::*
}
