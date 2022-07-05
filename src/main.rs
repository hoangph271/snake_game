extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::sync::{Arc, Mutex};

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    window::WindowSettings, ButtonEvent, ButtonState, EventLoop, EventSettings, Events,
    RenderEvent, UpdateEvent,
};

mod lib;
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
    }
}

fn play_background_music(is_game_ended: Arc<Mutex<bool>>) {
    use rodio::{source::Source, Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;

    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        // let binary = include_bytes!("bin/8_bit_adventure.mp3"); // TODO: Package with the binary
        let binary = File::open("src/bin/8_bit_adventure.mp3").unwrap();
        let file = BufReader::new(binary);
        let source = Decoder::new_looped(file).unwrap();

        stream_handle.play_raw(source.convert_samples()).unwrap();
        loop {
            // FIXME: Workaround for keeping the background music playing
            if *is_game_ended.lock().unwrap() {
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(10))
            }
        }
    } else {
        println!("Can NOT init sound output...!");
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window = create_window(open_gl);
    let mut game = create_game(open_gl);
    let is_game_ended = Arc::new(Mutex::new(false));

    let is_game_ended_clone = is_game_ended.clone();
    let audio_thread = std::thread::spawn(move || {
        play_background_music(is_game_ended_clone);
    });

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

    *is_game_ended.lock().unwrap() = true;
    audio_thread.join().unwrap();
}
