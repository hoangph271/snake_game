use opengl_graphics::GlGraphics;
use piston::input::*;
use piston::Button;

use crate::lib::colors;
use crate::lib::shared::Direction;
use crate::lib::snake::Snake;

use super::renderer::Renderer;

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
}
impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(colors::GRAY, gl);
        });

        let renderer: Renderer = (&self.snake).into();
        renderer.render(&mut self.gl, args)
    }

    pub fn update(&mut self) {
        if self.snake.is_alive() {
            self.snake.update();
        }
    }

    pub fn handle_button(&mut self, btn: &Button) {
        let last_heading = self.snake.heading.clone();

        self.snake.heading = match btn {
            Button::Keyboard(Key::Up) if last_heading != Direction::Down => Direction::Up,
            Button::Keyboard(Key::Down) if last_heading != Direction::Up => Direction::Down,
            Button::Keyboard(Key::Left) if last_heading != Direction::Right => Direction::Left,
            Button::Keyboard(Key::Right) if last_heading != Direction::Left => Direction::Right,
            _ => last_heading,
        }
    }
}
