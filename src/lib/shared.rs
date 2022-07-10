pub const PIXEL_SIZE: f64 = 10.0;
pub const FPS: u64 = 8;
pub const MAX_X: i32 = 20;
pub const MAX_Y: i32 = 20;

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    StandBy,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl From<&(i32, i32)> for Point {
    fn from(w: &(i32, i32)) -> Point {
        Point { x: w.0, y: w.1 }
    }
}
impl From<(i32, i32)> for Point {
    fn from(w: (i32, i32)) -> Point {
        Point { x: w.0, y: w.1 }
    }
}
pub fn square_from_coordinates(x: &i32, y: &i32) -> [f64; 4] {
    graphics::rectangle::square(
        (*x as f64) * PIXEL_SIZE,
        (*y as f64) * PIXEL_SIZE,
        PIXEL_SIZE,
    )
}

pub fn are_coordinates_collide(point_1: &Point, point_2: &Point) -> bool {
    point_1.x == point_2.x && point_1.y == point_2.y
}
