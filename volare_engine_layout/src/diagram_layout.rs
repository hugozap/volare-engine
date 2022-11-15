use crate::location::{Location, PositionableWithBoundingBox};
use crate::bounding_box::{BoundingBox};
use crate::shape_text::ShapeText;
use crate::shape_box::ShapeBox;
use crate::shape_group::ShapeGroup;
use crate::utils::*;

//struct diagram layout that implements Positionable and HasBoundingBox traits
pub struct DiagramLayout {
    pub location: Location,
    //children is a vector of Positionable and HasBoundingBox objects
    pub children: Vec<ShapeType>,
}

//impl diagram layout
impl DiagramLayout {
    pub fn new() -> DiagramLayout {
        DiagramLayout {
            location: Location { x: 0.0, y: 0.0 },
            children: Vec::new(),
        }
    }
}

//impl HasBoundingBox trait for DiagramLayout
impl PositionableWithBoundingBox for DiagramLayout {

    fn get_location(&self) -> &Location {
        &self.location
    }

    fn set_location(&mut self, location: &Location) {
        self.location.x = location.x;
        self.location.y = location.y;
    }


    fn get_bounding_box(&self) -> BoundingBox {
        //calculate the bounding box getting the min and max values from children
        let mut min_x = 0.0;
        let mut min_y = 0.0;
        let mut max_x = 0.0;
        let mut max_y = 0.0;

        for child in &self.children {
            let bb = get_shape_type_bounding_box(child);
            if bb.x < min_x {
                min_x = bb.x;
            }
            if bb.y < min_y {
                min_y = bb.y;
            }
            if bb.right() > max_x {
                max_x = bb.right();
            }
            if bb.bottom() > max_y {
                max_y = bb.bottom();
            }
        }

        BoundingBox {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y
        }
    }
}


//test get_bounding_box use ShapeBox as children

#[cfg(test)]
mod tests {
    use crate::shape_box::ShapeBox;

    use super::*;

    #[test]
    fn test_get_bounding_box() {
        let mut dl = DiagramLayout::new();

        let mut box1 = ShapeBox::new();
        box1.set_location(&Location { x: 0.0, y: 0.0 });
        box1.width = 10.0;
        box1.height = 10.0;

        let mut box2 = ShapeBox::new();
        box2.set_location(&Location { x: 10.0, y: 10.0 });
        box2.width = 10.0;
        box2.height = 10.0;

        dl.children.push(ShapeType::ShapeBox(box1));
        dl.children.push(ShapeType::ShapeBox(box2));

        let bb = dl.get_bounding_box();

        assert_eq!(bb.x, 0.0);
        assert_eq!(bb.y, 0.0);
        assert_eq!(bb.width, 20.0);
        assert_eq!(bb.height, 20.0);
    }
}
