extern crate find_folder;

use std::path::PathBuf;
use opengl_graphics::Texture;
use piston_window::TextureSettings;
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fmt, env};

const SIZE: usize = 64;

#[derive(PartialEq, Clone, Copy)]
pub enum PieceColor {
    BLACK,
    WHITE,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("type", &self.piece_type)
            .finish()
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Piece {
        Piece {
            piece_type,
            piece_color,
        }
    }
    pub fn get_icon(self) -> Texture {
        let path = match (self.piece_type, self.piece_color) {
            (PieceType::King, PieceColor::BLACK) => Piece::load_resources("king_black.png"),
            (PieceType::Queen, PieceColor::BLACK) => Piece::load_resources("queen_black.png"),
            (PieceType::Rook, PieceColor::BLACK) => Piece::load_resources("rook_black.png"),
            (PieceType::Bishop, PieceColor::BLACK) => Piece::load_resources("bishop_black.png"),
            (PieceType::Knight, PieceColor::BLACK) => Piece::load_resources("knight_black.png"),
            (PieceType::Pawn, PieceColor::BLACK) => Piece::load_resources("pawn_black.png"),
            (PieceType::King, PieceColor::WHITE) => Piece::load_resources("king_white.png"),
            (PieceType::Queen, PieceColor::WHITE) => Piece::load_resources("queen_white.png"),
            (PieceType::Rook, PieceColor::WHITE) => Piece::load_resources("rook_white.png"),
            (PieceType::Bishop, PieceColor::WHITE) => Piece::load_resources("bishop_white.png"),
            (PieceType::Knight, PieceColor::WHITE) => Piece::load_resources("knight_white.png"),
            (PieceType::Pawn, PieceColor::WHITE) => Piece::load_resources("pawn_white.png"),
        };

        Texture::from_path(path, &TextureSettings::new()).unwrap()
    }
    pub fn from_fen(fen: &str) -> [Option<Piece>;SIZE] {
        let dictionary: HashMap<char, Piece> = [
            ('k', Piece::new(PieceType::King, PieceColor::BLACK)),
            ('p', Piece::new(PieceType::Pawn, PieceColor::BLACK)),
            ('n', Piece::new(PieceType::Knight, PieceColor::BLACK)),
            ('b', Piece::new(PieceType::Bishop, PieceColor::BLACK)),
            ('r', Piece::new(PieceType::Rook, PieceColor::BLACK)),
            ('q', Piece::new(PieceType::Queen, PieceColor::BLACK)),
            ('K', Piece::new(PieceType::King, PieceColor::WHITE)),
            ('P', Piece::new(PieceType::Pawn, PieceColor::WHITE)),
            ('N', Piece::new(PieceType::Knight, PieceColor::WHITE)),
            ('B', Piece::new(PieceType::Bishop, PieceColor::WHITE)),
            ('R', Piece::new(PieceType::Rook, PieceColor::WHITE)),
            ('Q', Piece::new(PieceType::Queen, PieceColor::WHITE)),
        ].iter().cloned().collect();

        let mut pieces = [None; SIZE];
        let mut fen = fen.split(' ');
        let board = fen.next().unwrap();
        let mut rank: usize = 7;
        let mut file: usize = 0;
        for symbol in board.chars() {
            if symbol == '/' {
                file = 0;
                rank -= 1;
            } else if symbol.is_numeric() {
                file += symbol.to_digit(10).unwrap() as usize;
            } else if dictionary.contains_key(&symbol) {
                pieces[rank * 8 + file] = Some(*dictionary.get(&symbol).unwrap());
                file += 1;
            }
        }
        pieces
    }
    fn load_resources(filename:&str) -> PathBuf {
        // See <https://doc.rust-lang.org/stable/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
        let base_dir = option_env!("CARGO_MANIFEST_DIR").map_or_else(|| {
            let exe_path = env::current_exe().expect("Failed to get exe path");
            exe_path.parent().unwrap().to_path_buf()
        }, |crate_dir| {
            PathBuf::from(crate_dir)
        });
        base_dir.join("assets").join(filename)
    }
}


