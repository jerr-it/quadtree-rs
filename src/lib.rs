mod rectangle;
mod tree;

pub use rectangle::Rectangle;
pub use tree::Quadtree;

#[cfg(test)]
mod tests {
    use vector::Vector2;

    use crate::{Quadtree, Rectangle};

    #[derive(Debug, PartialEq)]
    struct Entity {
        position: Vector2<f32>,
    }

    #[test]
    fn qtree_test() {
        let entities: Vec<Entity> = (0..100).map(|x| Entity { position: Vector2::new(x as f32, x as f32) }).collect();

        let mut qtree = Quadtree::<&Entity>::new(Rectangle::new(
            Vector2::new(50.0, 50.0),
            Vector2::new(50.0, 50.0),
        ));

        for entity in &entities {
            let res = qtree.insert((entity.position, entity));
            assert!(!res.is_err());
        }

        let range = Rectangle::new(
            Vector2::new(25.0, 25.0),
            Vector2::new(25.0, 25.0),
        );

        let query_result = qtree.query(&range);

        for i in 0..50 {
            assert_eq!(entities[i], *query_result[i]);
        }

        let range = Rectangle::new(
            Vector2::new(25.0, 75.0),
            Vector2::new(20.0, 20.0),
        );

        let query_result = qtree.query(&range);
        assert!(query_result.is_empty());
    }
}
