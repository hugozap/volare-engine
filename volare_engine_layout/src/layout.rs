/* Layout calculation for each type of entity */

use std::f32::{INFINITY, NEG_INFINITY};

use crate::components::Float;
use crate::{
    diagram_builder::DiagramTreeNode, DiagramBuilder, EntityID, EntityType, FreeContainer,
    HorizontalStack, PolyLine, ShapeArrow, ShapeBox, ShapeEllipse, ShapeGroup, ShapeImage,
    ShapeLine, ShapeText, Table, VerticalStack,
};
use crate::{
    ConstraintLayoutContainer, ConstraintSystem, HorizontalAlignment, Point, ShapeArc, ShapeRect,
    ShapeSpacer, SizeBehavior, SpacerDirection, TextLine, VerticalAlignment,
};

use crate::transform::Transform;

/* The box layout includes the padding and the dimensions
of the wrapped element
The wrapped element position and size should be updated before calling this function
(except for grow behavior).
The wrapped element position is relative to the box position.
*/
pub fn layout_box(session: &mut DiagramBuilder, shape_box: &ShapeBox) {
    println!("Box: {:?}", shape_box);

    // Get the wrapped element dimensions
    let mut wrapped_elem_bounds = session.get_effective_bounds(shape_box.wrapped_entity.clone());

    // Calculate the box dimensions based on size behavior
    let box_width = match shape_box.box_options.width_behavior {
        SizeBehavior::Fixed(width) => {
            // For fixed width, use the specified width
            width
        }
        SizeBehavior::Content => {
            // Content sizing - size based on wrapped element + padding
            wrapped_elem_bounds.width + shape_box.box_options.padding * 2.0
        }
        SizeBehavior::Grow => {
            // TODO: Implement grow behavior in future iterations
            // For now, fall back to content behavior
            wrapped_elem_bounds.width + shape_box.box_options.padding * 2.0
        }
    };

    // Auto-wrap text if box has fixed width (do this BEFORE positioning)
    if let SizeBehavior::Fixed(fixed_width) = shape_box.box_options.width_behavior {
        let wrapped_entity_type = session.entityTypes.get(&shape_box.wrapped_entity);
        if let Some(EntityType::TextShape) = wrapped_entity_type {
            let available_width = fixed_width - shape_box.box_options.padding * 2.0;
            auto_wrap_text_in_box(session, &shape_box.wrapped_entity, available_width);
            // Re-get bounds after text wrapping
            wrapped_elem_bounds = session.get_effective_bounds(shape_box.wrapped_entity.clone());
        }
    }

    // Calculate the box height (after potential text wrapping)
    let box_height = match shape_box.box_options.height_behavior {
        SizeBehavior::Fixed(height) => height,
        SizeBehavior::Content => wrapped_elem_bounds.height + shape_box.box_options.padding * 2.0,
        SizeBehavior::Grow => wrapped_elem_bounds.height + shape_box.box_options.padding * 2.0,
    };

    // Calculate where we want the wrapped element's bounding box to be positioned
    let desired_content_x = match shape_box.box_options.width_behavior {
        SizeBehavior::Fixed(fixed_width) => {
            let available_width = fixed_width - shape_box.box_options.padding * 2.0;
            if wrapped_elem_bounds.width <= available_width
                && shape_box.box_options.horizontal_alignment == HorizontalAlignment::Center
            {
                // Center the content bounding box
                shape_box.box_options.padding + (available_width - wrapped_elem_bounds.width) / 2.0
            } else {
                shape_box.box_options.padding
            }
        }
        _ => {
            // For content and grow behaviors, position bounding box at padding
            shape_box.box_options.padding
        }
    };

    let desired_content_y = match shape_box.box_options.height_behavior {
        SizeBehavior::Fixed(fixed_height) => {
            let available_height = fixed_height - shape_box.box_options.padding * 2.0;
            if wrapped_elem_bounds.height <= available_height {
                // Center the content bounding box
                shape_box.box_options.padding
                    + (available_height - wrapped_elem_bounds.height) / 2.0
            } else {
                // Content is larger than available space, align bounding box to top edge
                shape_box.box_options.padding
            }
        }
        _ => {
            // For content and grow behaviors, position bounding box at padding
            shape_box.box_options.padding
        }
    };

    // FIXED: Apply bounding box compensation when positioning the wrapped element
    let transform_x = desired_content_x - wrapped_elem_bounds.x;
    let transform_y = desired_content_y - wrapped_elem_bounds.y;

    session.set_position(shape_box.wrapped_entity.clone(), transform_x, transform_y);

    println!(
        "Box: {}, width: {}, height: {}, padding: {}, content positioned at: ({}, {})",
        shape_box.entity,
        box_width,
        box_height,
        shape_box.box_options.padding,
        transform_x,
        transform_y
    );

    // Set the box dimensions
    session.set_size(shape_box.entity.clone(), box_width, box_height);
}

