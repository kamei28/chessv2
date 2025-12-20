/** 駒の初期配置 */
const START_POSITION: [Piece; 64] = [
    // 8段目
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
    Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
    // 7段目
    Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
    Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
    // 6〜3段目
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    Piece::None, Piece::None, Piece::None, Piece::None,
    // 2段目
    Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
    Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
    // 1段目
    Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen,
    Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/** 駒の種類を定義 */
pub enum Piece { WPawn, BPawn, Knight, Bishop, Rook, Queen, King, None }

#[derive(Debug, Clone, Copy)]
/** 判別用ボードの作成 */
pub struct PieceType([Piece; 64]);

/** defaultの手動設定 */
impl Default for PieceType {
    fn default() -> Self {
        PieceType(START_POSITION)
    }
}
