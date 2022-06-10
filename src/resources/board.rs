use bevy::prelude::debug;
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use crate::resources::piece::{
    Direction, Piece, Position, DIRECTION_OFFSETS, KNIGHT_DIRECTION_OFFSETS,
};
use crate::resources::piece_type::*;

#[derive(Clone, Copy)]
pub struct BoardMap {
    squares: [[Piece; 8]; 8],
    active_color: bool, // white is false, black is true
}

impl Default for BoardMap {
    fn default() -> Self {
        let squares = [[Piece(0); 8]; 8];

        Self {
            squares,
            active_color: false,
        }
    }
}

impl BoardMap {
    /// starting position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// white is uppercase, black is lowercase
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::default();
        let sections = fen.split_whitespace().collect::<Vec<_>>();
        let placement = sections[0].split('/').collect::<Vec<_>>();

        let mut index = 0;
        placement.iter().for_each(|x| {
            for mut x in x.chars() {
                let color = if x.is_uppercase() { WHITE } else { BLACK };

                if !x.is_numeric() {
                    x.make_ascii_lowercase();
                    let rank = match x {
                        'p' => PAWN,
                        'r' => ROOK,
                        'b' => BISHOP,
                        'q' => QUEEN,
                        'k' => KING,
                        'n' => KNIGHT,
                        _ => 0,
                    };
                    board.squares[7 - (index / 8)][index % 8] = Piece(color | rank);
                    index += 1;
                } else {
                    index += x.to_digit(10).unwrap() as usize;
                }
            }
        });
        board.active_color = if let Some(c) = sections[1].chars().next() {
            match c {
                'w' => false,
                'b' => true,
                _ => unreachable!("FEN incorrect"),
            }
        } else {
            false
        };

