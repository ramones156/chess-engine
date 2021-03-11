extern crate find_folder;

use std::path::Path;
use opengl_graphics::Texture;
use piston_window::TextureSettings;
use graphics::Image;

const SIZE: usize = 64;

#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    KingBlack,
    QueenBlack,
    RookBlack,
    BishopBlack,
    KnightBlack,
    PawnBlack,
    KingWhite,
    QueenWhite,
    RookWhite,
    BishopWhite,
    KnightWhite,
    PawnWhite,
    EMPTY,
}

#[derive( Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Piece {
        Piece {
            piece_type
        }
    }
    pub fn get_icon(&self) -> Texture {
        let path = match self.piece_type {
            PieceType::KingBlack => Path::new("assets/king_black.png"),
            PieceType::QueenBlack => Path::new("./assets/queen_black.png"),
            PieceType::RookBlack => Path::new("./assets/rooks_black.png"),
            PieceType::BishopBlack => Path::new("./assets/bishop_black.png"),
            PieceType::KnightBlack => Path::new("./assets/knight_black.png"),
            PieceType::PawnBlack => Path::new("./assets/pawn_black.png"),
            PieceType::KingWhite => Path::new("./assets/king_white.png"),
            PieceType::QueenWhite => Path::new("./assets/queen_white.png"),
            PieceType::RookWhite => Path::new("./assets/rook_white.png"),
            PieceType::BishopWhite => Path::new("./assets/bishop_white.png"),
            PieceType::KnightWhite => Path::new("./assets/knight_white.png"),
            PieceType::PawnWhite => Path::new("./assets/pawn_white.png"),
            PieceType::EMPTY => Path::new("")
        };

        Texture::from_path(path, &TextureSettings::new()).unwrap()
    }
    pub fn default_board() -> [Piece; 64] {
        let mut pieces = [Piece::default(); SIZE];
        pieces[0] = Piece::new(PieceType::KnightBlack);
        pieces[2] = Piece::new(PieceType::KnightWhite);
        pieces[4] = Piece::new(PieceType::PawnWhite);
        pieces[6] = Piece::new(PieceType::RookWhite);
        pieces[8] = Piece::new(PieceType::QueenBlack);
        pieces[10] = Piece::new(PieceType::QueenWhite);
        pieces[62] = Piece::new(PieceType::BishopBlack);
        pieces[31] = Piece::new(PieceType::BishopWhite);
        pieces
    }
}

impl Default for Piece {
    fn default() -> Self { Piece { piece_type: PieceType::EMPTY}}
}

