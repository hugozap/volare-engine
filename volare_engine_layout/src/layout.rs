/* Layout calculation for each type of entity */

use crate::{Session, ShapeBox, ShapeGroup, ShapeLine, ShapeText, ShapeEllipse, ShapeArrow, ShapeImage, VerticalStack, HorizontalStack, Table, EntityID};

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

pub fn layout_text(session: &mut Session, shape_text: &ShapeText) {
    // let (w, h) = session.measure_text.unwrap()(&shape_text.text, &shape_text.text_options);
    // session.set_size(shape_text.entity, w, h);
    
    /* for each line in lines, get the size and use it to position the next */
    let mut y = 0.0;
    let mut max_line_width = 0f64;
    for line in shape_text.lines.iter() {
        let line_size = session.measure_text.unwrap()(&line.text, &shape_text.text_options);
        if line_size.0 > max_line_width {
            max_line_width = line_size.0;
        }
        session.set_position(line.entity, 0.0, y);
        session.set_size(line.entity, line_size.0, line_size.1);
        y += line_size.1;
    }
    //set the size of the text element
    session.set_size(shape_text.entity, max_line_width, y);

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

/**
 * Calculates the layout for each of the cells according to table rules:
 * - Cells in the same column have the same width (eq to the max of widths)
 * - Cells in the same row have the same height (eq to the max of heights)
 * - Rows on top of each other
 * - Cols to the right of each other
 * - The sizes of the internal elements should be previously computed for this to work
 */
pub fn layout_table(session: &mut Session, table: &Table) {
    //we need to group elements by row and column, calculate their
    //natural sizes and then update their rows and columns
    let mut rows: Vec<Vec<EntityID>> = Vec::new();
    let mut cols: Vec<Vec<EntityID>> = Vec::new();
    let mut row_heights: Vec<f64> = Vec::new();
    let mut col_widths: Vec<f64> = Vec::new();

    //initialize rows and cols
    for (i, elem) in table.cells.iter().enumerate() {
        let row = i / table.cols;
        let col = i % table.cols;
        //add the element to the row and col
        if row >= rows.len() {
            rows.push(Vec::new());
            row_heights.push(0.0);
        }
        if col >= cols.len() {
            cols.push(Vec::new());
            col_widths.push(0.0);
        }
        rows[row].push(*elem);
        cols[col].push(*elem);
        
        //update the row and col sizes
        let elem_size = session.get_size(*elem);
        if elem_size.0 > col_widths[col] {
            col_widths[col] = elem_size.0;
        }
        if elem_size.1 > row_heights[row] {
            row_heights[row] = elem_size.1;
        }
    }

    //we already have each row and col and their sizes.
    //Now we have to update the position of each element
    //and the size of the table

    //iterate through rows and cols and update the position of each element
    let mut x = 0.0;
    for (i, col) in cols.iter().enumerate() {
        let mut y = 0.0;
        for elem in col.iter() {
            session.set_position(*elem, x, y);
            y += row_heights[i];
        }
        x += col_widths[i];
    }

    //update the size of the table
    let mut width = 0.0;
    let mut height = 0.0;

    for w in col_widths.iter() {
        width += w; 
    }

    for h in row_heights.iter() {
        height += h; 
    }

    session.set_size(table.entity, width, height);

    
}
  
