extern crate find_folder;

use std::path::Path;
use opengl_graphics::Texture;
use piston_window::TextureSettings;
use std::collections::HashMap;
use std::cmp::min;


const SIZE: usize = 64;


#[derive(PartialEq, Clone, Copy)]
pub enum PieceColor {
    BLACK,
    WHITE,
    NEITHER,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    EMPTY,
}

#[derive(Clone, Copy,PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Piece {
        Piece {
            piece_type,
            piece_color,
        }
    }
    pub fn get_icon(&self) -> Texture {
        let path = match (self.piece_type, self.piece_color) {
            (PieceType::King, PieceColor::BLACK) => Path::new("assets/king_black.png"),
            (PieceType::Queen, PieceColor::BLACK) => Path::new("./assets/queen_black.png"),
            (PieceType::Rook, PieceColor::BLACK) => Path::new("./assets/rook_black.png"),
            (PieceType::Bishop, PieceColor::BLACK) => Path::new("./assets/bishop_black.png"),
            (PieceType::Knight, PieceColor::BLACK) => Path::new("./assets/knight_black.png"),
            (PieceType::Pawn, PieceColor::BLACK) => Path::new("./assets/pawn_black.png"),
            (PieceType::King, PieceColor::WHITE) => Path::new("./assets/king_white.png"),
            (PieceType::Queen, PieceColor::WHITE) => Path::new("./assets/queen_white.png"),
            (PieceType::Rook, PieceColor::WHITE) => Path::new("./assets/rook_white.png"),
            (PieceType::Bishop, PieceColor::WHITE) => Path::new("./assets/bishop_white.png"),
            (PieceType::Knight, PieceColor::WHITE) => Path::new("./assets/knight_white.png"),
            (PieceType::Pawn, PieceColor::WHITE) => Path::new("./assets/pawn_white.png"),
            _ => Path::new("")
        };

        Texture::from_path(path, &TextureSettings::new()).unwrap()
    }
    pub fn default_board() -> [Piece; 64] {
        Piece::load_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn load_from_fen(fen: &str) -> [Piece; SIZE] {
        let dictionary: HashMap<char, Piece> = [
            ('k', Piece::new(PieceType::King, PieceColor::WHITE)),
            ('p', Piece::new(PieceType::Pawn, PieceColor::WHITE)),
            ('n', Piece::new(PieceType::Knight, PieceColor::WHITE)),
            ('b', Piece::new(PieceType::Bishop, PieceColor::WHITE)),
            ('r', Piece::new(PieceType::Rook, PieceColor::WHITE)),
            ('q', Piece::new(PieceType::Queen, PieceColor::WHITE)),
            ('K', Piece::new(PieceType::King, PieceColor::BLACK)),
            ('P', Piece::new(PieceType::Pawn, PieceColor::BLACK)),
            ('N', Piece::new(PieceType::Knight, PieceColor::BLACK)),
            ('B', Piece::new(PieceType::Bishop, PieceColor::BLACK)),
            ('R', Piece::new(PieceType::Rook, PieceColor::BLACK)),
            ('Q', Piece::new(PieceType::Queen, PieceColor::BLACK)),
        ].iter().cloned().collect();

        let mut pieces = [Piece::default(); SIZE];
        let board = fen.split(' ').next().unwrap();

        let mut i: usize = 64;
        for symbol in board.chars() {
            if symbol.is_numeric() {
                i -= symbol.to_digit(10).unwrap() as usize;
            } else if dictionary.contains_key(&symbol) {
                i -= 1;
                pieces[i] = *dictionary.get(&symbol).unwrap();
            }
        }
        pieces
    }
}

impl Default for Piece {
    fn default() -> Self { Piece { piece_type: PieceType::EMPTY, piece_color: PieceColor::NEITHER } }
}

