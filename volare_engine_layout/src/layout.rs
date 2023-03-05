/* Layout calculation for each type of entity */

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::{
    session::DiagramTreeNode, EntityID, EntityType, HorizontalStack, Session, ShapeArrow, ShapeBox,
    ShapeEllipse, ShapeGroup, ShapeImage, ShapeLine, ShapeText, Table, VerticalStack,TextOptions, BoxOptions
};

/* The box layout includes the padding and the dimensions
of the wrapped element
The wrapped element position and size should be updated before calling this function.
The wrapped element position is relative to the box position.
*/
pub fn layout_box(session: &mut Session, shape_box: &ShapeBox) {
    println!("Box: {:?}", shape_box);
    //get the wrapped element dimensions
    let wrapped_elem_size = session.get_size(shape_box.wrapped_entity);
    println!("Box Wrapped elem size: {:?}", wrapped_elem_size);

    //print element dimensions
    println!(
        "Box: {}, {}, {}, {}",
        shape_box.entity, wrapped_elem_size.0, wrapped_elem_size.1, shape_box.box_options.padding
    );
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
    {
        println!("Text: {:?}", shape_text);
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

        println!("max_line_width: {}", max_line_width);
        //set the size of the text element
        println!("Setting size to text entity: {} - {} {}", shape_text.entity, max_line_width, y);
        session.set_size(shape_text.entity, max_line_width, y);
    }
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
    session.set_size(
        shape_image.entity,
        shape_image.preferred_size.0,
        shape_image.preferred_size.1,
    );
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
            col_widths[col] = elem_size.0 + table.table_options.cell_padding as f64*2.0;
        }
        if elem_size.1 > row_heights[row] {
            row_heights[row] = elem_size.1 + table.table_options.cell_padding as f64*2.0;
        }
    }

    //print row heights and col widths
    println!("row heights: {:?}", row_heights);
    println!("col widths: {:?}", col_widths);

    //we already have each row and col and their sizes.
    //Now we have to update the position of each element
    //and the size of the table

    //iterate through rows and cols and update the position of each element
    let mut x = 0.0;
    for (i, col) in cols.iter().enumerate() {
        let mut y = 0.0;
        for elem in col.iter() {
            session.set_position(*elem, x + table.table_options.cell_padding as f64, y + table.table_options.cell_padding as f64);
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
    
    //print the size of the table
    println!("Table size: {:?}", (width, height));

    session.set_size(table.entity, width, height);
}


pub struct BoundingBox {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
//Calculate the layout for a tree of elements
pub fn layout_tree_node(session: &mut Session, root: &DiagramTreeNode) -> BoundingBox {

    //start with the bottom elements
    for child in &root.children {
        println!("Layout child: {:?}", child);
        layout_tree_node(session, child);
        //print size and position of the child
        let child_id = session.get_entity_id(child.entity_type, child.index);
        let child_size = session.get_size(child_id);
        let child_pos = session.get_position(child_id);
        println!("Child size: {:?}", child_size);
        println!("Child pos: {:?}", child_pos);
    }

    //Once the children are laid out, we can layout the current element
    //use methods in the layout module
    match root.entity_type {
        EntityType::TextShape => {
            {
    
                //get the Shape text entity
                let text = session.get_text(root.index).clone();
                layout_text(session, &text);
            }
        }
        EntityType::BoxShape => {
            //get the Shape box entity
            let box_shape = session.get_box(root.index).clone();
            layout_box(session, &box_shape);
        }
        EntityType::LineShape => {
            //get the Shape line entity
            let line = session.get_line(root.index).clone();
            layout_line(session, &line);
        }
        EntityType::ArrowShape => {
            //get the Shape arrow entity
            let arrow = session.get_arrow(root.index).clone();
            layout_arrow(session, &arrow);
        }
        EntityType::EllipseShape => {
            //get the Shape ellipse entity
            let ellipse = session.get_ellipse(root.index).clone();
            layout_ellipse(session, &ellipse);
        }
        EntityType::ImageShape => {
            //get the Shape image entity
            let image = session.get_image(root.index).clone();
            layout_image(session, &image);
        }
        EntityType::VerticalStackShape => {
            //get the VerticalStack entity
            let vertical_stack = session.get_vertical_stack(root.index).clone();
            layout_vertical_stack(session, &vertical_stack);
        }

        EntityType::HorizontalStackShape => {
            //get the HorizontalStack entity
            let horizontal_stack = session.get_horizontal_stack(root.index).clone();
            layout_horizontal_stack(session, &horizontal_stack);
        }

        EntityType::TableShape => {
            //get the Table entity
            let table = session.get_table(root.index).clone();
            layout_table(session, &table);
        }

        EntityType::GroupShape => {
            //get the Group entity
            let group = session.get_group(root.index).clone();
            layout_group(session, &group);
        }
        //if not recognized, show the name of it in the panic
        _ => panic!("Unknown entity type: {:?}", root.entity_type),
    }

    //Return the bounding box for the root element
    let entity_id = session.get_entity_id(root.entity_type, root.index);
    let size = session.get_size(entity_id);
    let position = session.get_position(entity_id);
    BoundingBox {
        x: position.0,
        y: position.1,
        width: size.0,
        height: size.1,
    }
}


//Test that a box with a text inside is correctly laid out
#[test]
fn test_layout_box_with_text() {
    let mut session = Session::new();
    session.set_measure_text_fn(|_, _| (10.0, 10.0));
    let text = session.new_text("hello", TextOptions{
        font_size: 20.0,
        line_width: 200,
        ..Default::default()
    });
    let box_options = BoxOptions{
        padding: 10.0,
        ..Default::default()
    };
    let box_shape = session.new_box(text.clone(), box_options.clone() );

    //print box options
    println!("--box options: {:?}", box_options);


    //layout the box
    layout_tree_node(&mut session, &box_shape);

    let text_id = session.get_entity_id(EntityType::TextShape, text.index);
    let box_id = session.get_entity_id(EntityType::BoxShape, box_shape.index);

    let text_position = session.get_position(text_id);
    let text_size = session.get_size(text_id);

    let box_position = session.get_position(box_id);
    let box_size = session.get_size(box_id);
    //assert equal positions


    // assert the box size is greater than the text size
    println!("box size: {:?}", box_size);
    println!("text size: {:?}", text_size);
        // and the text size should not be zero
        assert!(text_size.0 > 0.0);
    assert_eq!(box_size.0, 30.0);
    assert!(box_size.1 > text_size.1);





}