
pub enum Color {
    BLACK,
    WHITE,
    NEITHER
}

pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN, // small pieces
    EMPTY
}

pub struct Piece {
    pub(crate) color: Color,
    pub(crate) piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece {
            color,
            piece_type,
        }
    }
}