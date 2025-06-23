/* Layout calculation for each type of entity */

use crate::{
    diagram_builder::DiagramTreeNode, DiagramBuilder, EntityID, EntityType, HorizontalStack,
    ShapeArrow, ShapeBox, ShapeEllipse, ShapeGroup, ShapeImage, ShapeLine, ShapeText, Table,
    VerticalStack, PolyLine, FreeContainer,
};

/* The box layout includes the padding and the dimensions
of the wrapped element
The wrapped element position and size should be updated before calling this function.
The wrapped element position is relative to the box position.
*/
pub fn layout_box(session: &mut DiagramBuilder, shape_box: &ShapeBox) {
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
pub fn layout_group(session: &mut DiagramBuilder, shape_group: &ShapeGroup) {
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

pub fn layout_text(session: &mut DiagramBuilder, shape_text: &ShapeText) {
    // let (w, h) = session.measure_text.unwrap()(&shape_text.text, &shape_text.text_options);
    // session.set_size(shape_text.entity, w, h);
    /* for each line in lines, get the size and use it to position the next */
    {
        println!("Text: {:?}", shape_text);
        let mut y = 0.0;
        let mut max_line_width = 0f64;
        for line in shape_text.lines.iter() {
            println!("Line: {:?}", line);
            let textLine = session.get_text_line(*line);
            let line_size = session.measure_text.unwrap()(&textLine.text, &shape_text.text_options);
            if line_size.0 > max_line_width {
                max_line_width = line_size.0;
            }
            session.set_position(*line, 0.0, y);
            session.set_size(*line, line_size.0, line_size.1);
            y += line_size.1 + shape_text.text_options.line_spacing as f64;
        }
        y-= shape_text.text_options.line_spacing as f64; // Adjust for the last line spacing

        println!("max_line_width: {}", max_line_width);
        //set the size of the text element
        println!(
            "Setting size to text entity: {} - {} {}",
            shape_text.entity, max_line_width, y
        );
        session.set_size(shape_text.entity, max_line_width, y);
    }
}

/**
 * Updates the size of the line entity based on the start and end points
 */
pub fn layout_line(session: &mut DiagramBuilder, shape_line: &ShapeLine) {
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
pub fn layout_arrow(session: &mut DiagramBuilder, shape_arrow: &ShapeArrow) {
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
pub fn layout_ellipse(session: &mut DiagramBuilder, shape_ellipse: &ShapeEllipse) {
    let w = shape_ellipse.radius.0 * 2.0;
    let h = shape_ellipse.radius.1 * 2.0;
    session.set_size(shape_ellipse.entity, w, h);
}

pub fn layout_rect(session: &mut DiagramBuilder, entity: EntityID, width: f64, height: f64) {
    //set the size of the rect
    session.set_size(entity, width, height);

}

/**
 * Sets the image entity size to the preferred size
 */
pub fn layout_image(session: &mut DiagramBuilder, shape_image: &ShapeImage) {
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
pub fn layout_vertical_stack(session: &mut DiagramBuilder, vertical_stack: &VerticalStack) {
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

pub fn layout_horizontal_stack(session: &mut DiagramBuilder, horizontal_stack: &HorizontalStack) {
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
pub fn layout_table(session: &mut DiagramBuilder, table: &Table) {
    //we need to group elements by row and column, calculate their
    //natural sizes and then update their rows and columns
    let mut rows: Vec<Vec<EntityID>> = Vec::new();
    let mut cols: Vec<Vec<EntityID>> = Vec::new();
    let mut row_heights: Vec<f64> = Vec::new();
    let mut col_widths: Vec<f64> = Vec::new();

    // Add variables to store line positions
    let mut horizontal_line_positions: Vec<f64> = Vec::new();
    let mut vertical_line_positions: Vec<f64> = Vec::new();

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
            col_widths[col] = elem_size.0 + table.table_options.cell_padding as f64 * 2.0;
        }
        if elem_size.1 > row_heights[row] {
            row_heights[row] = elem_size.1 + table.table_options.cell_padding as f64 * 2.0;
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
        for (j, elem) in col.iter().enumerate() {
            session.set_position(
                *elem,
                x + table.table_options.cell_padding as f64,
                y + table.table_options.cell_padding as f64,
            );
            y += row_heights[j];
        }

        x += col_widths[i];
    }

    //Update the position of the horizontal lines
    let mut y = 0.0;
    for (i, row) in rows.iter().enumerate() {
        horizontal_line_positions.push(y);
        y += row_heights[i];
    }

    //Update the position of the vertical lines
    let mut x = 0.0;
    for (i, col) in cols.iter().enumerate() {
        vertical_line_positions.push(x);
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

    //Update the size of the table header rect
    session.set_size(table.header_rect, width, row_heights[0]);

    //print the size of the table
    println!("Table size: {:?}", (width, height));

    session.set_size(table.entity, width, height);

    //We need to update the position of the horizontal lines and their size
    for (i, line) in table.row_lines.iter().enumerate() {
        //get the size of the line (should be 0,0 by default)
        let line_size = session.get_size(*line);
        if i < horizontal_line_positions.len() {
            //set the y position of the horizontal line, x will be 0
            session.set_position(*line, 0.0, horizontal_line_positions[i]);
            //update the size, we only need to update the height and leave the width as it is (0 by default)
            session.set_size(*line, width, line_size.1);
        }
    }

    for (i, line) in table.col_lines.iter().enumerate() {
        //get the size of the line (should be 0,0 by default)
        let line_size = session.get_size(*line);
        if i < vertical_line_positions.len() {
            //set the x position of the vertical line, y will be 0
            session.set_position(*line, vertical_line_positions[i], 0.0);
            //update the size, we only need to update the width and leave the height as it is (0 by default)
            session.set_size(*line, line_size.0, height);
        }
    }
}

pub fn layout_polyline(session: &mut DiagramBuilder, polyline: &PolyLine) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = 0.0;
    let mut height = 0.0;
    for (i, point) in polyline.points.iter().enumerate() {
        if i == 0 {
            x = point.0;
            y = point.1;
        } else {
            if point.0 < x {
                width += x - point.0;
                x = point.0;
            }
            if point.1 < y {
                height += y - point.1;
                y = point.1;
            }
        }
    }
    session.set_size(polyline.entity, width, height);
}

/**
 * Layout for the FreeContainer
 * Children have absolute positions relative to the container
 * The container size is determined by the maximum extent of its children
 */
pub fn layout_free_container(session: &mut DiagramBuilder, container: &FreeContainer) {
    // We need to determine the size of the container based on the positions and sizes of its children
    let mut max_width = 0.0;
    let mut max_height = 0.0;
    
    // Iterate through all children and find the maximum extent
    for (child_id, position) in &container.children {
        // Get the child's size
        let child_size = session.get_size(*child_id);
        
        // Set the child's position relative to the container
        session.set_position(*child_id, position.0, position.1);
        
        // Calculate the right and bottom edges of this child
        let right = position.0 + child_size.0;
        let bottom = position.1 + child_size.1;
        
        // Update the maximum extent
        if right > max_width {
            max_width = right;
        }
        if bottom > max_height {
            max_height = bottom;
        }
    }
    
    // Add a small margin to ensure we have enough space
    let margin = 2.0;
    max_width += margin;
    max_height += margin;
    
    // Set the container's size
    session.set_size(container.entity, max_width, max_height);
}


pub struct BoundingBox {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
//Calculate the layout for a tree of elements
pub fn layout_tree_node(session: &mut DiagramBuilder, root: &DiagramTreeNode) -> BoundingBox {
    //start with the bottom elements
    for child in &root.children {
        println!("Layout child: {:?}", child);
        layout_tree_node(session, child);
        //print size and position of the child

        let child_size = session.get_size(child.entity_id);
        let child_pos = session.get_position(child.entity_id);
        println!("Child size: {:?}", child_size);
        println!("Child pos: {:?}", child_pos);
    }

    //Once the children are laid out, we can layout the current element
    //use methods in the layout module
    match root.entity_type {
        EntityType::TextShape => {
            {
                //get the Shape text entity
                let text = session.get_text(root.entity_id).clone();
                layout_text(session, &text);
            }
        }
        EntityType::BoxShape => {
            //get the Shape box entity
            let box_shape = session.get_box(root.entity_id).clone();
            layout_box(session, &box_shape);
        }

        EntityType::RectShape => {
            //get the Rect entity
            let rect = session.get_rectangle(root.entity_id);
            layout_rect(session, root.entity_id, rect.rect_options.width, rect.rect_options.height);
        }
        
        EntityType::LineShape => {
            //get the Shape line entity
            let line = session.get_line(root.entity_id).clone();
            layout_line(session, &line);
        }
        EntityType::ArrowShape => {
            //get the Shape arrow entity
            let arrow = session.get_arrow(root.entity_id).clone();
            layout_arrow(session, &arrow);
        }
        EntityType::EllipseShape => {
            //get the Shape ellipse entity
            let ellipse = session.get_ellipse(root.entity_id).clone();
            layout_ellipse(session, &ellipse);
        }
        EntityType::ImageShape => {
            //get the Shape image entity
            let image = session.get_image(root.entity_id).clone();
            layout_image(session, &image);
        }
        EntityType::VerticalStackShape => {
            //get the VerticalStack entity
            let vertical_stack = session.get_vertical_stack(root.entity_id).clone();
            layout_vertical_stack(session, &vertical_stack);
        }

        EntityType::HorizontalStackShape => {
            //get the HorizontalStack entity
            let horizontal_stack = session.get_horizontal_stack(root.entity_id).clone();
            layout_horizontal_stack(session, &horizontal_stack);
        }

        EntityType::TableShape => {
            //get the Table entity
            let table = session.get_table(root.entity_id).clone();
            layout_table(session, &table);
        }

        EntityType::GroupShape => {
            //get the Group entity
            let group = session.get_group(root.entity_id).clone();
            layout_group(session, &group);
        }

        EntityType::PolyLine => {
            let polyline = session.get_polyline(root.entity_id).clone();
            layout_polyline(session, &polyline);
        }
        EntityType::FreeContainer => {
            let container = session.get_free_container(root.entity_id).clone();
            layout_free_container(session, &container);
        }

        //if not recognized, show the name of it in the panic
        _ => panic!("Unknown entity type: {:?}", root.entity_type),
    }

    //Return the bounding box for the root element
    let size = session.get_size(root.entity_id);
    let position = session.get_position(root.entity_id);
    BoundingBox {
        x: position.0,
        y: position.1,
        width: size.0,
        height: size.1,
    }
}

//import textoptions defined in src/components/mod.rs
use crate::components::BoxOptions;
use crate::components::TextOptions;
//Test that a box with a text inside is correctly laid out
#[test]
fn test_layout_box_with_text() {
    let mut session = DiagramBuilder::new();
    session.set_measure_text_fn(|_, _| (10.0, 10.0));
    let text = session.new_text(
        "hello",
        TextOptions {
            font_size: 20.0,
            line_width: 200,
            ..Default::default()
        },
    );
    let box_options = BoxOptions {
        padding: 10.0,
        ..Default::default()
    };
    let box_shape = session.new_box(text.clone(), box_options.clone());

    //print box options
    println!("--box options: {:?}", box_options);

    //layout the box
    layout_tree_node(&mut session, &box_shape);

    let text_position = session.get_position(text.entity_id);
    let text_size = session.get_size(text.entity_id);

    let box_position = session.get_position(box_shape.entity_id);
    let box_size = session.get_size(box_shape.entity_id);
    //assert equal positions

    // assert the box size is greater than the text size
    println!("box size: {:?}", box_size);
    println!("text size: {:?}", text_size);
    // and the text size should not be zero
    assert!(text_size.0 > 0.0);
    assert_eq!(box_size.0, 30.0);
    assert!(box_size.1 > text_size.1);
}
