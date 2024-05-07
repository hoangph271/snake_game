use opengl_graphics::GlGraphics;
use piston::input::*;
use piston::Button;
use rand::Rng;

use crate::modules::colors;
use crate::modules::shared::Direction;
use crate::modules::snake::Snake;

use super::food::Food;
use super::renderer::Renderer;
use super::shared::{Point, MAX_X, MAX_Y};

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Option<Food>,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(colors::GRAY, gl);
        });

        let renderer: Renderer = (&self.snake).into();
        renderer.render(&mut self.gl, args);

        if let Some(food) = &self.food {
            let renderer: Renderer = food.into();
            renderer.render(&mut self.gl, args);
        }
    }

    pub fn update(&mut self) {
        if let Some(food) = &self.food {
            if self.snake.can_eat(&food.location) {
                self.snake.eat(food.location.clone());
                self.food = None;
            }
        }

        if self.snake.is_alive() {
            self.snake.update();
        }

        if self.food.is_none() {
            self.food = loop {
                let mut rng = rand::thread_rng();
                let random_point: Point = (rng.gen_range(0..MAX_X), rng.gen_range(0..MAX_Y)).into();

                if !self.snake.snake_collide(&random_point) {
                    break Some(Food {
                        location: random_point,
                    });
                }
            }
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
