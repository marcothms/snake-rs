extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
                     graphics::clear(BLACK, gl);
        });

        self.snake.render(&mut self.gl, args);
    }
}

struct Snake {
    x_pos: i32,
    y_pos: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            self.x_pos as f64,
            self.y_pos as f64,
            20.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(GREEN, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "snake-rs",
        [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            x_pos: 50,
            y_pos: 50
        },
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}

