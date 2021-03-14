use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs};
use graphics::grid::Grid;
use crate::pieces::{Piece, PieceType, PieceColor};
use graphics::*;
use graphics::rectangle::square;
use piston::{GenericEvent, Button, MouseButton};
use std::cmp::min;

extern crate piston_window;

const SIZE: usize = 64;
// top, bottom, left, right, top left, bottom right, top right, bottom left + 8 knight moves
const OFFSETS: [isize; 16] = [8, -8, -1, 1, 9, -9, 7, -7, 6, 15, -6, -15, 17, 10, -17, -10];

pub struct Board {
    // OpenGL drawing backend.
    pub gl: GlGraphics,
    pub grid: Grid,
    pub pieces: [Piece; SIZE],
    pub moving_color: PieceColor,
    pub selected: Option<usize>,
    pub released: Option<usize>,
    pub selected_piece: Piece,
    pub cursor_pos: [f64; 2],
}

impl Board {
    pub fn render(&mut self, args: &RenderArgs) {
        let pieces = self.pieces;

        for i in 0..SIZE {
            // 0, 1, 2, 3, 4, 5, 6, 7
            let rank = 7 - (i % 8);
            // 0, 0, 0, 0, 0, 0, 0, 1
            let file = i / 8;

            let piece: Piece = pieces[i];
            self.draw_square(args, piece, rank, file)
        }

        if let Some(i) = self.selected {
            // 0, 1, 2, 3, 4, 5, 6, 7
            let rank = i % 8;
            // 0, 0, 0, 0, 0, 0, 0, 1
            let file = (i / 8);
            self.draw_square(args, Piece::default(), rank, 7 - file);
            self.draw_move(7 - rank, 7 - file, args);
        }
        if let Some(i) = self.released {
            if self.selected_piece.piece_type != PieceType::EMPTY {
                // original location
                let orig = self.selected.unwrap();
                // println!("orig: {}, new: {}", orig, i);
                // 0, 1, 2, 3, 4, 5, 6, 7
                let rank = i % 8;
                // 0, 0, 0, 0, 0, 0, 0, 1
                let file = 7 - (i / 8);

                let moves = Board::get_moves(self, self.selected_piece, 7 - (orig % 8), 7 - (orig / 8));
                let translation = &(i as isize - orig as isize);
                // println!("trans: {}", translation);
                let target: Piece = self.pieces[63 - i];
                if moves.contains(translation) && target.piece_color != self.selected_piece.piece_color {
                    self.draw_square(args, self.selected_piece, rank, file);
                    // Input piece adn remove from original
                    self.pieces[63 - orig] = Piece::default();
                    self.pieces[63 - i] = self.selected_piece;
                } else {
                    self.pieces[63 - orig] = self.selected_piece;
                }
                // empty selection and release
                self.selected_piece = Piece::default();
                self.selected = None;
                self.released = None;
            }
        }
    }
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let (x, y) = Board::calculate_coords(self.cursor_pos[0], self.cursor_pos[1]);
            // println!("Cell picked: {},{}", x, y);
            let i = (x - 1) + (y - 1) * 8;
            println!("index selected: {}", i);
            let piece: Piece = self.pieces[63 - i];
            // println!("Piece: {}", i);
            if piece.piece_type != PieceType::EMPTY {
                self.selected = Some(i);
                self.selected_piece = self.pieces[63 - i];
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            let (x, y) = Board::calculate_coords(self.cursor_pos[0], self.cursor_pos[1]);
            // println!("Cell picked: {},{}", x, y);
            let i = (x - 1) + (y - 1) * 8;
            // println!("index released: {}", i);
            if self.selected_piece.piece_type != PieceType::EMPTY {
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
    fn draw_square(&mut self, args: &RenderArgs, piece: Piece, rank: usize, file: usize) {
        let checker_square: [f64; 4] = rectangle::square(0.0, 0.0, 50.0);
        let white: [f32; 4] = color::hex("F0D9B5");
        let black: [f32; 4] = color::hex("946f51");
        let (x, y) = ((rank * 50) as f64, (file * 50) as f64);
        let color_state = (rank + file) % 2 != 0;
        match piece.piece_type {
            PieceType::EMPTY => {
                self.gl.draw(args.viewport(), |c, gl| {
                    if color_state {
                        rectangle(black, checker_square, c.transform.trans(x, y), gl);
                    } else {
                        rectangle(white, checker_square, c.transform.trans(x, y), gl);
                    };
                });
            }
            _ => {
                let image = Image::new().rect(square(x, y, 50.0));
                let texture = piece.get_icon();
                self.gl.draw(args.viewport(), |c, gl| {
                    if color_state {
                        rectangle(black, checker_square, c.transform.trans(x, y), gl);
                    } else {
                        rectangle(white, checker_square, c.transform.trans(x, y), gl);
                    };
                    image.draw(&texture, &c.draw_state, c.transform, gl);
                });
            }
        }
    }
    fn draw_move(&mut self, rank: usize, file: usize, args: &RenderArgs) {
        // println!("rank: {}, file: {}", rank, file);
        let checker_square: [f64; 4] = rectangle::square(15.0, 15.0, 20.0);
        let red = [199.0, 0.0, 0.0, 0.5];
        let moves: Vec<isize> = self.get_moves(self.selected_piece, rank, file);
        let pieces = self.pieces;

        for m in moves.iter() {
            let coord: isize = (rank + (file + 1) * 8) as isize - *m;
            let (x, y) = ((7 - coord % 8) as f64 * 50.0, ((coord / 8) - 1) as f64 * 50.0);
            // println!("rank: {}, file: {}, index: {},move: {}", rank, file, coord, m);

            self.gl.draw(args.viewport(), |c, gl| {
                // rectangle(red, checker_square, c.transform.trans(x, y), gl);
                ellipse(red, checker_square, c.transform.trans(x, y), gl);
            });
        }
    }
    fn get_moves(&self, piece: Piece, rank: usize, file: usize) -> Vec<isize> {
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

        match piece.piece_type {
            PieceType::Knight => {
                let directions = &direction_to_edge[4..8];
                let offsets = &OFFSETS[8..16];
                for i in 0..1 {
                    let x_offset = offsets[i] % 8;
                    let edge = directions[i / 2] as isize;
                    println!("offset: {}, edge: {}", x_offset, edge);
                    moves.push(offsets[i]);
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
                    _ => {}
                }
            }
            _ => {}
        };
        moves
    }
    fn sliding_piecs_moves(&self, directions: &[usize], types: &[isize]) -> Vec<isize> {
        let mut moves: Vec<isize> = vec![];
        let directions = &directions;
        // iter through north,south,east,west
        for x in 0..directions.len() {
            // move a square until it hits the edge
            // println!("amount to edge: {}", directions[x]);
            for y in 0..directions[x] {
                let m: isize = types[x] * (1 + y as isize);
                moves.push(m);
            }
        }
        moves
    }
}