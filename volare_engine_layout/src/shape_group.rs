use crate::bounding_box::BoundingBox;
use crate::location::{Location, PositionableWithBoundingBox};
use crate::shape_box::ShapeBox;
use crate::utils::*;
/**
 * The group has its own position separated from its children
 * When painting, the rendering layer should transform the space to move
 * to the group position and then paint the children (which are relative to the group)
 */
pub struct ShapeGroup {
    pub location: Location,
    pub children: Vec<ShapeType>,
}

//impl box

impl ShapeGroup {
    pub fn new() -> ShapeGroup {
        ShapeGroup {
            location: Location { x: 0.0, y: 0.0 },
            children: Vec::new(),
        }
    }
}

//impl PositionableWithBoundingBox trait for ShapeBox

impl PositionableWithBoundingBox for ShapeGroup {
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
            height: max_y - min_y,
        }
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bounding_box() {
        let mut shape_group = ShapeGroup::new();
        let mut shape_box = ShapeBox::new();
        shape_box.location.x = 10.0;
        shape_box.location.y = 10.0;
        shape_box.width = 10.0;
        shape_box.height = 10.0;
        shape_group.children.push(ShapeType::ShapeBox(shape_box));
        let mut shape_box = ShapeBox::new();
        shape_box.location.x = 20.0;
        shape_box.location.y = 20.0;
        shape_box.width = 10.0;
        shape_box.height = 10.0;
        shape_group.children.push(ShapeType::ShapeBox(shape_box));
        let bb = shape_group.get_bounding_box();
        assert_eq!(bb.x, 0f64);
        assert_eq!(bb.y, 0f64);
        assert_eq!(bb.width, 30.0);
        assert_eq!(bb.height, 30.0);
    }
}
