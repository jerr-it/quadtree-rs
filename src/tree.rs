use vector::Vector2;

use crate::{Rectangle, Positioned};

const NODE_CAPACITY: usize = 4;

pub struct Quadtree<'a, T> {
    boundary: Rectangle,

    entries: Vec<&'a T>,

    quadrants: Option<[Box<Quadtree<'a, T>>; 4]>,
}

impl<'a, T: Positioned + Sync> Quadtree<'a, T> {
    pub fn new(center_x: f32, center_y: f32, half_dim_x: f32, half_dim_y: f32) -> Quadtree<'a, T> {
        Quadtree { 
            boundary: Rectangle::new(center_x, center_y, half_dim_x, half_dim_y), 
            entries: Vec::new(), 
            quadrants: None,
        }
    }

    pub fn insert(&mut self, entry: &'a T) -> Result<(), Box<dyn std::error::Error>> {
        if !self.boundary.contains(entry) {
            return Err("Entry not within bounds")?;
        }

        if self.entries.len() < NODE_CAPACITY && self.quadrants.is_none() {
            self.entries.push(entry);
            return Ok(());
        }

        if self.quadrants.is_none() {
            self.subdivide();
        }

        for quadrant in self.quadrants.as_mut().unwrap() {
            if quadrant.insert(entry).is_ok() {
                return Ok(());
            }
        }

        Err("This should not happen")?
    }

    pub fn query(&self, range: &Rectangle) -> Vec<&T> {
        let mut result = Vec::new();

        if !self.boundary.intersects(&range) {
            return result;
        }

        for entry in &self.entries {
            if range.contains(*entry) {
                result.push(*entry);
            }
        }

        if self.quadrants.is_none() {
            return result;
        }

        for quadrant in self.quadrants.as_ref().unwrap() {
            result.append(&mut quadrant.query(range));
        }

        result
    }

    fn subdivide(&mut self) {
        let (px, py) = (self.boundary.center.x, self.boundary.center.y);
        let (hx, hy) = (self.boundary.half_dim.x, self.boundary.half_dim.y);

        let (hx, hy) = (hx / 2.0, hy / 2.0);

        // North-West quadrant
        let nw_center = Vector2::new(px - hx, py - hy);
        let north_west = Box::new(Quadtree::new(nw_center.x, nw_center.y, hx, hy));

        // North-East quadrant
        let ne_center = Vector2::new(px + hx, py - hy);
        let north_east = Box::new(Quadtree::new(ne_center.x, ne_center.y, hx, hy));    

        // South-West quadrant
        let sw_center = Vector2::new(px - hx, py + hy);
        let south_west = Box::new(Quadtree::new(sw_center.x, sw_center.y, hx, hy));

        // South-East quadrant
        let se_center = Vector2::new(px + hx, py + hy);
        let south_east = Box::new(Quadtree::new(se_center.x, se_center.y, hx, hy));

        self.quadrants = Some([north_west, north_east, south_west, south_east]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector::Vector2;
    use crate::Rectangle;
    use crate::Positioned;

    #[test]
    fn test_insert() {
        let mut tree = Quadtree::new(0.0, 0.0, 100.0, 100.0);
        let entry = Vector2::new(50.0, 50.0);
        assert!(tree.insert(&entry).is_ok());
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut tree = Quadtree::new(0.0, 0.0, 100.0, 100.0);
        let entry = Vector2::new(150.0, 150.0);
        assert!(tree.insert(&entry).is_err());
    }

    #[test]
    fn test_query() {
        let mut tree = Quadtree::new(0.0, 0.0, 100.0, 100.0);
        let entry = Vector2::new(50.0, 50.0);
        tree.insert(&entry).unwrap();
        let range = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let result = tree.query(&range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position(), entry.position());
    }

    #[test]
    fn test_query_out_of_bounds() {
        let mut tree = Quadtree::new(0.0, 0.0, 100.0, 100.0);
        let entry = Vector2::new(150.0, 150.0);
        if tree.insert(&entry).is_err() {}
        let range = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let result = tree.query(&range);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_subdivide_through_insert() {
        let mut tree = Quadtree::new(0.0, 0.0, 100.0, 100.0);
        let entry1 = Vector2::new(25.0, 25.0);
        let entry2 = Vector2::new(75.0, 25.0);
        let entry3 = Vector2::new(25.0, 75.0);
        let entry4 = Vector2::new(75.0, 75.0);
        let entry5 = Vector2::new(80.0, 80.0);
        
        tree.insert(&entry1).unwrap();
        tree.insert(&entry2).unwrap();
        tree.insert(&entry3).unwrap();
        tree.insert(&entry4).unwrap();
        tree.insert(&entry5).unwrap();

        assert_eq!(tree.quadrants.as_ref().unwrap().len(), 4);
        assert_eq!(tree.entries.len(), 4);

        // Check if the entries are in the correct quadrants
        let quadrants = tree.quadrants.as_ref().unwrap();

        let north_west = quadrants[0].as_ref();
        let north_east = quadrants[1].as_ref();
        let south_west = quadrants[2].as_ref();
        let south_east = quadrants[3].as_ref();

        let north_west_entries = north_west.query(
            &Rectangle::new(25.0, 25.0, 25.0, 25.0));
        let north_east_entries = north_east.query(
            &Rectangle::new(75.0, 25.0, 25.0, 25.0));
        let south_west_entries = south_west.query(
            &Rectangle::new(25.0, 75.0, 25.0, 25.0));
        let south_east_entries = south_east.query(
            &Rectangle::new(75.0, 75.0, 25.0, 25.0));

        assert_eq!(north_west_entries.len(), 0);
        assert_eq!(north_east_entries.len(), 0);
        assert_eq!(south_west_entries.len(), 0);
        assert_eq!(south_east_entries.len(), 1);
    }
}