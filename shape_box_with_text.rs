/* The BoxWithText shape is a box with text inside it. It is a composite shape that contains a box and a text shape. The text is centered inside the box. This is the main shape for a lot of diagrams*/
use crate::bounding_box::{BoundingBox};
use crate::location::{Location, PositionableWithBoundingBox};
use crate::shape_text::TextOptions;

pub struct ShapeBoxWithText {
    pub location: Location,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub text_options: TextOptions,
}

//impl box

impl ShapeBoxWithText {
    pub fn new() -> ShapeBoxWithText {
        ShapeBox {
            location: Location { x: 0.0, y: 0.0 },
            text: String::new(),
            text_options: TextOptions {
                font_family: String::from("Arial"),
                font_size: 12.0,
                text_color: String::from("black"),
            },
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
