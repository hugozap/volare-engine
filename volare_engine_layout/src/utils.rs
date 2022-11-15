use crate::bounding_box::BoundingBox;
use crate::location::{Location, PositionableWithBoundingBox};
use crate::shape_box::ShapeBox;
use crate::shape_group::ShapeGroup;
use crate::shape_text::ShapeText;
use crate::vertical_stack::VerticalStack;

pub enum ShapeType {
    ShapeText(ShapeText),
    ShapeGroup(ShapeGroup),
    ShapeBox(ShapeBox),
    ShapeVerticalStack(VerticalStack),
}

pub fn get_shape_type_bounding_box(shape_type: &ShapeType) -> BoundingBox {
    match shape_type {
        ShapeType::ShapeText(text) => text.get_bounding_box(),
        ShapeType::ShapeGroup(group) => group.get_bounding_box(),
        ShapeType::ShapeBox(box_) => box_.get_bounding_box(),
        ShapeType::ShapeVerticalStack(vertical_stack) => vertical_stack.get_bounding_box(),
    }
}
