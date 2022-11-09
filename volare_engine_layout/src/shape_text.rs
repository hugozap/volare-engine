//struct ShapeText that is PositionableWithBoundingBox

use crate::bounding_box::{BoundingBox};
use crate::location::{Location, PositionableWithBoundingBox};
use crate::session::{Session};



pub struct ShapeText {
    pub location: Location,
    pub text: String,
    pub text_options: TextOptions,
}

//struct with text options: font family, font size
#[derive(Default)]
pub struct TextOptions {
    pub font_family: String,
    pub font_size: f64,
    pub text_color: String,
}

//impl ShapeText

impl ShapeText {
    pub fn new() -> ShapeText {
        ShapeText {
            location: Location { x: 0.0, y: 0.0 },
            text: String::new(),
            text_options: TextOptions {
                font_family: String::from("Arial"),
                font_size: 12.0,
                text_color: String::from("black"),
            },
        }
    }

    pub fn measure_text(&self) -> (f64, f64) {
        let session = Session::get_instance();
        let measure_text = session.measure_text.unwrap();
        measure_text(&self.text, &self.text_options)
    }
}

//impl PositionableWithBoundingBox trait for ShapeText

impl PositionableWithBoundingBox for ShapeText {
    fn get_location(&self) -> &Location {
        &self.location
    }

    fn set_location(&mut self, location: &Location) {
        self.location.x = location.x;
        self.location.y = location.y;
    }

    fn get_bounding_box(&self) -> BoundingBox {
        let (w,h) = self.measure_text();
        BoundingBox {
            x: self.location.x,
            y: self.location.y,
            width: w,
            height: h,
        }
    }
}

//test

#[cfg(test)]
mod tests {
    use crate::shape_text::ShapeText;
    use crate::bounding_box::{BoundingBox};
    use crate::location::{Location, PositionableWithBoundingBox};

    #[test]
    fn test_get_bounding_box() {

        //set dummy measure_text
        let session = crate::session::Session::get_instance();
        session.measure_text = Some(|text, text_options| {
            (100.0, 100.0)
        });

        let mut stext = ShapeText::new();
        stext.text = String::from("Hello World");
       
        let bb = stext.get_bounding_box();
        assert_eq!(bb.x, 0.0);
        assert_eq!(bb.y, 0.0);
        assert_eq!(bb.width, 100.0);
        assert_eq!(bb.height, 100.0);


    }
}
