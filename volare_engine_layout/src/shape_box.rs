use crate::bounding_box::{BoundingBox};
use crate::location::{Location, PositionableWithBoundingBox};

pub struct ShapeBox {
    pub location: Location,
    pub width: f64,
    pub height: f64,
}

//impl box

impl ShapeBox {
    pub fn new() -> ShapeBox {
        ShapeBox {
            location: Location { x: 0.0, y: 0.0 },
            width: 0.0,
            height: 0.0
        }
    }
}

//impl PositionableWithBoundingBox trait for ShapeBox

impl PositionableWithBoundingBox for ShapeBox {
    fn get_location(&self) -> &Location {
        &self.location
    }

    fn set_location(&mut self, location: &Location) {
        self.location.x = location.x;
        self.location.y = location.y;
    }

    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            x: self.location.x,
            y: self.location.y,
            width: self.width,
            height: self.height
        }
    }
}