use vector::Vector2;

use crate::Rectangle;

const NODE_CAPACITY: usize = 4;

pub struct Quadtree<T> {
    boundary: Rectangle,

    entries: Vec<(Vector2<f32>, T)>,

    quadrants: Option<[Box<Quadtree<T>>; 4]>,
}

impl<T: Copy> Quadtree<T> {
    pub fn new(boundary: Rectangle) -> Quadtree<T> {
        Quadtree::<T> { 
            boundary, 
            entries: Vec::new(), 
            quadrants: None,
        }
    }

    pub fn insert(&mut self, entry: (Vector2<f32>, T)) -> Result<(), Box<dyn std::error::Error>> {
        if !self.boundary.contains(&entry.0) {
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

    pub fn query(&self, range: &Rectangle) -> Vec<T> {
        let mut result = Vec::new();

        if !self.boundary.intersects(&range) {
            return result;
        }

        for entry in &self.entries {
            if range.contains(&entry.0) {
                result.push(entry.1);
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

        let half_dim = Vector2::new(hx / 2.0, hy / 2.0);

        // North-West quadrant
        let nw_center = Vector2::new(px - hx / 2.0, py - hy / 2.0);
        let north_west = Box::new(Quadtree::<T>::new(
            Rectangle::new(nw_center, half_dim.clone()))
        );
    
        // North-East quadrant
        let ne_center = Vector2::new(px + hx / 2.0, py - hy / 2.0);
        let north_east = Box::new(Quadtree::<T>::new(
            Rectangle::new(ne_center, half_dim.clone()))
        );

        // South-West quadrant
        let sw_center = Vector2::new(px - hx / 2.0, py + hy / 2.0);
        let south_west = Box::new(Quadtree::<T>::new(
            Rectangle::new(sw_center, half_dim.clone()))
        );

        // South-East quadrant
        let se_center = Vector2::new(px + hx / 2.0, py + hy / 2.0);
        let south_east = Box::new(Quadtree::<T>::new(
            Rectangle::new(se_center, half_dim.clone()))
        );

        self.quadrants = Some([north_west, north_east, south_west, south_east]);
    }
}