fn calculate_optimal_line_width(
    session: &DiagramBuilder,
    text: &str,
    text_options: &TextOptions,
    available_width: Float,
) -> usize {
    // Binary search for optimal line_width
    let mut min_width = 10;
    let mut max_width = text.len();
    let mut best_width = min_width;

    while min_width <= max_width {
        let mid_width = (min_width + max_width) / 2;

        // Test this line_width
        let mut test_options = text_options.clone();
        test_options.line_width = mid_width;

        let wrapped_lines = textwrap::wrap(text, mid_width);
        if wrapped_lines.is_empty() {
            break;
        }

        // Measure the widest line
        let max_line_width = wrapped_lines
            .iter()
            .map(|line| session.measure_text.unwrap()(line, &test_options).0)
            .fold(0.0f32, |a, b| a.max(b));

        if max_line_width <= available_width {
            best_width = mid_width;
            min_width = mid_width + 1; // Try wider
        } else {
            max_width = mid_width - 1; // Try narrower
        }
    }

    best_width
}

fn auto_wrap_text_in_box(
    session: &mut DiagramBuilder,
    text_entity_id: &EntityID,
    available_width: Float,
) {
    // Get the current text shape
    let text_shape = session.get_text(text_entity_id.clone()).clone();

    // Calculate optimal line_width using actual text measurement
    let new_line_width = calculate_optimal_line_width(
        session,
        &text_shape.text,
        &text_shape.text_options,
        available_width,
    );
    // Only re-layout if line_width changed significantly
    if (new_line_width as i32 - text_shape.text_options.line_width as i32).abs() > 5 {
        // Create new text options with updated line_width
        let mut new_text_options = text_shape.text_options.clone();
        new_text_options.line_width = new_line_width;

        // Re-create text lines with new wrapping
        let text_lines = textwrap::wrap(&text_shape.text, new_line_width);
        let mut new_lines = Vec::new();

        // Update existing lines or create new ones
        for (i, line_text) in text_lines.iter().enumerate() {
            let line_id = if i < text_shape.lines.len() {
                // Reuse existing line
                let existing_line_id = text_shape.lines[i].clone();
                let existing_line = session.get_text_line_mut(existing_line_id.clone());
                if let Some(existing_line) = existing_line {
                    existing_line.text = line_text.to_string();
                } else {
                    println!(
                        "Warning: Text line ID {} not found in session",
                        existing_line_id
                    );
                    continue;
                }
                existing_line_id
            } else {
                // Create new line
                // Note: here we are creating new elements on layout
                let line_id = format!("{}-autowrap-line-{}", text_entity_id, i);
                session.new_entity(line_id.clone(), EntityType::TextLine);
                let text_line = TextLine {
                    entity: line_id.clone(),
                    text: line_text.to_string(),
                };
                session.add_text_line(line_id.clone(), text_line);
                line_id
            };
            new_lines.push(line_id);
        }

        // Update the text shape with new options and lines
        let updated_text_shape = ShapeText {
            entity: text_shape.entity.clone(),
            text: text_shape.text.clone(),
            text_options: new_text_options,
            lines: new_lines,
        };

        // Update the session with the new text shape
        session.add_text(text_entity_id.clone(), updated_text_shape.clone());

        // Re-layout the text with new dimensions
        layout_text(session, &updated_text_shape);
    }
}

