extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

const PIXEL_SIZE: i32 = 20;
const FIELD_SIZE_X: i32 = 30;
const FIELD_SIZE_Y: i32 = 30;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
                     graphics::clear(BLACK, gl);
        });

        self.snake.render(&mut self.gl, args);

        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn button_press(&mut self, button: &Button) {
        self.snake.change_direction(&button);
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    direction: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * PIXEL_SIZE) as f64,
            (self.pos_y * PIXEL_SIZE) as f64,
            PIXEL_SIZE as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(GREEN, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => self.pos_y -=1,
            Direction::Down => self.pos_y +=1,
            Direction::Left => self.pos_x -=1,
            Direction::Right => self.pos_x +=1,
        }

        if self.pos_x > FIELD_SIZE_X {
            self.pos_x = 0;
        } else if self.pos_x < 0 {
            self.pos_x = FIELD_SIZE_X;
        }

        if self.pos_y > FIELD_SIZE_Y {
            self.pos_y = 0;
        } else if self.pos_y < 0 {
            self.pos_y = FIELD_SIZE_Y;
        }

    }

    fn change_direction(&mut self, button: &Button) {
        let last_direction = self.direction.clone();

        self.direction = match button {
            Button::Keyboard(Key::W) => Direction::Up,
            Button::Keyboard(Key::S) => Direction::Down,
            Button::Keyboard(Key::A) => Direction::Left,
            Button::Keyboard(Key::D) => Direction::Right,
            _ => last_direction,
        }
    }
}

struct Food {
    pos_x: i32,
    pos_y: i32,
}

impl Food {
    fn new() -> Food {
        let random_x: i32 = 3;
        let random_y: i32 = 3;

        Food { pos_x: random_x, pos_y: random_y }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * PIXEL_SIZE) as f64,
            (self.pos_y * PIXEL_SIZE) as f64,
            PIXEL_SIZE as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "snake-rs",
        [(FIELD_SIZE_X * PIXEL_SIZE) as f64,
        (FIELD_SIZE_Y * PIXEL_SIZE) as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 0,
            pos_y: 0,
            direction: Direction::Right,
        },
        food: Food::new(),
    };


    let mut events = Events::new(EventSettings::new())
        .max_fps(60)
        .ups(10);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {
                game.button_press(&b.button);
            }
        }
    }
}