        board
    }
    pub fn get_fen(&self) -> String {
        todo!()
    }
    pub fn get_piece(&self, on: Position) -> Piece {
        self.squares[on[0]][on[1]]
    }
    pub fn get_active_color(&self) -> bool {
        self.active_color
    }
    fn set_piece(&mut self, _on: Position, value: u32) {
        self.squares[_on[0]][_on[1]] = Piece(value);
    }

    /// make a move with check
    pub fn move_turn(&mut self, on: [usize; 2], to: [usize; 2]) -> bool {
        let (x, y) = (on[0], on[1]);
        let piece = self.squares[x][y];
        if !piece.is_piece() {
            println!("You're trying to move a piece that's empty");
            return false;
        }
        if (piece.is_white() && !self.active_color) || (piece.is_black() && self.active_color) {
            if self.is_valid_move(on, to) {
                self.set_piece([x, y], if (x + y) % 2 != 0 { BLACK } else { WHITE });
                let (x, y) = (to[0], to[1]);
                self.set_piece([x, y], piece.0);
                self.active_color = !self.active_color;
                return true;
            } else {
                println!("Move was invalid");
            }
        } else {
            println!("Piece is not yours");
        }
        false
    }
    /// check if move is valid
    pub fn is_valid_move(&self, from: Position, to: Position) -> bool {
        let piece_to = self.squares[to[0]][to[1]];

        if !piece_to.is_piece() || piece_to.get_color() != self.active_color {
            let moves = self.gen_legal_moves(from);
            return moves.contains(&to);
        }
        false
    }
    /// generate only legal moves for piece
    pub fn gen_legal_moves(&self, from: Position) -> Vec<Position> {
        let mut temp_board = *self;
        let moves = temp_board.gen_moves(from);
        let mut legal_moves = vec![];
        debug!("moves {:?}", &moves);

        for to in moves.into_iter() {
            let last_piece = temp_board.squares[to[0]][to[1]].0;
            temp_board.make_move(from, to);
            let next_moves = temp_board.gen_opponent_moves();
            // println!("next possible moves: {:?}", next_moves);
            if !next_moves.iter().any(|x| {
                let next_piece = temp_board.squares[x[0]][x[1]];
                if next_piece.is_piece() && next_piece.get_color() == temp_board.active_color {
                    // println!("{:?}", next_piece);
                    if let Some(t) = next_piece.get_type() {
                        return t == PieceType::King;
                    }
                }
                false
            }) {
                legal_moves.push(to);
            }

            temp_board.undo_move(from, to, last_piece);
        }
        debug!("legal moves {:?}", &legal_moves);
        legal_moves
    }
    /// generate all possible moves for piece
    pub fn gen_moves(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        if let Some(piece_type) = piece_from.get_type() {
            return match piece_type {
                PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                    self.gen_sliding(from, piece_type)
                }
                PieceType::Pawn => self.gen_pawn(from),
                PieceType::King => self.gen_king(from),
                PieceType::Knight => self.gen_knight(from),
            };
        }
        vec![]
    }
    fn gen_sliding(&self, from: Position, piece_type: PieceType) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        let start = if piece_type == PieceType::Bishop {
            4
        } else {
            0
        };
        let end = if piece_type == PieceType::Rook { 4 } else { 8 };
        for (direction, offset) in DIRECTION_OFFSETS.iter().enumerate().take(end).skip(start) {
            for n in 0..self.len_to_edge(from, Direction::from(direction)) {
                let index = from[0] * 8 + from[1];
                let target_index = (index as i32 + offset * (n + 1) as i32).clamp(0, 63) as usize;
                let target_move = [target_index / 8, target_index % 8];
                let target_piece = self.squares[target_move[0]][target_move[1]];

                if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                    // your own color is in the way
                    // println!("Piece is yours!");
                    break;
                }
                moves.push(target_move);
                // self.squares[target_move[0]][target_move[1]] = Piece(100);

                if target_piece.is_piece() && target_piece.get_color() != piece_from.get_color() {
                    // Enemy piece and capturable
                    // println!("Piece is not yours, but you should still break the loop!");
                    break;
                }
            }
        }
        moves
    }
    fn gen_king(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        for direction in DIRECTION_OFFSETS {
            let index = from[0] * 8 + from[1];
            let target_index = index as i32 + direction;
            if !(0..=63).contains(&target_index) {
                continue;
            }
            let target_move = [target_index as usize / 8, target_index as usize % 8];
            let target_piece = self.squares[target_move[0]][target_move[1]];

            if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                // your own color is in the way
                // println!("Piece is yours!");
                continue;
            }
            moves.push(target_move);
            // self.squares[target_move[0]][target_move[1]] = Piece(100);

            if target_piece.is_piece() && target_piece.get_color() != piece_from.get_color() {
                // Enemy piece and capturable
                // println!("Piece is not yours, but you should still break the loop!");
                continue;
            }
        }
        moves
    }
    fn gen_pawn(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        let shift = if piece_from.get_color() { -1 } else { 1 };

        // piece blocking
        let is_blocking = self.squares[(from[0] as i32 + shift) as usize][from[1]].is_piece();
        if !is_blocking {
            moves.push([(from[0] as i32 + shift) as usize, from[1]]);
        }

        // hasn't moved yet
        if (piece_from.is_black() && from[0] == 6 || piece_from.is_white() && from[0] == 1)
            && !is_blocking
        {
            moves.push([(from[0] as i32 + (shift * 2)) as usize, from[1]]);
        }

        if (1..8).contains(&from[1]) {
            let to_left_pos = [(from[0] as i32 + shift) as usize, from[1] - 1];
            let to_left = self.squares[to_left_pos[0]][to_left_pos[1]];
            if to_left.is_piece() && to_left.get_color() != piece_from.get_color() {
                moves.push(to_left_pos);
            }
        }

        if (0..7).contains(&from[1]) {
            let to_right_pos = [(from[0] as i32 + shift) as usize, from[1] + 1];
            let to_right = self.squares[to_right_pos[0]][to_right_pos[1]];
            if to_right.is_piece() && to_right.get_color() != piece_from.get_color() {
                moves.push(to_right_pos);
            }
        }
        moves
    }
    fn gen_knight(&self, from: Position) -> Vec<Position> {
        let piece_from = self.squares[from[0]][from[1]];
        let mut moves = vec![];
        for direction in KNIGHT_DIRECTION_OFFSETS {
            let new_pos = [direction[0] + from[0] as i32, direction[1] + from[1] as i32];
            if (0..8).contains(&new_pos[0]) && (0..8).contains(&new_pos[1]) {
                let to_move = [new_pos[0] as usize, new_pos[1] as usize];
                let target_piece = self.squares[to_move[0]][to_move[1]];
                if target_piece.is_piece() && target_piece.get_color() == piece_from.get_color() {
                    continue;
                }

                moves.push(to_move);
            }
        }

        moves
    }
    fn len_to_edge(&self, pos: Position, direction: Direction) -> usize {
        let (rank, file) = (pos[0], pos[1]);
        let north = 7 - rank;
        let south = rank;
        let west = file;
        let east = 7 - file;

        match direction {
            Direction::North => north,
            Direction::NorthEast => min(north, east),
            Direction::East => east,
            Direction::SouthEast => min(south, east),
            Direction::South => south,
            Direction::SouthWest => min(south, west),
            Direction::West => west,
            Direction::NorthWest => min(north, west),
        }
    }
    /// make a move (without check)
    fn make_move(&mut self, from: Position, to: Position) {
        self.set_piece(to, self.get_piece(from).0);
        self.set_piece(
            from,
            if (from[0] + from[1]) % 2 != 0 {
                BLACK
            } else {
                WHITE
            },
        );
    }
    fn undo_move(&mut self, from: Position, to: Position, last_piece: u32) {
        self.set_piece(from, self.get_piece(to).0);
        self.set_piece(to, last_piece);
    }
    fn gen_opponent_moves(&self) -> Vec<Position> {
        let mut opponent_moves = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let piece = self.squares[rank][file];
                if piece.is_piece() && piece.get_color() != self.active_color {
                    // println!("found enemy piece! {:?}", piece);
                    let moves = self.gen_moves([rank, file]);
                    opponent_moves.extend(moves);
                }
            }
        }
        opponent_moves
    }
}

impl Debug for BoardMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..8 {
            writeln!(f, "{} {:?}", 8 - x, self.squares[7 - x]).unwrap();
        }
        writeln!(f, "   a   b   c   d   e   f   g   h").unwrap();
        if self.active_color {
            writeln!(f, "black's turn").unwrap();
        } else {
            writeln!(f, "white's turn").unwrap();
        }
        Ok(())
    }
}

impl Deref for BoardMap {
    type Target = [[Piece; 8]; 8];

    fn deref(&self) -> &Self::Target {
        &self.squares
    }
}

impl DerefMut for BoardMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.squares
    }
}
