use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use super::shared::{self, Point};

pub struct PixelsPack {
    pub color: [f32; 4],
    pub points: Vec<Point>,
}

pub struct Renderer {
    pub pixels_packs: Vec<PixelsPack>,
}

impl Renderer {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        for pixels_pack in &self.pixels_packs {
            for point in &pixels_pack.points {
                let square = shared::square_from_coordinates(&point.x, &point.y);

                gl.draw(args.viewport(), |c, gl| {
                    graphics::rectangle(pixels_pack.color, square, c.transform, gl);
                });
            }
        }
    }
}
