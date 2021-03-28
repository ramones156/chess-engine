use graphics::grid::Grid;
use crate::pieces::{Piece, PieceType, PieceColor};
use graphics::*;
use graphics::rectangle::square;
use std::cmp::min;
use self::piston_window::{RenderArgs, GenericEvent, Button, MouseButton, OpenGL};
use opengl_graphics::GlGraphics;
use std::ops::Sub;

extern crate piston_window;

const SIZE: usize = 64;
// top, bottom, left, right, top left, bottom right, top right, bottom left
const OFFSETS: [isize; 8] = [8, -8, -1, 1, 9, -9, 7, -7];

pub struct Board {
    // OpenGL drawing backend.
    pub gl: GlGraphics,
    pub grid: Grid,
    pub pieces: [Option<Piece>; SIZE],
    pub moving_color: PieceColor,
    pub selected: Option<usize>,
    pub released: Option<usize>,
    pub selected_piece: Option<Piece>,
    pub cursor_pos: [f64; 2],
}

impl Board {
    pub fn new(fen: &str) -> Board {
        Board {
            gl: GlGraphics::new(OpenGL::V3_2),
            grid: Grid {
                cols: 8,
                rows: 8,
                units: 50.0,
            },
            pieces: Piece::from_fen(fen),
            moving_color: PieceColor::WHITE,
            selected: None,
            released: None,
            selected_piece: None,
            cursor_pos: [0.0, 0.0],
        }
    }
    pub fn default_board() -> Self {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let pieces = self.pieces;

        for i in 0..64 {
            let (rank, file) = Board::index_to_coords(i);
            let piece: Option<Piece> = pieces[i];
            self.draw_square(args, piece, rank, file)
        }

        if let Some(i) = self.selected {
            let (rank, file) = Board::index_to_coords(i);
            self.draw_square(args, None, rank, file);
            self.draw_move(rank, file, args);
        }
        if let Some(i) = self.released {
            if self.selected_piece != None && i < 64 {
                // original location
                let orig = self.selected.unwrap();
                let (orig_x, orig_y) = Board::index_to_coords(orig);
                let (rank, file) = Board::index_to_coords(i);
                let moves = self.get_moves(self.selected_piece.unwrap(), orig_x, orig_y);
                let translation = &(i as isize - orig as isize);
                // println!("trans: {}", translation);
                let target: Option<Piece> = self.pieces[i];
                if !moves.contains(translation) {
                    println!("Move doesn't exist");
                    self.pieces[orig] = self.selected_piece;
                } else {
                    if target == None || (target.unwrap().piece_color != self.selected_piece.unwrap().piece_color) {
                        self.draw_square(args, self.selected_piece, rank, file);
                        // Input piece and remove from original
                        self.pieces[orig] = None;
                        self.pieces[i] = self.selected_piece;
                    }
                }
            }
            // empty selection and release
            self.selected_piece = None;
            self.selected = None;
            self.released = None;
        }
    }
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let (x, y) = Board::calculate_coords(self.cursor_pos[0], self.cursor_pos[1]);
            // println!("Cell picked: {},{}", x - 1, y - 1);
            let i = Board::coords_to_index(x - 1, y - 1);
            // println!("index selected: {}", i);
            let piece: Option<Piece> = self.pieces[i];
            // println!("Piece: {}", i);
            if piece != None {
                // println!("Piece found!");
                self.selected = Some(i);
                self.selected_piece = self.pieces[i];
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            let (x, y) = Board::calculate_coords(self.cursor_pos[0], self.cursor_pos[1]);
            // println!("Cell released: {},{}", x-1,y-1);
            let i = Board::coords_to_index(x - 1, y - 1);
            // println!("index released: {}", i);
            if self.selected_piece != None {
                self.released = Some(i);
            }
        }
    }
    fn calculate_coords(x: f64, y: f64) -> (usize, usize) {
        // Compute the cell position.
        let cell_x = 1 + (x / 50.0) as usize;
        let cell_y = 8 - (y / 50.0) as usize;
        // println!("Cursor pos: {},{}", x, y);
        (cell_x, cell_y)
    }
    fn draw_square(&mut self, args: &RenderArgs, piece: Option<Piece>, rank: usize, file: usize) {
        let checker_square: [f64; 4] = rectangle::square(0.0, 0.0, 50.0);
        let white: [f32; 4] = color::hex("F0D9B5");
        let black: [f32; 4] = color::hex("946f51");
        let (x, y) = ((rank * 50) as f64, ((7 - file) * 50) as f64);
        let color_state = if (rank + file) % 2 == 0 { black } else { white };
        match piece {
            None => {
                self.gl.draw(args.viewport(), |c, gl| {
                    rectangle(color_state, checker_square, c.transform.trans(x, y), gl);
                });
            }
            _ => {
                let image = Image::new().rect(square(0.0, 0.0, 50.0));
                let texture = piece.unwrap().get_icon();
                self.gl.draw(args.viewport(), |c, gl| {
                    rectangle(color_state, checker_square, c.transform.trans(x, y), gl);
                    image.draw(&texture, &c.draw_state, c.transform.trans(x, y), gl);
                });
            }
        }
    }
    fn draw_move(&mut self, rank: usize, file: usize, args: &RenderArgs) {
        // println!("rank: {}, file: {}", rank, file);
        let checker_small: [f64; 4] = rectangle::square(15.0, 15.0, 20.0);
        let checker: [f64; 4] = rectangle::square(0.0, 0.0, 50.0);
        let red = [199.0, 0.0, 0.0, 0.5];
        let moves = self.get_moves(self.selected_piece.unwrap(), rank, file);
        let pieces = self.pieces;
//TODO testcases
        for m in moves.iter() {
            let index = (Board::coords_to_index(rank, file)).wrapping_add(*m as usize);
            let (x, y) = Board::index_to_coords(index);
            // println!("x: {}, y:{}, i:{}", x, y, index);
            self.gl.draw(args.viewport(), |c, gl| {
                // rectangle(red, checker_square, c.transform.trans(x, y), gl);
                if pieces[index] == None {
                    ellipse(red, checker_small, c.transform.trans(x as f64 * 50.0, (7 - y) as f64 * 50.0), gl);
                } else {
                    rectangle(red, checker, c.transform.trans(x as f64 * 50.0, (7 - y) as f64 * 50.0), gl);
                }
            });
        }
    }
    fn get_moves(&self, piece: Piece, rank: usize, file: usize) -> Vec<isize> {
        let mut moves: Vec<isize> = vec![];

        let north = 7 - file;
        let south = file;
        let east = rank;
        let west = 7 - rank;
        // north, south, east, west, nw, se, ne, sw
        let direction_to_edge: [usize; 8] = [
            north,
            south,
            east,
            west,
            min(north, west),
            min(south, east),
            min(north, east),
            min(south, west),
        ];
        match piece.piece_type {
            PieceType::Knight => {
                let offsets: [[isize; 2]; 8] = [[-2, 1], [-1, 2], [2, -1], [1, -2], [1, 2], [2, 1], [-1, -2], [-2, -1]];
                for d in 0..8 {
                    let (x, y) = (((rank as isize) + offsets[d][0]) as usize, ((file as isize) + offsets[d][1]) as usize);
                    if x < 8 && y < 8 {
                        let i = Board::coords_to_index(x, y);
                        let target = self.pieces[i];
                        if i < 64 && target == None {
                            moves.push((offsets[d][0] + offsets[d][1] * 8));
                        }
                    }
                }
            }
            PieceType::King => moves = Vec::from(&OFFSETS[0..8]),
            PieceType::Queen => moves = self.sliding_piecs_moves(&direction_to_edge[0..8], &OFFSETS[0..8]),
            PieceType::Rook => moves = self.sliding_piecs_moves(&direction_to_edge[0..4], &OFFSETS[0..8]),
            PieceType::Bishop => moves = self.sliding_piecs_moves(&direction_to_edge[4..8], &OFFSETS[4..8]),
            PieceType::Pawn => {
                match piece.piece_color {
                    PieceColor::WHITE => moves = Vec::from(&OFFSETS[0..1]),
                    PieceColor::BLACK => moves = Vec::from(&OFFSETS[1..2]),
                }
            }
        };
        moves
    }
    fn sliding_piecs_moves(&self, edge: &[usize], offsets: &[isize]) -> Vec<isize> {
        let mut moves: Vec<isize> = vec![];
        let directions = &edge;
        let pieces = self.pieces;
        // println!("r:{},f:{}", rank, file);
        // iter through north,south,east,west
        for x in 0..directions.len() {
            // move a square until it hits the edge
            // println!("amount to edge: {}", directions[x]);
            for y in 0..directions[x] {
                let m: isize = offsets[x] * (1 + y as isize);
                // println!("move: {}, index: {}", m, index);
                let p: Option<Piece> = pieces[self.selected.unwrap().wrapping_add(m as usize)];
                if p == None {
                    moves.push(m);
                } else if p.unwrap().piece_color == self.selected_piece.unwrap().piece_color {
                    break;
                } else if p.unwrap().piece_color != self.selected_piece.unwrap().piece_color {
                    moves.push(m);
                    break;
                }
            }
        }
        moves
    }
    fn index_to_coords(index: usize) -> (usize, usize) {
        // x: 0, 1, 2, 3, 4, 5, 6, 7, 0
        // y: 0, 0, 0, 0, 0, 0, 0, 0, 1
        (index % 8, index / 8)
    }
    fn coords_to_index(rank: usize, file: usize) -> usize {
        // i: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, .. 63
        rank + file * 8
    }
}

