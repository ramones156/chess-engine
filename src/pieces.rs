extern crate find_folder;

use std::path::Path;
use opengl_graphics::Texture;
use piston_window::TextureSettings;
use std::collections::HashMap;
use std::cmp::min;


const SIZE: usize = 64;
// top, bottom, left, right, top left, bottom right, top right, bottom left
const OFFSETS: [isize; 8] = [8, -8, -1, 1, 9, -9, 7, -7];

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

#[derive(Clone, Copy)]
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
    pub fn get_moves(&self, rank: usize, file: usize) -> Vec<isize> {
        let mut moves: Vec<isize> = vec![];

        let north = file;
        let south = 7 - file;
        let east = 7 - rank;
        let west = rank;
        // north, south, west, east, nw, se, ne, sw
        let direction_to_edge: [usize; 8] = [
            north,
            south,
            east,
            west,
            min(north, west),
            min(south, east),
            min(north, east),
            min(south, west)];

        match self.piece_type {
            PieceType::Knight => {}
            PieceType::King => moves = Vec::from(OFFSETS),
            PieceType::Queen => {
                moves = Piece::sliding_piecs_moves(0, 8, direction_to_edge);
            }
            PieceType::Rook => {
                moves = Piece::sliding_piecs_moves(0, 4, direction_to_edge);
            }
            PieceType::Bishop => {
                moves = Piece::sliding_piecs_moves(4, 8, direction_to_edge);
            }
            PieceType::Pawn => {}
            _ => {}
        };
        moves
    }
    fn sliding_piecs_moves(start: usize, end: usize, directions: [usize; 8]) -> Vec<isize> {
        let mut moves: Vec<isize> = vec![];
        let move_types = &OFFSETS[start..end];
        let directions = &directions[start..end];
        // iter through north,south,east,west
        for x in 0..end - start {
            // move a square until it hits the edge
            println!("amount to edge: {}", directions[x]);
            for y in 0..directions[x] {
                let m: isize = move_types[x] * (1 + y as isize);
                moves.push(m);
            }
        }
        moves
    }
}

impl Default for Piece {
    fn default() -> Self { Piece { piece_type: PieceType::EMPTY, piece_color: PieceColor::NEITHER } }
}

