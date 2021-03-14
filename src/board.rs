use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs};
use graphics::grid::Grid;
use crate::pieces::{Piece, PieceType, PieceColor};
use graphics::*;
use graphics::rectangle::square;
use piston::{GenericEvent, Button, MouseButton};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::borrow::Borrow;

extern crate piston_window;

const SIZE: usize = 64;

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
    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let pieces = self.pieces;

        for i in 0..SIZE {
            // 0, 1, 2, 3, 4, 5, 6, 7
            let rank = 7 - (i % 8);
            // 0, 0, 0, 0, 0, 0, 0, 1
            let file = i / 8;

            let piece: Piece = pieces[i];
            Board::draw_square(args, gl, piece, rank, file)
        }

        if let Some(i) = self.selected {
            // 0, 1, 2, 3, 4, 5, 6, 7
            let rank = i % 8;
            // 0, 0, 0, 0, 0, 0, 0, 1
            let file = 7 - (i / 8);
            Board::draw_square(args, gl, Piece::new(PieceType::EMPTY, PieceColor::NEITHER), rank, file);
            Board::draw_move(self.selected_piece, rank, file, args, gl);
        }
        if let Some(i) = self.released {
            if self.selected_piece.piece_type != PieceType::EMPTY {
                // original location
                let orig = self.selected.unwrap();
                println!("orig: {}, new: {}", orig, i);
                // 0, 1, 2, 3, 4, 5, 6, 7
                let rank = i % 8;
                // 0, 0, 0, 0, 0, 0, 0, 1
                let file = 7 - (i / 8);

                let moves = self.selected_piece.get_moves(7 - (orig % 8), 7 - (orig / 8));
                let translation = &(i as isize - orig as isize);
                println!("trans: {}", translation);
                for x in moves.iter() {
                    println!("move possible: {}", x);
                }
                if moves.contains(translation) {
                    Board::draw_square(args, gl, self.selected_piece, rank, file);
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
            // println!("index selected: {}", i);
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
    fn draw_square(args: &RenderArgs, gl: &mut GlGraphics, piece: Piece, rank: usize, file: usize) {
        let checker_square: [f64; 4] = rectangle::square(0.0, 0.0, 50.0);
        let white: [f32; 4] = color::hex("F0D9B5");
        let black: [f32; 4] = color::hex("946f51");
        let (x, y) = ((rank * 50) as f64, (file * 50) as f64);
        let color_state = (rank + file) % 2 != 0;
        match piece.piece_type {
            PieceType::EMPTY => {
                gl.draw(args.viewport(), |c, gl| {
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
                gl.draw(args.viewport(), |c, gl| {
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
    fn draw_move(selected: Piece, rank: usize, file: usize, args: &RenderArgs, gl: &mut GlGraphics) {
        let checker_square: [f64; 4] = rectangle::square(0.0, 0.0, 50.0);
        let red = color::hex("9bc700");
        let mut moves: Vec<isize> = selected.get_moves(rank, file);

        for i in moves.iter() {
            let coord = (rank as isize + file as isize * 8) - i;
            let (x, y) = (((coord as f64 % 8.0) * 50.0, (coord / 8) as f64 * 50.0));
            // println!("x: {}, y: {}, coord: {},rank: {}, file: {}", x, y, coord, rank, file);
            gl.draw(args.viewport(), |c, gl| {
                rectangle(red, checker_square, c.transform.trans(x, y), gl);
            });
        }
    }
}