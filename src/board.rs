use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};
use graphics::grid::Grid;

pub struct Board {
    // OpenGL drawing backend.
    pub gl: GlGraphics,
    pub grid: Grid,
}

impl Board {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let white: [f32; 4] = color::hex("F0D9B5");
        let black: [f32; 4] = color::hex("946f51");

        let square = rectangle::square(0.0, 0.0, 50.0);
        // let grid = self.grid;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(white, gl);

            // let cells = grid.cells();
            // grid.draw(&line,
            //           &c.draw_state,
            //           c.transform,
            //           gl);

            for file in 0..8 {
                for rank in 0..8 {
                    let x: f64 = (file * 50) as f64;
                    let y: f64 = (rank * 50) as f64;
                    let color_state = (file + rank) % 2 != 0;
                    if color_state {
                        rectangle(black, square, c.transform.trans(x, y), gl)
                    } else {
                        rectangle(white, square, c.transform.trans(x, y), gl)
                    };
                }
            }
        });
    }
}