// Helper function to estimate character width based on font
fn estimate_char_width(text_options: &TextOptions) -> Float {
    // Rough estimation: font_size * 0.6 for typical fonts
    // You could make this more sophisticated based on font_family
    text_options.font_size * 0.6
}

/**
 * Update the group size based on the size of the elements.
 * Group elements must be positioned before calling this function.
 * (Doesn't update the position of the elements)
 */

pub fn layout_group(session: &mut DiagramBuilder, shape_group: &ShapeGroup) {
    // Calculate actual bounding box using positions, not just max dimensions
    let mut min_x = Float::INFINITY;
    let mut min_y = Float::INFINITY;
    let mut max_x = Float::NEG_INFINITY;
    let mut max_y = Float::NEG_INFINITY;

    for elem in shape_group.elements.iter() {
        let elem_bounds = session.get_effective_bounds(elem.clone());
        let elem_pos = session.get_position(elem.clone()); // Uses transforms behind the scenes

        min_x = min_x.min(elem_pos.0);
        min_y = min_y.min(elem_pos.1);
        max_x = max_x.max(elem_pos.0 + elem_bounds.width);
        max_y = max_y.max(elem_pos.1 + elem_bounds.height);
    }

    if min_x != Float::INFINITY {
        let width = max_x - min_x;
        let height = max_y - min_y;
        session.set_size(shape_group.entity.clone(), width, height);

        // use bounding box compensation since set_position works with transform origins
        let group_bounds = session.get_effective_bounds(shape_group.entity.clone());
        let transform_x = min_x - group_bounds.x;
        let transform_y = min_y - group_bounds.y;
        session.set_position(shape_group.entity.clone(), transform_x, transform_y);
    } else {
        session.set_size(shape_group.entity.clone(), 0.0, 0.0);
        session.set_position(shape_group.entity.clone(), 0.0, 0.0);
    }
}

// WHY this function doesn't need the bounding box compensation:
//
// 1. Text lines don't have individual rotations - they're just positioned within the text entity
// 2. The rotation transform is applied to the parent text entity as a whole
// 3. Individual lines are positioned relative to (0,0) within the text entity
// 4. When the text entity is rotated, all lines rotate together as one unit
//
// Transform hierarchy:
// Text Entity (has rotation transform)
//   └── Line 1 (positioned at 0, 0 relative to text entity)
//   └── Line 2 (positioned at 0, 16 relative to text entity)
//   └── Line 3 (positioned at 0, 32 relative to text entity)
pub fn layout_text(session: &mut DiagramBuilder, shape_text: &ShapeText) {
    let mut y = 0.0;
    let mut max_line_width = 0.0;

    for (i, line) in shape_text.lines.iter().enumerate() {
        let textLine = session.get_text_line(line.clone());
        let line_size = session.measure_text.unwrap()(&textLine.text, &shape_text.text_options);

        if line_size.0 > max_line_width {
            max_line_width = line_size.0;
        }

        session.set_position(line.clone(), 0.0, y);
        session.set_size(line.clone(), line_size.0, line_size.1);

        // Add line height
        y += line_size.1;

        // Add line spacing ONLY between lines (not after the last line)
        if i < shape_text.lines.len() - 1 {
            y += shape_text.text_options.line_spacing;
        }
    }

    // No need to subtract line spacing at the end
    session.set_size(shape_text.entity.clone(), max_line_width, y);
}

