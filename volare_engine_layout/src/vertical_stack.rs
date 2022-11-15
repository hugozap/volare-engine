/*
 * The VerticalStack is a shape that stacks other shapes vertically.
* it implements the PositionableWithBoundingBox trait
*/

use crate::bounding_box::BoundingBox;
use crate::location::{Location, PositionableWithBoundingBox};
//import the shapetype enum
use crate::utils::*;
use std::cell::RefCell;

pub struct VerticalStack {
    pub location: Location,
    pub width: f64,
    pub height: f64,
    pub padding: f64,
    pub children: Vec<RefCell<ShapeType>>,
    elems: Vec<RefCell<Box<dyn PositionableWithBoundingBox>>>,
}

//impl VerticalStack
impl VerticalStack {
    pub fn new() -> VerticalStack {
        VerticalStack {
            location: Location { x: 0.0, y: 0.0 },
            width: 0.0,
            height: 0.0,
            padding: 0.0,
            elems: Vec::new(),
            children: Vec::new(),
        }
    }

    /* Update the position of the elements in the stack */
    pub fn update_layout(&mut self) {
        let mut y = 0.0;
        for elem in &self.elems {
            let mut elem = elem.borrow_mut();
            elem.set_location(&Location {
                x: 0.0,
                y: y + self.padding,
            });
            y += elem.get_bounding_box().height + self.padding;
        }
        
    }
}

impl PositionableWithBoundingBox for VerticalStack {
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
            height: self.height,
        }
    }
}
