/* Layout calculation for each type of entity */

use crate::{Session, ShapeBox, ShapeGroup, ShapeLine, ShapeText, ShapeEllipse, ShapeArrow, ShapeImage, VerticalStack, HorizontalStack};

/* The box layout includes the padding and the dimensions
of the wrapped element
The wrapped element position and size should be updated before calling this function.
The wrapped element position is relative to the box position.
*/
pub fn layout_box(session: &mut Session, shape_box: &ShapeBox) {
    //get the wrapped element dimensions
    let wrapped_elem_size = session.get_position(shape_box.wrapped_entity);
    //set the box dimensions
    session.set_size(
        shape_box.entity,
        wrapped_elem_size.0 + shape_box.box_options.padding * 2.0,
        wrapped_elem_size.1 + shape_box.box_options.padding * 2.0,
    );
    //Update the wrapped element position
    session.set_position(
        shape_box.wrapped_entity,
        shape_box.box_options.padding,
        shape_box.box_options.padding,
    );
}

/**
 * Update the group size based on the size of the elements.
 * Group elements must be positioned before calling this function.
 * (Doesn't update the position of the elements)
 */
pub fn layout_group(session: &mut Session, shape_group: &ShapeGroup) {
    //update group dimensions
    let mut width = 0.0;
    let mut height = 0.0;
    for elem in shape_group.elements.iter() {
        let elem_size = session.get_size(*elem);
        if elem_size.0 > width {
            width = elem_size.0;
        }
        if elem_size.1 > height {
            height = elem_size.1;
        }
    }
    session.set_size(shape_group.entity, width, height);
}

/**
 * Updates the size of the text entity based on the text and text options
 */
pub fn layout_text(session: &mut Session, shape_text: &ShapeText) {
    let (w, h) = session.measure_text.unwrap()(&shape_text.text, &shape_text.text_options);
    session.set_size(shape_text.entity, w, h);
}

/**
 * Updates the size of the line entity based on the start and end points
 */
pub fn layout_line(session: &mut Session, shape_line: &ShapeLine) {
    let start = shape_line.start;
    let end = shape_line.end;
    //the line x is the minimum of the start and end x
    let x = start.0.min(end.0);
    let y = start.1.min(end.1);

    session.set_size(
        shape_line.entity,
        (end.0 - start.0).abs(),
        (end.1 - start.1).abs(),
    );

    session.set_position(shape_line.entity, x, y);
}

/**
 * Updates the size of the arrow entity based on the start and end points
 */
pub fn layout_arrow(session: &mut Session, shape_arrow: &ShapeArrow) {
    let start = shape_arrow.start;
    let end = shape_arrow.end;
    //the line x is the minimum of the start and end x
    let x = start.0.min(end.0);
    let y = start.1.min(end.1);

    session.set_size(
        shape_arrow.entity,
        (end.0 - start.0).abs(),
        (end.1 - start.1).abs(),
    );

    session.set_position(shape_arrow.entity, x, y);
}

/**
 * Updates the size of the ellipse entity based on the horizontal and vertical radius
 * radius.0 is the horizontal radius and radius.1 is the vertical radius
 * The position of the ellipse is the top left corner of the bounding box
 */
pub fn layout_ellipse(session: &mut Session, shape_ellipse: &ShapeEllipse) {
    let w = shape_ellipse.radius.0 * 2.0;
    let h = shape_ellipse.radius.1 * 2.0;
    session.set_size(shape_ellipse.entity, w, h);
}

/**
 * Sets the image entity size to the preferred size
 */
pub fn layout_image(session: &mut Session, shape_image: &ShapeImage) {
    session.set_size(shape_image.entity, shape_image.preferred_size.0, shape_image.preferred_size.1);

}

/**
 * Updates the position of the elements in the vertical stack
 * and the size of the vertical stack
 */
pub fn layout_vertical_stack(session: &mut Session, vertical_stack: &VerticalStack) {
    let mut y = 0.0;
    let mut width = 0.0;
    for elem in vertical_stack.elements.iter() {
        let elem_size = session.get_size(*elem);
        session.set_position(*elem, 0.0, y);
        y += elem_size.1;
        if elem_size.0 > width {
            width = elem_size.0;
        }
    }
    session.set_size(vertical_stack.entity, width, y);

}

pub fn layout_horizontal_stack(session: &mut Session, horizontal_stack: &HorizontalStack) {
    let mut x = 0.0;
    let mut height = 0.0;
    for elem in horizontal_stack.elements.iter() {
        let elem_size = session.get_size(*elem);
        session.set_position(*elem, x, 0.0);
        x += elem_size.0;
        if elem_size.1 > height {
            height = elem_size.1;
        }
    }
    session.set_size(horizontal_stack.entity, x, height);

}
  
