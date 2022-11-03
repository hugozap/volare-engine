
use crate::bounding_box::{BoundingBox};
pub struct Location {
    pub x: f64,
    pub y: f64,
}

// positionable trait
pub trait PositionableWithBoundingBox {
     fn set_location(&mut self, location: &Location);
     fn get_location(&self) -> &Location;
    fn get_bounding_box(&self) -> BoundingBox;
}