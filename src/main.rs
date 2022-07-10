extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod lib;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    window::WindowSettings, ButtonEvent, ButtonState, EventLoop, EventSettings, Events,
    RenderEvent, UpdateEvent,
};

use lib::game::Game;
use lib::shared::{Direction, FPS, MAX_X, MAX_Y, PIXEL_SIZE};
use lib::snake::Snake;

fn create_window(open_gl: OpenGL) -> GlutinWindow {
    WindowSettings::new(
        "Snake game...!",
        [(MAX_X as f64) * PIXEL_SIZE, (MAX_Y as f64) * PIXEL_SIZE],
    )
    .graphics_api(open_gl)
    .exit_on_esc(true)
    .build()
    .expect("create_window() failed...!")
}
fn create_game(open_gl: OpenGL) -> Game {
    Game {
        gl: GlGraphics::new(open_gl),
        snake: Snake {
            body: vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
                .iter()
                .map(|tuple| tuple.into())
                .collect(),
            heading: Direction::StandBy,
        },
        food: None
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window = create_window(open_gl);
    let mut game = create_game(open_gl);

    let mut events = Events::new(EventSettings::new()).ups(FPS);
    while let Some(e) = events.next(&mut window) {
        if !game.snake.is_alive() {
            break;
        }

        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if e.update_args().is_some() {
            game.update();
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.handle_button(&args.button)
            }
        }
    }
}