pub fn layout_spacer(session: &mut DiagramBuilder, spacer: &ShapeSpacer) {
    let (width, height) = match spacer.spacer_options.direction {
        SpacerDirection::Horizontal => (spacer.spacer_options.width, 1.0),
        SpacerDirection::Vertical => (1.0, spacer.spacer_options.height),
        SpacerDirection::Both => (spacer.spacer_options.width, spacer.spacer_options.height),
    };

    session.set_size(spacer.entity.clone(), width, height);
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
        shape_line.entity.clone(),
        (end.0 - start.0).abs(),
        (end.1 - start.1).abs(),
    );

    session.set_position(shape_line.entity.clone(), x, y);
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
        shape_arrow.entity.clone(),
        (end.0 - start.0).abs(),
        (end.1 - start.1).abs(),
    );

    session.set_position(shape_arrow.entity.clone(), x, y);
}

/**
 * Updates the size of the ellipse entity based on the horizontal and vertical radius
 * radius.0 is the horizontal radius and radius.1 is the vertical radius
 * The position of the ellipse is the top left corner of the bounding box
 */
pub fn layout_ellipse(session: &mut DiagramBuilder, shape_ellipse: &ShapeEllipse) {
    // Only set the size based on radius
    let width = shape_ellipse.radius.0 * 2.0;
    let height = shape_ellipse.radius.1 * 2.0;
    session.set_size(shape_ellipse.entity.clone(), width, height);
}

pub fn layout_rect(session: &mut DiagramBuilder, rect: &ShapeRect) {
    // If the rect has a fixed size, use that
    let width = match rect.rect_options.width_behavior {
        SizeBehavior::Fixed(w) => w,
        _ => 0.0,
    };
    let height = match rect.rect_options.height_behavior {
        SizeBehavior::Fixed(h) => h,
        _ => 0.0,
    };

    session.set_size(rect.entity.clone(), width, height);
}

/**
 * Sets the image entity size to the preferred size
 */
pub fn layout_image(session: &mut DiagramBuilder, shape_image: &ShapeImage) {
    let width = match shape_image.width_behavior {
        SizeBehavior::Fixed(val) => val,
        SizeBehavior::Content => 100.0, // TODO: Obtener size de la data de la imagen
        _ => 100.0,
    };

    let height = match shape_image.height_behavior {
        SizeBehavior::Fixed(val) => val,
        SizeBehavior::Content => 100.0, // TODO: Obtener size de la data de la imagen
        _ => 100.0,
    };

    session.set_size(shape_image.entity.clone(), width, height);
}
/**
 * Updates the position of the elements in the vertical stack
 * and the size of the vertical stack
 */
pub fn layout_vertical_stack(session: &mut DiagramBuilder, vertical_stack: &VerticalStack) {
    let mut logical_y = 0.0; // Where we want each element's bounding box to start
    let mut width = 0.0;

    for elem in vertical_stack.elements.iter() {
        println!("DEBUG:::y: {}", logical_y);
        let elem_bounds = session.get_effective_bounds(elem.clone());

        // FIXED: Position the element so its bounding box starts at logical_y
        let transform_y = logical_y - elem_bounds.y;
        session.set_position(elem.clone(), 0.0, transform_y);

        // FIXED: Add the effective height to logical_y for next element
        logical_y += elem_bounds.height;

        if elem_bounds.width > width {
            width = elem_bounds.width;
        }
    }

    // Set the stack size to the total logical height
    session.set_size(vertical_stack.entity.clone(), width, logical_y);

    // Second pass: adjust x positions for horizontal alignment
    for elem in vertical_stack.elements.iter() {
        // FIXED: Use effective bounds consistently for alignment calculations
        let elem_bounds = session.get_effective_bounds(elem.clone());
        let current_pos = session.get_position(elem.clone());

        let x = match vertical_stack.horizontal_alignment {
            HorizontalAlignment::Left => -elem_bounds.x, // Compensate for bounding box offset
            HorizontalAlignment::Center => (width - elem_bounds.width) / 2.0 - elem_bounds.x,
            HorizontalAlignment::Right => width - elem_bounds.width - elem_bounds.x,
        };

        session.set_position(elem.clone(), x, current_pos.1); // Update x, keep y
    }
}

