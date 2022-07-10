mod rectangle;
mod tree;

pub use rectangle::Rectangle;
pub use tree::Quadtree;
use vector::Vector2;

pub trait Positioned {
    fn position(&self) -> &Vector2<f32>;
}

impl Positioned for Vector2<f32> {
    fn position(&self) -> &Vector2<f32> {
        self
    }
}