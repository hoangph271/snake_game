use crate::lib::shared::Direction;

use crate::lib::colors;
use crate::lib::shared;

use super::renderer::PixelsPack;
use super::renderer::Renderer;
use super::shared::Point;

pub struct Snake {
    pub body: Vec<Point>,
    pub heading: Direction,
}

fn correct_next_head(next_head: &mut Point) {
    if next_head.x < 0 {
        next_head.x = shared::MAX_X;
    }
    if next_head.x > shared::MAX_X {
        next_head.x = 0;
    }

    if next_head.y < 0 {
        next_head.y = shared::MAX_Y;
    }
    if next_head.y > shared::MAX_Y {
        next_head.y = 0;
    }
}

impl Snake {
    pub fn update(&mut self) {
        let snake_head = self.snake_head();
        let mut next_head: Point = match self.heading {
            Direction::Up => (snake_head.x, snake_head.y - 1),
            Direction::Down => (snake_head.x, snake_head.y + 1),
            Direction::Left => (snake_head.x - 1, snake_head.y),
            Direction::Right => (snake_head.x + 1, snake_head.y),
            Direction::StandBy => (snake_head.x, snake_head.y),
        }
        .into();

        correct_next_head(&mut next_head);

        if self.heading != Direction::StandBy {
            self.body.push(next_head);
            self.body.remove(0);
        }
    }

    fn snake_head(&self) -> &Point {
        self.body.last().expect("PANIC! - Snake head is None")
    }

    fn snake_body(&self) -> &[Point] {
        &self.body[0..self.body.len() - 1]
    }

    pub fn is_alive(&self) -> bool {
        let head = self.snake_head();

        self.snake_collide(head)
    }

    pub fn snake_collide(&self, point: &Point) -> bool {
        for body_node in self.snake_body() {
            if shared::are_coordinates_collide(body_node, point) {
                return true;
            }
        }

        if shared::are_coordinates_collide(point, self.snake_head()) {
            return true;
        }

        false
    }

    pub fn can_eat(&self, point: &Point) -> bool {
        shared::are_coordinates_collide(self.snake_head(), &point)
    }
    pub fn eat(&mut self, point: Point) {
        self.body.push(point);
    }
}

impl From<&Snake> for Renderer {
    fn from(snake: &Snake) -> Renderer {
        let head_pixels_pack = PixelsPack {
            points: vec![snake.snake_head().clone()],
            color: colors::GREEN,
        };
        let body_pixel_packs = PixelsPack {
            color: colors::FADED_GREEN,
            points: snake.snake_body().into(),
        };

        Renderer {
            pixels_packs: vec![head_pixels_pack, body_pixel_packs],
        }
    }
}