pub fn layout_horizontal_stack(session: &mut DiagramBuilder, horizontal_stack: &HorizontalStack) {
    let mut logical_x = 0.0; // Where we want each element's bounding box to start
    let mut height = 0.0;

    for elem in horizontal_stack.elements.iter() {
        let elem_bounds = session.get_effective_bounds(elem.clone());

        // FIXED: Position the element so its bounding box starts at logical_x
        let transform_x = logical_x - elem_bounds.x;
        session.set_position(elem.clone(), transform_x, 0.0);

        // FIXED: Add the effective width to logical_x for next element
        logical_x += elem_bounds.width;

        if elem_bounds.height > height {
            height = elem_bounds.height;
        }
    }

    // Set the stack size to the total logical width
    session.set_size(horizontal_stack.entity.clone(), logical_x, height);

    // Second pass: adjust y positions for vertical alignment
    for elem in horizontal_stack.elements.iter() {
        // FIXED: Use effective bounds consistently for alignment calculations
        let elem_bounds = session.get_effective_bounds(elem.clone());
        let current_pos = session.get_position(elem.clone());

        let y = match horizontal_stack.vertical_alignment {
            VerticalAlignment::Top => -elem_bounds.y, // Compensate for bounding box offset
            VerticalAlignment::Center => (height - elem_bounds.height) / 2.0 - elem_bounds.y,
            VerticalAlignment::Bottom => height - elem_bounds.height - elem_bounds.y,
        };

        session.set_position(elem.clone(), current_pos.0, y);
    }
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
    let mut row_heights: Vec<Float> = Vec::new();
    let mut col_widths: Vec<Float> = Vec::new();

    // Add variables to store line positions
    let mut horizontal_line_positions: Vec<Float> = Vec::new();
    let mut vertical_line_positions: Vec<Float> = Vec::new();

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
        rows[row].push(elem.clone());
        cols[col].push(elem.clone());

        //update the row and col sizes
        let elem_bounds = session.get_effective_bounds(elem.clone());

        let content_width = elem_bounds.width + table.table_options.cell_padding as Float * 2.0;
        let content_height = elem_bounds.height + table.table_options.cell_padding as Float * 2.0;
        if content_width > col_widths[col] {
            col_widths[col] = content_width;
        }
        if content_height > row_heights[row] {
            row_heights[row] = content_height;
        }
    }

    //print row heights and col widths
    println!("row heights: {:?}", row_heights);
    println!("col widths: {:?}", col_widths);

    //we already have each row and col and their sizes.
    //Now we have to update the position of each element
    //and the size of the table

    //iterate through rows and cols and update the position of each element
    let mut logical_x = 0.0;
    for (i, col) in cols.iter().enumerate() {
        let mut logical_y = 0.0;
        for (j, elem) in col.iter().enumerate() {
            let elem_bounds = session.get_effective_bounds(elem.clone());

            // Calculate where we want the element's bounding box to be (with padding)
            let desired_x = logical_x + table.table_options.cell_padding as Float;
            let desired_y = logical_y + table.table_options.cell_padding as Float;

            // Compensate for the element's bounding box offset
            let transform_x = desired_x - elem_bounds.x;
            let transform_y = desired_y - elem_bounds.y;

            // FIXED: Instead of overwriting position, add translation to existing transform
            let current_transform = session.get_transform(elem.clone());
            let position_transform = Transform::translation(transform_x, transform_y);
            let new_transform = current_transform.combine(&position_transform);
            session.set_transform(elem.clone(), new_transform);

            logical_y += row_heights[j];
        }
        logical_x += col_widths[i];
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
    let width: Float = col_widths.iter().sum();
    let height: Float = row_heights.iter().sum();

    //Update the size of the table header rect
    if let Some(header_rect) = &table.header_rect {
        session.set_size(header_rect.clone(), width, row_heights[0]);
    }

    //print the size of the table
    println!("Table size: {:?}", (width, height));

    session.set_size(table.entity.clone(), width, height);

    //We need to update the position of the horizontal lines and their size
    for (i, line_id) in table.row_lines.iter().enumerate() {
        //get the size of the line (should be 0,0 by default)
        let line_size = session.get_size(line_id.clone());
        if i < horizontal_line_positions.len() {
            let line = session.get_line_mut(line_id.clone());
            let line_w:Float;
            let line_h: Float;
            if let Some(line) = line {
                line.start = (0.0, horizontal_line_positions[i]);
                line.end = (width, horizontal_line_positions[i]);
                line_w = line.end.0 - line.start.0;
                line_h = line.end.1 - line.start.1;
                // correct size of line
                session.set_size(
                    line_id.clone(),
                    line_w,
                    line_h
                )
            }
        }
    }

    for (i, line_id) in table.col_lines.iter().enumerate() {
        //get the size of the line (should be 0,0 by default)
        if i < vertical_line_positions.len() {
            let line = session.get_line_mut(line_id.clone());
            let line_w:Float;
            let line_h: Float;
            if let Some(line) = line {
                line.start = (vertical_line_positions[i], 0.0);
                line.end = (vertical_line_positions[i],height);
                line_w = line.end.0 - line.start.0;
                line_h = line.end.1 - line.start.1;
                // correct size of line
                session.set_size(
                    line_id.clone(),
                    line_w,
                    line_h
                )
            }
        }
    }
}

