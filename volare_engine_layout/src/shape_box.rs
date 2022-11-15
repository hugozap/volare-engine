use crate::bounding_box::{BoundingBox};
use crate::location::{Location, PositionableWithBoundingBox};
//use shapetype enum
use crate::utils::*;

pub struct ShapeBox {
    pub location: Location,
    pub width: f64,
    pub height: f64,
    pub padding: f64,
    pub elem: Option<Box<ShapeType>>,
    pub box_options: BoxOptions,

}

//BoxOptions struct
#[derive(Default)]
pub struct BoxOptions {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
}

//impl box

impl ShapeBox {
    pub fn new() -> ShapeBox {
        ShapeBox {
            location: Location { x: 0.0, y: 0.0 },
            width: 0.0,
            height: 0.0,
            padding: 0.0,
            elem: None,
            box_options: BoxOptions {
                fill_color: String::from("white"),
                stroke_color: String::from("black"),
                stroke_width: 1.0,
            },
        }
    }

    /// The element position is relative to the box location
    /// The top left corner will be equal to (padding, padding)
    pub fn get_element_position(&self) -> Location {
        Location {
            x: self.padding,
            y: self.padding,
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

        //if there is an element in the box then return the bounding box of the element + padding
        //otherwise return the bounding box with the width and height originally set
        //
        match &self.elem {
            Some(elem) => {
                let mut bounding_box = get_shape_type_bounding_box(&**elem);
                bounding_box.width += self.padding * 2.0;
                bounding_box.height += self.padding * 2.0;
                println!("bounding box for elem,  width: {}", bounding_box.width);
                bounding_box
            }
            None => BoundingBox {
                x: self.location.x,
                y: self.location.y,
                width: self.width,
                height: self.height,
            },
        }
    }
}
//tests

#[cfg(test)]
mod tests {
    use crate::shape_box::ShapeBox;
    use crate::shape_text::ShapeText;
    use crate::bounding_box::{BoundingBox};
    use crate::location::{ PositionableWithBoundingBox};
    use crate::session::Session;
    use crate::diagram_layout::{ShapeType};

    #[test]
    fn test_get_bounding_box() {

        let mut sbox = ShapeBox::new();
        sbox.width = 100.0;
        sbox.height = 100.0;
        sbox.padding = 10.0;

        let bb = sbox.get_bounding_box();
        assert_eq!(bb.x, 0.0);
        assert_eq!(bb.y, 0.0);
        assert_eq!(bb.width, 100.0);
        assert_eq!(bb.height, 100.0);

    }

    #[test]
    fn test_get_bounding_box_with_element() {

        //set dummy measure_text
        let session = Session::get_instance();
        session.measure_text = Some(|_text, _text_options| {
            (1000.0, 100.0)
        });

        let mut sbox = ShapeBox::new();
        sbox.width = 100.0;
        sbox.height = 100.0;
        sbox.padding = 10.0;

        //set text element
        let mut stext = ShapeText::new();
        stext.text = String::from("Hello World");
        sbox.elem = Some(Box::new(ShapeType::ShapeText(stext)));

        let bb = sbox.get_bounding_box();
        assert_eq!(bb.x, 0.0);
        assert_eq!(bb.y, 0.0);
        assert_eq!(bb.width, 1020.0);
        assert_eq!(bb.height, 120.0);


       
    }

}
