use super::{
    colors,
    renderer::{PixelsPack, Renderer},
    shared::Point,
};

pub struct Food {
    pub location: Point,
}

impl From<&Food> for Renderer {
    fn from(food: &Food) -> Renderer {
        Renderer {
            pixels_packs: vec![PixelsPack {
                points: vec![food.location.clone()],
                color: colors::RED,
            }],
        }
    }
}
