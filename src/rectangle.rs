use vector::Vector2;

use crate::Positioned;

pub struct Rectangle {
    pub center: Vector2<f32>,
    pub half_dim: Vector2<f32>,
}

impl Rectangle {
    pub fn new(center_x: f32, center_y: f32, half_dim_x: f32, half_dim_y: f32) -> Rectangle {
        Rectangle { 
            center: Vector2::new(center_x,center_y), 
            half_dim: Vector2::new(half_dim_x, half_dim_y)
        }
    }

    pub fn contains(&self, point: &dyn Positioned) -> bool {
        let point = point.position();
        !(
            point.x < self.center.x - self.half_dim.x || 
            point.x > self.center.x + self.half_dim.x || 
            point.y < self.center.y - self.half_dim.y ||
            point.y > self.center.y + self.half_dim.y
        )
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(
            other.center.x - other.half_dim.x > self.center.x + self.half_dim.x || 
            other.center.x + other.half_dim.x < self.center.x - self.half_dim.x || 
            other.center.y - other.half_dim.y > self.center.y + self.half_dim.y || 
            other.center.y + other.half_dim.y < self.center.y - self.half_dim.y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let point = Vector2::new(0.5, 0.5);

        assert!(rect.contains(&point));
    }

    #[test]
    fn test_rectangle_intersects() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let other = Rectangle::new(0.5, 0.5, 1.0, 1.0);

        assert!(rect.intersects(&other));
    }

    #[test]
    fn test_rectangle_does_not_intersect() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let other = Rectangle::new(4.0, 4.0, 1.0, 1.0);

        assert!(!rect.intersects(&other));
    }

    #[test]
    fn test_rectangle_does_not_contain() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let point = Vector2::new(2.0, 2.0);

        assert!(!rect.contains(&point));
    }
}