#[cfg(test)]
mod convert_tests {
    use crate::board::Board;

    #[test]
    fn to_coords_test() {
        assert_eq!((0, 0), Board::index_to_coords(0));
        assert_eq!((0, 1), Board::index_to_coords(8));
        assert_eq!((1, 1), Board::index_to_coords(9));
        assert_eq!((7, 7), Board::index_to_coords(63));
    }

    #[test]
    fn to_index_test() {
        assert_eq!(0, Board::coords_to_index(0, 0));
        assert_eq!(8, Board::coords_to_index(0, 1));
        assert_eq!(9, Board::coords_to_index(1, 1));
        assert_eq!(63, Board::coords_to_index(7, 7));
    }
}

#[cfg(test)]
mod load_tests {
    use crate::board::{Piece, PieceType, PieceColor};

    #[test]
    fn no_pieces() {
        let pieces = Piece::from_fen("8/8/8/8/8/8/8/8");
        for &piece in pieces.iter() {
            assert_eq!(None, piece);
        }
    }

    #[test]
    fn one_piece() {
        let pieces = Piece::from_fen("8/8/7q/8/8/8/8/8");
        let piece = pieces[47];

        assert_ne!(None, piece);
        assert_eq!(PieceType::Queen, piece.unwrap().piece_type)
    }

    #[test]
    fn default_board() {
        let pieces = Piece::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(Piece { piece_type: PieceType::Rook, piece_color: PieceColor::WHITE }, pieces[0].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Knight, piece_color: PieceColor::WHITE }, pieces[1].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Queen, piece_color: PieceColor::WHITE }, pieces[3].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Pawn, piece_color: PieceColor::WHITE }, pieces[8].unwrap());
        assert_eq!(None, pieces[16]);
        assert_eq!(Piece { piece_type: PieceType::Pawn, piece_color: PieceColor::BLACK }, pieces[48].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Rook, piece_color: PieceColor::BLACK }, pieces[56].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Queen, piece_color: PieceColor::BLACK }, pieces[59].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Knight, piece_color: PieceColor::BLACK }, pieces[62].unwrap());
        assert_eq!(Piece { piece_type: PieceType::Rook, piece_color: PieceColor::BLACK }, pieces[63].unwrap());
    }
}