pub fn layout_polyline(session: &mut DiagramBuilder, polyline: &PolyLine) {
    if polyline.points.is_empty() {
        session.set_size(polyline.entity.clone(), 0.0, 0.0);
        return;
    }

    // Find the actual bounding box of all points
    let mut min_x = Float::INFINITY;
    let mut min_y = Float::INFINITY;
    let mut max_x = Float::NEG_INFINITY;
    let mut max_y = Float::NEG_INFINITY;

    for point in polyline.points.iter() {
        min_x = min_x.min(point.0);
        min_y = min_y.min(point.1);
        max_x = max_x.max(point.0);
        max_y = max_y.max(point.1);
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    // Set the polyline size to its actual bounding box
    session.set_size(polyline.entity.clone(), width, height);
}

/**
 * Layout for the FreeContainer
 * Children have absolute positions relative to the container
 * The container size is determined by the maximum extent of its children
*/
pub fn layout_free_container(session: &mut DiagramBuilder, container: &FreeContainer) {
    let mut max_width = 0.0;
    let mut max_height = 0.0;

    for (child_id, desired_position) in &container.children {
        // TODO: This can be set on creation time
        session.set_position(child_id.clone(), desired_position.0, desired_position.1);

        // FIX: Use effective bounds instead of raw size
        let child_bounds = session.get_effective_bounds(child_id.clone());

        // Calculate the extent based on position + effective bounds dimensions
        let right = desired_position.0 + child_bounds.width; // Use width from bounds
        let bottom = desired_position.1 + child_bounds.height; // Use height from bounds

        if right > max_width {
            max_width = right;
        }
        if bottom > max_height {
            max_height = bottom;
        }
    }

    session.set_size(container.entity.clone(), max_width, max_height);
}

pub fn layout_arc(session: &mut DiagramBuilder, shape_arc: &ShapeArc) {
    let diameter = shape_arc.radius * 2.0;
    session.set_size(shape_arc.entity.clone(), diameter, diameter);
}

//TODO: Hay que cambiar, layout no debe crear el constraint system
// el constraint system se registra en el builder
pub fn layout_constraint_container(
    session: &mut DiagramBuilder,
    container: &ConstraintLayoutContainer,
) -> anyhow::Result<()> {
    println!("layout_constraint_container called");
    let child_sizes: Vec<(String, (Float, Float))> = container
        .children
        .iter()
        .filter_map(|child_id| Some((child_id.clone(), session.get_size(child_id.clone()).clone())))
        .collect();

    let system = session.get_constraint_system_mut(container.entity.clone());

    // Use existing sizes as suggestions
    // at this point children already have their sizes calculated
    for (child_id, (w, h)) in child_sizes {
        system
            .suggest_size(child_id.as_str(), w, h)
            .map_err(|e| anyhow::anyhow!("Failed to suggest size for entity {:?}", e))?;
    }

    // Solve constraints
    let results = system.solve()?;

    // Apply results
    let mut container_width = 0.0;
    let mut container_height = 0.0;

    // Negative positions for children are problematic, we compensate by adding an offset
    // if the element that's most to the left has -10, all elements will be added 10 to x

    let mut min_x: Float = INFINITY;
    let mut min_y: Float = INFINITY;

    for (_, (x, y, _, _)) in results.clone() {
        min_x = min_x.min(x);
        min_y = min_y.min(y)
    }

    let offset_x = if min_x < 0.0 { min_x.abs() } else { 0.0 };

    let offset_y = if min_y < 0.0 { min_y.abs() } else { 0.0 };

    println!("offset_x: {}", offset_x);
    println!("offset_y: {}", offset_y);

    // Set final position and size for children
    for (entity_id, (x, y, width, height)) in results {
        println!(
            "constraint variable: {}, x:{}, y:{}, w:{}, h:{}",
            entity_id.clone(),
            x,
            y,
            width,
            height
        );
        session.set_position(entity_id.clone(), x + offset_x, y + offset_y);
        session.set_size(entity_id.clone(), width, height);

        let right = x + offset_x + width;
        let bottom = y + offset_y + height;
        if right > container_width {
            container_width = right;
        }

        if bottom > container_height {
            container_height = bottom;
        }
    }

    println!(
        "container size calculated: {},{}",
        container_width, container_height
    );
    session.set_size(container.entity.clone(), container_width, container_height);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub x: Float,
    pub y: Float,
    pub width: Float,
    pub height: Float,
}

//Calculate the layout for a tree of elements
pub fn layout_tree_node(session: &mut DiagramBuilder, root: &DiagramTreeNode) -> BoundingBox {
    //start with the bottom elements
    for child in &root.children {
        println!("Layout child: {:?}", child);
        layout_tree_node(session, child);
        //print size and position of the child

        let child_size = session.get_size(child.entity_id.clone());
        let child_pos = session.get_position(child.entity_id.clone());
        println!("Child size: {:?}", child_size);
        println!("Child pos: {:?}", child_pos);
    }

    //Once the children are laid out, we can layout the current element
    //use methods in the layout module
    match root.entity_type {
        EntityType::SpacerShape => {
            let spacer = session.get_spacer(root.entity_id.clone()).clone();
            layout_spacer(session, &spacer);
        }
        EntityType::TextShape => {
            {
                //get the Shape text entity
                let text = session.get_text(root.entity_id.clone()).clone();
                layout_text(session, &text);
            }
        }
        EntityType::BoxShape => {
            //get the Shape box entity
            let box_shape = session.get_box(root.entity_id.clone()).clone();
            layout_box(session, &box_shape);
        }

        EntityType::RectShape => {
            //get the Rect entity
            let rect = session.get_rectangle(root.entity_id.clone()).clone();
            layout_rect(session, &rect);
        }

        EntityType::LineShape => {
            //get the Shape line entity
            let line = session.get_line(root.entity_id.clone()).clone();
            layout_line(session, &line);
        }
        EntityType::ArrowShape => {
            //get the Shape arrow entity
            let arrow = session.get_arrow(root.entity_id.clone()).clone();
            layout_arrow(session, &arrow);
        }
        EntityType::EllipseShape => {
            //get the Shape ellipse entity
            let ellipse = session.get_ellipse(root.entity_id.clone()).clone();
            layout_ellipse(session, &ellipse);
        }
        EntityType::ImageShape => {
            //get the Shape image entity
            let image = session.get_image(root.entity_id.clone()).clone();
            layout_image(session, &image);
        }
        EntityType::VerticalStackShape => {
            //get the VerticalStack entity
            let vertical_stack = session.get_vertical_stack(root.entity_id.clone()).clone();
            layout_vertical_stack(session, &vertical_stack);
        }

        EntityType::HorizontalStackShape => {
            //get the HorizontalStack entity
            let horizontal_stack = session.get_horizontal_stack(root.entity_id.clone()).clone();
            layout_horizontal_stack(session, &horizontal_stack);
        }

        EntityType::TableShape => {
            //get the Table entity
            let table = session.get_table(root.entity_id.clone()).clone();
            layout_table(session, &table);
        }

        EntityType::GroupShape => {
            //get the Group entity
            let group = session.get_group(root.entity_id.clone()).clone();
            layout_group(session, &group);
        }

        EntityType::PolyLine => {
            let polyline = session.get_polyline(root.entity_id.clone()).clone();
            layout_polyline(session, &polyline);
        }
        EntityType::FreeContainer => {
            let container = session.get_free_container(root.entity_id.clone()).clone();
            layout_free_container(session, &container);
        }

        EntityType::ConstraintLayoutContainer => {
            let container = session
                .get_constraint_layout(root.entity_id.clone())
                .clone();
            layout_constraint_container(session, &container);
        }

        EntityType::ArcShape => {
            let arc = session.get_arc(root.entity_id.clone()).clone();
            layout_arc(session, &arc);
        }

        //if not recognized, show the name of it in the panic
        _ => panic!("Unknown entity type: {:?}", root.entity_type),
    }

    session.get_effective_bounds(root.entity_id.clone())
}

//import textoptions defined in src/components/mod.rs
use crate::components::{BoxOptions, TextOptions};
//Test that a box with a text inside is correctly laid out
#[test]
fn test_layout_box_with_text() {
    let mut session = DiagramBuilder::new();
    session.set_measure_text_fn(|_, _| (10.0, 10.0));
    let text = session.new_text(
        "testid".to_string(),
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
    let box_shape = session.new_box("testbox".to_string(), text.clone(), box_options.clone());

    //print box options
    println!("--box options: {:?}", box_options);

    //layout the box
    layout_tree_node(&mut session, &box_shape);

    let text_position = session.get_position(text.entity_id.clone());
    let text_size = session.get_size(text.entity_id.clone());

    let box_position = session.get_position(box_shape.entity_id.clone());
    let box_size = session.get_size(box_shape.entity_id.clone());
    //assert equal positions

    // assert the box size is greater than the text size
    println!("box size: {:?}", box_size);
    println!("text size: {:?}", text_size);
    // and the text size should not be zero
    assert!(text_size.0 > 0.0);
    assert_eq!(box_size.0, 30.0);
    assert!(box_size.1 > text_size.1);
}

#[test]
fn test_box_fixed_size() {
    let mut session = DiagramBuilder::new();
    session.set_measure_text_fn(|_, _| (10.0, 10.0));
    let text = session.new_text(
        "testid".to_string(),
        "hello",
        TextOptions {
            font_size: 20.0,
            line_width: 200,
            ..Default::default()
        },
    );
    let box_options = BoxOptions {
        padding: 10.0,
        width_behavior: SizeBehavior::Fixed(100.0),
        height_behavior: SizeBehavior::Fixed(50.0),
        ..Default::default()
    };
    let box_shape = session.new_box("testbox".to_string(), text.clone(), box_options.clone());

    //layout the box
    layout_tree_node(&mut session, &box_shape);

    let text_position = session.get_position(text.entity_id.clone());
    let text_size = session.get_size(text.entity_id.clone());

    let box_position = session.get_position(box_shape.entity_id.clone());
    let box_size = session.get_size(box_shape.entity_id.clone());

    //assert equal positions
    // Assert that the text is centered within the box
    assert_eq!(
        text_position.0,
        box_options.padding
            + (box_options.width_behavior.unwrap_fixed().unwrap()
                - box_options.padding * 2.0
                - text_size.0)
                / 2.0
    );
    assert_eq!(
        text_position.1,
        box_options.padding
            + (box_options.height_behavior.unwrap_fixed().unwrap()
                - box_options.padding * 2.0
                - text_size.1)
                / 2.0
    );
    assert_eq!(box_position, (0.0, 0.0));

    // assert the box size is equal to the fixed size
    assert_eq!(box_size.0, 100.0);
    assert_eq!(box_size.1, 50.0);
}
