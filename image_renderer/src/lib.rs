use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use bresenham::Bresenham;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use volare_engine_layout::Float;

use volare_engine_layout::{
    diagram_builder::DiagramTreeNode, DiagramBuilder, EntityID, EntityType, Fill, Renderer,
    RendererError,
};

/**
 * This is the PNG renderer. It will render the diagram to a PNG stream.
 */

pub struct PNGRenderer;

impl<W: Write> Renderer<W> for PNGRenderer {
    fn render(
        &self,
        session: &DiagramBuilder,
        diagram_node: &DiagramTreeNode,
        stream: &mut W,
    ) -> Result<(), RendererError> {
        let root_size = session.get_size(diagram_node.entity_id.clone());

        // Use a scaling factor for higher resolution output but don't scale too much
        // 1.5 is a good balance between quality and maintaining layout proportions
        // TODO: Esto debe ser un parametro?
        let scaling_factor = 1.5;

        // Calculate image dimensions with scaling
        let width = ((root_size.0 * scaling_factor).ceil() as u32).max(200);
        let height = ((root_size.1 * scaling_factor).ceil() as u32).max(200);

        // Debug output
        println!("Creating PNG image with dimensions: {}x{}", width, height);

        // Create an image with a white background
        let mut imgbuf = RgbaImage::from_fn(width, height, |_, _| Rgba([255, 255, 255, 255]));

        // Draw a border to see image bounds (for debugging)
        let border_color = Rgba([200, 200, 200, 255]);
        for x in 0..width {
            imgbuf.put_pixel(x, 0, border_color);
            imgbuf.put_pixel(x, height - 1, border_color);
        }
        for y in 0..height {
            imgbuf.put_pixel(0, y, border_color);
            imgbuf.put_pixel(width - 1, y, border_color);
        }

        // Pass scaling factor to the render function
        render_node(
            diagram_node,
            session,
            &mut imgbuf,
            (0.0, 0.0),
            scaling_factor,
        );

        // Write the PNG image to the stream
        let encoder = image::png::PngEncoder::new(stream);
        encoder
            .encode(
                imgbuf.as_raw(),
                imgbuf.width(),
                imgbuf.height(),
                image::ColorType::Rgba8,
            )
            .map_err(|e| RendererError::new(&e.to_string()))?;

        Ok(())
    }
}

// Render a node and its children
fn render_node(
    node: &DiagramTreeNode,
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    parent_offset: (Float, Float),
    scale: Float,
) {
    let entity_id = node.entity_id.clone();
    let pos = session.get_position(entity_id.clone());

    // Calculate absolute position by adding parent offset, then apply scaling
    let abs_pos = (
        (parent_offset.0 + pos.0) * scale,
        (parent_offset.1 + pos.1) * scale,
    );

    // Debug output to track node positioning
    let size = session.get_size(entity_id.clone());
    println!(
        "Rendering node type: {:?}, id: {}, pos: ({:.1}, {:.1}), size: ({:.1}, {:.1})",
        node.entity_type,
        entity_id.clone(),
        abs_pos.0,
        abs_pos.1,
        size.0,
        size.1
    );

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::BoxShape => {
            render_box(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::TextShape => {
            render_text(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::VerticalStackShape => {
            render_vertical_stack(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::HorizontalStackShape => {
            render_horizontal_stack(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::ImageShape => {
            render_image(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::TableShape => {
            // Get table properties
            let table_shape = session.get_table(entity_id.clone());
            let size = session.get_size(entity_id.clone());

            // Apply scaling factor to dimensions
            let width = (size.0 * scale).ceil() as u32;
            let height = (size.1 * scale).ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;

            if x >= 0
                && y >= 0
                && width > 0
                && height > 0
                && x + width as i32 <= imgbuf.width() as i32
                && y + height as i32 <= imgbuf.height() as i32
            {
                // Draw table outer border with specified color
                let border_color = parse_color(&table_shape.table_options.border_color);
                let border_width = (table_shape.table_options.border_width as f32 * scale) as u32;

                // Draw outer border (make it thicker for visibility)
                let rect = Rect::at(x, y).of_size(width, height);
                for i in 0..border_width {
                    if i < border_width {
                        let inner_rect = Rect::at(x + i as i32, y + i as i32)
                            .of_size(width - 2 * i, height - 2 * i);
                        draw_hollow_rect_mut(imgbuf, inner_rect, border_color);
                    }
                }

                // Draw header area
                let header_rect = session.get_size(table_shape.header_rect.clone());
                let header_height = (header_rect.1 * scale).ceil() as u32;
                if header_height > 0 {
                    // Get the exact header color from the options
                    let header_fill_color =
                        parse_color(&table_shape.table_options.header_fill_color);

                    // Debug print the header color
                    println!(
                        "Table header color: {}",
                        table_shape.table_options.header_fill_color
                    );

                    // Fill the header area
                    let header_rect = Rect::at(x, y).of_size(width, header_height);
                    draw_filled_rect_mut(imgbuf, header_rect, header_fill_color);
                    draw_hollow_rect_mut(imgbuf, header_rect, border_color);
                }

                // Use the predefined grid lines from the table
                // This uses the actual table_shape.col_lines and table_shape.row_lines
                // instead of trying to infer them from child positions

                // Draw column lines (vertical dividers)
                for col_line_id in &table_shape.col_lines {
                    // Get the position of this column line
                    let line_pos = session.get_position(col_line_id.clone());
                    let line_size = session.get_size(col_line_id.clone());

                    // Calculate the absolute x position with scaling
                    let line_x = (abs_pos.0 + line_pos.0 * scale as f32).round() as i32;

                    // Only draw if the line is within the image bounds
                    if line_x >= 0 && line_x < imgbuf.width() as i32 {
                        // Draw a vertical line from top to bottom of table
                        for i in 0..height {
                            let y_pos = y + i as i32;
                            if y_pos >= 0 && y_pos < imgbuf.height() as i32 {
                                imgbuf.put_pixel(line_x as u32, y_pos as u32, border_color);
                            }
                        }
                    }
                }

                // Draw row lines (horizontal dividers)
                for row_line_id in &table_shape.row_lines {
                    // Get the position of this row line
                    let line_pos = session.get_position(row_line_id.clone());
                    let line_size = session.get_size(row_line_id.clone());

                    // Calculate the absolute y position with scaling
                    let line_y = (abs_pos.1 + line_pos.1 * scale as f32).round() as i32;

                    // Only draw if the line is within the image bounds
                    if line_y >= 0 && line_y < imgbuf.height() as i32 {
                        // Draw a horizontal line from left to right of table
                        for i in 0..width {
                            let x_pos = x + i as i32;
                            if x_pos >= 0 && x_pos < imgbuf.width() as i32 {
                                imgbuf.put_pixel(x_pos as u32, line_y as u32, border_color);
                            }
                        }
                    }
                }

                // Render children (cells)
                for child in node.children.iter() {
                    render_node(child, session, imgbuf, abs_pos, scale);
                }
            }
        }
        EntityType::EllipseShape => {
            // Get ellipse properties
            let ellipse_shape = session.get_ellipse(entity_id.clone());
            let size = session.get_size(entity_id.clone());

            // Apply scaling factor to dimensions
            let width = (size.0 * scale).ceil() as u32;
            let height = (size.1 * scale).ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;

            if x >= 0
                && y >= 0
                && width > 0
                && height > 0
                && x + width as i32 <= imgbuf.width() as i32
                && y + height as i32 <= imgbuf.height() as i32
            {
                // Get colors from the ellipse properties
                let fill_color = parse_color(&ellipse_shape.ellipse_options.fill_color);
                let stroke_color = parse_color(&ellipse_shape.ellipse_options.stroke_color);

                // Calculate center coordinates
                let center_x = x + (width / 2) as i32;
                let center_y = y + (height / 2) as i32;
                let radius_x = (width / 2) as i32;
                let radius_y = (height / 2) as i32;

                // Draw filled ellipse first, using floating point for more accurate ellipse equation
                for py in y..y + height as i32 {
                    if py < 0 || py >= imgbuf.height() as i32 {
                        continue; // Skip if outside vertical bounds
                    }

                    for px in x..x + width as i32 {
                        if px < 0 || px >= imgbuf.width() as i32 {
                            continue; // Skip if outside horizontal bounds
                        }

                        // Calculate if this pixel is inside the ellipse using floating point
                        // for higher precision: (x/a)² + (y/b)² <= 1
                        let dx = (px - center_x) as f32;
                        let dy = (py - center_y) as f32;
                        let rx = radius_x as f32;
                        let ry = radius_y as f32;

                        let eq_value = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry);

                        if eq_value <= 1.0 {
                            imgbuf.put_pixel(px as u32, py as u32, fill_color);
                        }
                    }
                }

                // Draw the ellipse border using a modified Bresenham algorithm for smoother outlines
                // This implementation gives much higher quality anti-aliased edges
                draw_anti_aliased_ellipse(
                    imgbuf,
                    center_x,
                    center_y,
                    radius_x,
                    radius_y,
                    stroke_color,
                    (ellipse_shape.ellipse_options.stroke_width * scale as f32) as f32,
                );
            }
        }
        EntityType::PolyLine => {
            render_polyline(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }
        EntityType::FreeContainer => {
            render_free_container(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }

        EntityType::ArcShape => {
            render_arc(session, imgbuf, entity_id.clone(), node, abs_pos, scale);
        }

        // For this initial implementation, we'll skip other shapes
        _ => {}
    }
}

fn render_arc(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    use std::f32::consts::PI;
    
    let arc_shape = session.get_arc(entity_id.clone());
    let size = session.get_size(entity_id.clone());
    
    // Apply scaling factor to dimensions
    let scaled_radius = arc_shape.radius * scale;
    let center_x = (pos.0 + arc_shape.center.0 * scale) as i32;
    let center_y = (pos.1 + arc_shape.center.1 * scale) as i32;
    
    // Get colors
    let stroke_color = parse_color(&arc_shape.arc_options.stroke_color);
    let fill_color = parse_color(&arc_shape.arc_options.fill_color);
    let stroke_width = arc_shape.arc_options.stroke_width * scale;
    
    // Get normalized angles and convert to radians
    let (start_angle, end_angle) = arc_shape.normalize_angles();
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;
    
    // For filled arcs, we need to fill the sector
    if arc_shape.arc_options.filled {
        render_filled_arc_sector(
            imgbuf,
            center_x,
            center_y,
            scaled_radius,
            start_rad,
            end_rad,
            fill_color,
            stroke_color,
            stroke_width,
        );
    } else {
        // For unfilled arcs, just draw the arc curve
        render_arc_curve(
            imgbuf,
            center_x,
            center_y,
            scaled_radius,
            start_rad,
            end_rad,
            stroke_color,
            stroke_width,
        );
    }
}

// Helper function to render a filled arc sector (pie slice)
fn render_filled_arc_sector(
    imgbuf: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: Float,
    start_rad: Float,
    end_rad: Float,
    fill_color: Rgba<u8>,
    stroke_color: Rgba<u8>,
    stroke_width: Float,
) {
    let radius_i = radius as i32;
    
    // Fill the sector by checking each pixel in the bounding box
    for y in (center_y - radius_i)..=(center_y + radius_i) {
        for x in (center_x - radius_i)..=(center_x + radius_i) {
            if x >= 0 && x < imgbuf.width() as i32 && y >= 0 && y < imgbuf.height() as i32 {
                let dx = (x - center_x) as Float;
                let dy = (y - center_y) as Float;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    // Calculate angle of this pixel
                    let angle = dy.atan2(dx);
                    let angle_deg = angle * 180.0 / std::f32::consts::PI;
                    let normalized_angle = if angle_deg < 0.0 { angle_deg + 360.0 } else { angle_deg };
                    
                    // Check if this angle is within our arc
                    let start_deg = start_rad * 180.0 / std::f32::consts::PI;
                    let end_deg = end_rad * 180.0 / std::f32::consts::PI;
                    
                    let angle_in_arc = if end_deg > start_deg {
                        normalized_angle >= start_deg && normalized_angle <= end_deg
                    } else {
                        normalized_angle >= start_deg || normalized_angle <= end_deg
                    };
                    
                    if angle_in_arc {
                        imgbuf.put_pixel(x as u32, y as u32, fill_color);
                    }
                }
            }
        }
    }
    
    // Draw the arc outline
    render_arc_curve(imgbuf, center_x, center_y, radius, start_rad, end_rad, stroke_color, stroke_width);
    
    // Draw lines from center to arc endpoints for filled sectors
    let start_x = center_x + (radius * start_rad.cos()) as i32;
    let start_y = center_y + (radius * start_rad.sin()) as i32;
    let end_x = center_x + (radius * end_rad.cos()) as i32;
    let end_y = center_y + (radius * end_rad.sin()) as i32;
    
    draw_anti_aliased_line(imgbuf, center_x, center_y, start_x, start_y, stroke_color, stroke_width);
    draw_anti_aliased_line(imgbuf, center_x, center_y, end_x, end_y, stroke_color, stroke_width);
}

// Helper function to render just the arc curve
fn render_arc_curve(
    imgbuf: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: Float,
    start_rad: Float,
    end_rad: Float,
    stroke_color: Rgba<u8>,
    stroke_width: Float,
) {
    // Calculate the angle step based on radius for smooth curves
    let num_steps = (radius * 2.0).max(60.0) as i32; // More steps for larger arcs
    let angle_range = if end_rad > start_rad {
        end_rad - start_rad
    } else {
        (2.0 * std::f32::consts::PI) - start_rad + end_rad
    };
    
    let angle_step = angle_range / num_steps as Float;
    
    let mut prev_x = center_x + (radius * start_rad.cos()) as i32;
    let mut prev_y = center_y + (radius * start_rad.sin()) as i32;
    
    for i in 1..=num_steps {
        let current_angle = if end_rad > start_rad {
            start_rad + angle_step * i as Float
        } else {
            let angle = start_rad + angle_step * i as Float;
            if angle > 2.0 * std::f32::consts::PI {
                angle - 2.0 * std::f32::consts::PI
            } else {
                angle
            }
        };
        
        let current_x = center_x + (radius * current_angle.cos()) as i32;
        let current_y = center_y + (radius * current_angle.sin()) as i32;
        
        draw_anti_aliased_line(imgbuf, prev_x, prev_y, current_x, current_y, stroke_color, stroke_width);
        
        prev_x = current_x;
        prev_y = current_y;
    }
}

fn render_group(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    _entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_box(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    let size = session.get_size(entity_id.clone());
    let box_shape = session.get_box(node.entity_id.clone());

    // Convert to i32 for drawing functions with scaling
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    // Safety check to avoid drawing outside the image bounds
    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        // Skip this box if it's outside the bounds
        return;
    }

    let rect = Rect::at(x, y).of_size(width, height);

    // Handle fill color
    match &box_shape.box_options.fill_color {
        Fill::Color(color) => {
            let rgba = parse_color(color);
            draw_filled_rect_mut(imgbuf, rect, rgba);
        }
        // For now, we'll just use a default color for gradients
        _ => {
            draw_filled_rect_mut(imgbuf, rect, Rgba([255, 255, 255, 255]));
        }
    }

    // Draw border/stroke with scaled width
    let stroke_color = parse_color(&box_shape.box_options.stroke_color);
    let stroke_width = (box_shape.box_options.stroke_width * scale).ceil() as u32;

    // Draw border with proper thickness
    for i in 0..stroke_width {
        if i < stroke_width {
            let inner_rect =
                Rect::at(x + i as i32, y + i as i32).of_size(width - 2 * i, height - 2 * i);
            draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
        }
    }

    // Render children inside the box, accounting for padding
    for child in node.children.iter() {
        let child_id = child.entity_id.clone();
        let child_pos = session.get_position(child_id.clone());

        // Instead of trying to be clever about nested transforms, let's use the simplest
        // and most direct approach: manually handle rendering the text here

        // Log child details
        println!(
            "Rendering box child: id={}, type={:?}, box_pos=({:.1},{:.1}), child_pos=({:.1},{:.1})",
            child_id.clone(),
            child.entity_type,
            pos.0,
            pos.1,
            child_pos.0,
            child_pos.1
        );

        // Get absolute position for child relative to box
        let abs_x = (pos.0 + child_pos.0 * scale).round() as i32;
        let abs_y = (pos.1 + child_pos.1 * scale).round() as i32;

        // Calculate absolute position without any scaling
        let adjusted_parent_offset = (pos.0 / scale, pos.1 / scale);

        // Log the adjusted values
        println!(
            "  Using adjusted_parent_offset=({:.1},{:.1})",
            adjusted_parent_offset.0, adjusted_parent_offset.1
        );

        // Render the child with the adjusted parent offset
        // TODO: Centering logic for elems inside box
        render_node(child, session, imgbuf, adjusted_parent_offset, scale);
    }
}

fn render_text(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Get the text and render it directly
    let text_shape = session.get_text(entity_id.clone());
    //get size
    let size = session.get_size(entity_id.clone());
    //TODO: dynamically load the font data!
    //let font_data = include_bytes!("../../demo/assets/Roboto-Regular.ttf");
    let font_data = include_bytes!("../../demo/assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    // Convert text color string to RGBA
    let text_color = parse_color(&text_shape.text_options.text_color);
    println!(
        "  Text: '{}', color: {}, pos: ({}, {})",
        text_shape.text, text_shape.text_options.text_color, pos.0, pos.1
    );

    // Render text directly
    // let dpi = 120.0;
    // let dpi_scale_factor = dpi / 72.0;
    // let font_size = text_shape.text_options.font_size * dpi_scale_factor;

    let font_size = text_shape.text_options.font_size;
    let font_scale = Scale::uniform(font_size * scale as f32);

    // Render each line - use position data from layout engine
    // but adjust line spacing if needed for better aesthetics
    let line_count = text_shape.lines.len();
    let line_spacing_factor = if line_count > 1 { 0.6 } else { 1.0 }; // Further reduce spacing for multi-line text in boxes

    for (i, line_id) in text_shape.lines.iter().enumerate() {
        let line = session.get_text_line(line_id.clone());
        let lineSize = session.get_size(line_id.clone());
        let line_pos = session.get_position(line_id.clone());

        // Calculate base position without any margins yet
        let base_x = (pos.0 * scale) as i32 + (line_pos.0 * scale).round() as i32;

        // For multi-line text, calculate position with adjusted spacing
        let y_pos = if i == 0 {
            // First line uses original position
            line_pos.1
        } else {
            // Subsequent lines use compressed spacing
            let prev_line_pos = session.get_position(text_shape.lines[i - 1].clone());
            prev_line_pos.1 + (line_pos.1 - prev_line_pos.1) * line_spacing_factor
        };

        let line_y = pos.0 as i32 + (y_pos * scale).round() as i32;

        // Calculate the actual rendered text width using font metrics
        // for precise centering
        //let rendered_width = get_text_width(&line.text, &font, font_scale);
        let rendered_width = lineSize.0;
        // Calculate the left-side bearing (space before the first glyph)
        // This is needed because RustType positioning doesn't always start exactly at the x position we provide
        let first_char_glyph = font
            .glyph(line.text.chars().next().unwrap_or(' '))
            .scaled(font_scale)
            .positioned(rusttype::point(0.0, 0.0));

        let left_bearing = if let Some(bb) = first_char_glyph.pixel_bounding_box() {
            bb.min.x
        } else {
            0
        };

        // Center the text horizontally within the box
        // Adjust for the bounding box left side offset
        let centered_x =
            base_x + ((lineSize.0 as f32 - rendered_width as f32) / 2.0) as i32 - left_bearing;

        // Draw the text with centered position
        draw_high_quality_text(
            imgbuf,
            &line.text,
            centered_x,
            line_y,
            &font,
            font_scale,
            text_color,
            size.0 as i32, // Use box width as max width
        );
    }
}

// Draw text with higher quality anti-aliasing
fn draw_high_quality_text(
    imgbuf: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    font: &Font,
    scale: Scale,
    color: Rgba<u8>,
    _max_width: i32, // We keep this parameter for API compatibility but don't use it
) {
    // Calculate the vertical metrics once
    let v_metrics = font.v_metrics(scale);
    let offset_y = v_metrics.ascent;

    // Layout the glyphs in the text with proper positioning
    let mut caret = rusttype::point(0.0, offset_y);
    let mut last_glyph_id = None;
    let mut glyphs: Vec<rusttype::PositionedGlyph> = Vec::new();

    // Process each character for proper kerning and positioning
    for c in text.chars() {
        // Create the glyph
        let base_glyph = font.glyph(c);

        // Apply kerning if we have a previous glyph
        if let Some(previous) = last_glyph_id {
            caret.x += font.pair_kerning(scale, previous, base_glyph.id());
        }

        last_glyph_id = Some(base_glyph.id());

        // Get the advance width before we consume the glyph with scaled()
        let advance_width = base_glyph.scaled(scale).h_metrics().advance_width;

        // Position the glyph and add it to our collection
        // We need to create the glyph again since scaled() consumes it
        let positioned_glyph = font.glyph(c).scaled(scale).positioned(caret);
        glyphs.push(positioned_glyph);

        // Advance the caret using our saved advance_width
        caret.x += advance_width;
    }

    // Draw each glyph with anti-aliasing
    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image
            glyph.draw(|gx, gy, glyph_opacity| {
                // Map to actual screen position
                let px = x + bounding_box.min.x + gx as i32;
                let py = y + bounding_box.min.y + gy as i32;

                // Only draw if inside image bounds
                if px >= 0 && px < imgbuf.width() as i32 && py >= 0 && py < imgbuf.height() as i32 {
                    // Create a color with adjusted alpha for anti-aliasing
                    let alpha = (glyph_opacity * color[3] as f32) as u8;
                    let antialiased_color = Rgba([color[0], color[1], color[2], alpha]);

                    // Blend with existing pixels for smoother rendering
                    blend_pixel(imgbuf, px, py, antialiased_color, glyph_opacity);
                }
            });
        }
    }
}

fn render_vertical_stack(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(entity_id.clone());
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    if x >= 0
        && y >= 0
        && width > 0
        && height > 0
        && x + width as i32 <= imgbuf.width() as i32
        && y + height as i32 <= imgbuf.height() as i32
    {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 0, 255, 128]));
    }

    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_horizontal_stack(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(entity_id.clone());
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    if x >= 0
        && y >= 0
        && width > 0
        && height > 0
        && x + width as i32 <= imgbuf.width() as i32
        && y + height as i32 <= imgbuf.height() as i32
    {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 255, 0, 128]));
    }

    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_polyline(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    _node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Get polyline properties
    let polyline = session.get_polyline(entity_id.clone());
    let stroke_color = parse_color(&polyline.line_options.stroke_color);
    let stroke_width = (polyline.line_options.stroke_width * scale) as f32;

    // Need at least 2 points to draw a line
    if polyline.points.len() < 2 {
        return;
    }

    // Calculate absolute position with scaling
    let abs_x = pos.0;
    let abs_y = pos.1;

    // Draw line segments connecting all points
    for i in 0..polyline.points.len() - 1 {
        let (x1, y1) = polyline.points[i];
        let (x2, y2) = polyline.points[i + 1];

        // Apply scaling and offset
        let x1_scaled = (abs_x + x1 * scale) as i32;
        let y1_scaled = (abs_y + y1 * scale) as i32;
        let x2_scaled = (abs_x + x2 * scale) as i32;
        let y2_scaled = (abs_y + y2 * scale) as i32;

        // Draw an anti-aliased line with proper thickness
        draw_anti_aliased_line(
            imgbuf,
            x1_scaled,
            y1_scaled,
            x2_scaled,
            y2_scaled,
            stroke_color,
            stroke_width,
        );
    }

    // If it's a closed path (first point == last point), we're already done
    // Otherwise, check if the polyline should be closed by connecting last point to first
    if polyline.points.len() > 2 && polyline.points[0] != polyline.points[polyline.points.len() - 1]
    {
        // If user wants a closed shape (determined by checking if the first and last points are close enough)
        // This is just a heuristic - future implementations could add an explicit "closed" property
        let first = polyline.points[0];
        let last = polyline.points[polyline.points.len() - 1];
        let distance = ((first.0 - last.0).powi(2) + (first.1 - last.1).powi(2)).sqrt();

        // If points are very close, consider it a closed shape (like a polygon)
        if distance < 5.0 {
            // Apply scaling and offset
            let x1_scaled = (abs_x + last.0 * scale) as i32;
            let y1_scaled = (abs_y + last.1 * scale) as i32;
            let x2_scaled = (abs_x + first.0 * scale) as i32;
            let y2_scaled = (abs_y + first.1 * scale) as i32;

            // Draw the closing line
            draw_anti_aliased_line(
                imgbuf,
                x1_scaled,
                y1_scaled,
                x2_scaled,
                y2_scaled,
                stroke_color,
                stroke_width,
            );
        }
    }
}

fn render_free_container(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Log debug information
    println!("==================================================");
    println!(
        "Rendering FreeContainer: id={}, pos=({:.1}, {:.1})",
        entity_id.clone(),
        pos.0,
        pos.1
    );

    // Unscale the position (since render_node applies scaling for us)
    let container_pos = session.get_position(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    // Convert to i32 for drawing functions with scaling
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    println!("FreeContainer size: {}x{}", width, height);
    println!("FreeContainer absolute position: x={}, y={}", x, y);

    // Get the container object
    let container = session.get_free_container(entity_id.clone());

    // Draw the container background first if specified
    if let Some(bg_color) = &container.background_color {
        let fill_color = parse_color(bg_color);
        let rect = Rect::at(x, y).of_size(width, height);
        draw_filled_rect_mut(imgbuf, rect, fill_color);
        println!("Drew container background with color: {}", bg_color);
    }

    // Draw the container border if specified
    if let Some(border_color) = &container.border_color {
        if container.border_width > 0.0 {
            let stroke_color = parse_color(border_color);
            let stroke_width = (container.border_width * scale).ceil() as u32;

            // Draw border with proper thickness
            for i in 0..stroke_width {
                if i < stroke_width {
                    let inner_rect =
                        Rect::at(x + i as i32, y + i as i32).of_size(width - 2 * i, height - 2 * i);
                    draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
                }
            }
            println!("Drew container border with color: {}", border_color);
        }
    }

    // Log the children counts
    println!(
        "Container has {} stored positions and {} children in tree",
        container.children.len(),
        node.children.len()
    );

    // Create a mapping from child entity IDs to their positions
    let mut child_positions = std::collections::HashMap::new();
    for (child_id, position) in &container.children {
        child_positions.insert(child_id.clone(), *position);
    }

    // Debug output of all children
    for (i, child) in node.children.iter().enumerate() {
        let child_id = child.entity_id.clone();
        if let Some(rel_pos) = child_positions.get(&child_id) {
            println!(
                "Child[{}]: id={}, type={:?}, stored_pos=({:.1},{:.1})",
                i, child_id, child.entity_type, rel_pos.0, rel_pos.1
            );
        } else {
            println!(
                "Child[{}]: id={}, type={:?}, NO STORED POSITION",
                i, child_id, child.entity_type
            );
        }
    }

    // Render each child with its calculated position
    for (i, child_node) in node.children.iter().enumerate() {
        let child_id = child_node.entity_id.clone();

        // Get the child's position relative to the container from the stored mapping
        if let Some(rel_pos) = child_positions.get(&child_id) {
            // Child is in the FreeContainer's children map

            // For debugging, get the child's size
            let child_size = session.get_size(child_id.clone());
            println!(
                "Child[{}]: id={}, size=({:.1},{:.1})",
                i,
                child_id.clone(),
                child_size.0,
                child_size.1
            );

            // The key fix: since render_node applies pos and scaling again,
            // we need to provide a corrected parent_offset that when combined with
            // the child's position and scaled will result in the correct absolute position

            // Get the child's original position in the session
            let original_child_pos = session.get_position(child_id.clone());

            // Calculate the expected final position we want
            let desired_final_pos = (pos.0 + rel_pos.0 * scale, pos.1 + rel_pos.1 * scale);

            // Calculate the parent_offset that will give us this position after render_node applies
            // its own calculation: abs_pos = (parent_offset + pos) * scale
            // So we need: parent_offset = desired_final_pos / scale - pos
            let adjusted_parent_offset = (
                desired_final_pos.0 / scale - original_child_pos.0,
                desired_final_pos.1 / scale - original_child_pos.1,
            );

            println!("Rendering child[{}]: desired_pos=({:.1},{:.1}), original_pos=({:.1},{:.1}), rel_pos=({:.1},{:.1})", 
                i, desired_final_pos.0, desired_final_pos.1, original_child_pos.0, original_child_pos.1, rel_pos.0, rel_pos.1);
            println!(
                "  Using adjusted_parent_offset=({:.1},{:.1})",
                adjusted_parent_offset.0, adjusted_parent_offset.1
            );

            render_node(child_node, session, imgbuf, adjusted_parent_offset, scale);
        } else {
            // Child doesn't have a stored position
            println!(
                "WARNING: Child[{}] id={} has no stored position in FreeContainer!",
                i, child_id
            );

            // For children without explicit positions in the container, we'll use their
            // original positions from the session, which might be relative to the container
            let child_pos = session.get_position(child_id);

            // Calculate the desired final position
            let desired_final_pos = (pos.0 + child_pos.0 * scale, pos.1 + child_pos.1 * scale);

            // The same adjustment as above for children with stored positions
            let adjusted_parent_offset = (
                desired_final_pos.0 / scale - child_pos.0,
                desired_final_pos.1 / scale - child_pos.1,
            );

            // Log the calculated position
            println!("Child[{}] with no stored position: desired_pos=({:.1},{:.1}), original_pos=({:.1},{:.1})", 
                     i, desired_final_pos.0, desired_final_pos.1, child_pos.0, child_pos.1);
            println!(
                "  Using adjusted_parent_offset=({:.1},{:.1})",
                adjusted_parent_offset.0, adjusted_parent_offset.1
            );

            // Render the child with the adjusted parent offset
            render_node(child_node, session, imgbuf, adjusted_parent_offset, scale);
        }
    }

    println!("==================================================");
}

fn render_image(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    _node: &DiagramTreeNode,
    pos: (Float, Float),
    scale: Float,
) {
    // Get image properties
    let image_shape = session.get_image(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    // Apply scaling factor to dimensions
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;

    // Skip if outside bounds
    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        println!("Image outside bounds, skipping");
        return;
    }

    // Load the image either from file or base64 data
    let loaded_img = if let Some(file_path) = &image_shape.file_path {
        // Load image from file
        println!("Loading image from file: {}", file_path);
        match load_image_from_file(file_path) {
            Ok(img) => img,
            Err(e) => {
                println!("Error loading image from file: {}", e);
                // Return with a placeholder or error indicator
                draw_placeholder_image(imgbuf, x, y, width, height);
                return;
            }
        }
    } else if !image_shape.image.is_empty() {
        // Load image from base64 data
        println!("Loading image from base64 data");
        match load_image_from_base64(&image_shape.image) {
            Ok(img) => img,
            Err(e) => {
                println!("Error loading image from base64: {}", e);
                // Return with a placeholder or error indicator
                draw_placeholder_image(imgbuf, x, y, width, height);
                return;
            }
        }
    } else {
        println!("No image data or file path provided");
        // Draw an empty placeholder if no source is provided
        draw_placeholder_image(imgbuf, x, y, width, height);
        return;
    };

    // Resize the image to fit the allocated space while maintaining aspect ratio
    let resized_img = loaded_img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    // Convert the image to an RgbaImage
    let img_rgba = resized_img.to_rgba8();

    // Draw the image onto our output buffer at the specified position
    for (ix, iy, pixel) in img_rgba.enumerate_pixels() {
        let dest_x = x + ix as i32;
        let dest_y = y + iy as i32;

        // Only draw within bounds
        if dest_x >= 0
            && dest_x < imgbuf.width() as i32
            && dest_y >= 0
            && dest_y < imgbuf.height() as i32
        {
            imgbuf.put_pixel(dest_x as u32, dest_y as u32, *pixel);
        }
    }

    // Draw a thin border around the image for visual clarity
    let border_color = Rgba([80, 80, 80, 255]);
    let rect = Rect::at(x, y).of_size(width, height);
    draw_hollow_rect_mut(imgbuf, rect, border_color);
}

// Helper function to load an image from a file
fn load_image_from_file(file_path: &str) -> Result<DynamicImage, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    match image::open(path) {
        Ok(img) => Ok(img),
        Err(e) => Err(format!("Failed to load image: {}", e)),
    }
}

// Helper function to load an image from base64 data
fn load_image_from_base64(base64_str: &str) -> Result<DynamicImage, String> {
    // Decode base64 string to bytes
    let img_data = match BASE64.decode(base64_str) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to decode base64: {}", e)),
    };

    // Load image from memory
    match image::load_from_memory(&img_data) {
        Ok(img) => Ok(img),
        Err(e) => Err(format!("Failed to load image from memory: {}", e)),
    }
}

// Draw a placeholder for missing or error images
fn draw_placeholder_image(imgbuf: &mut RgbaImage, x: i32, y: i32, width: u32, height: u32) {
    // Fill with light gray
    let fill_color = Rgba([220, 220, 220, 255]);
    let rect = Rect::at(x, y).of_size(width, height);
    draw_filled_rect_mut(imgbuf, rect, fill_color);

    // Draw border
    let border_color = Rgba([150, 150, 150, 255]);
    draw_hollow_rect_mut(imgbuf, rect, border_color);

    // Draw an X from corner to corner
    if width > 10 && height > 10 {
        // Draw diagonal lines for the X
        for i in 0..width.min(height) {
            let ix = x + i as i32;
            let iy = y + i as i32;
            if ix < imgbuf.width() as i32 && iy < imgbuf.height() as i32 {
                imgbuf.put_pixel(ix as u32, iy as u32, Rgba([100, 100, 100, 255]));
            }

            let ix2 = x + i as i32;
            let iy2 = y + (height - i - 1) as i32;
            if ix2 < imgbuf.width() as i32 && iy2 >= 0 && iy2 < imgbuf.height() as i32 {
                imgbuf.put_pixel(ix2 as u32, iy2 as u32, Rgba([100, 100, 100, 255]));
            }
        }
    }
}

// Helper function to convert color string to Rgba
fn parse_color(color_str: &str) -> Rgba<u8> {
    match color_str.to_lowercase().as_str() {
        "black" => Rgba([0, 0, 0, 255]),
        "white" => Rgba([255, 255, 255, 255]),
        "red" => Rgba([255, 0, 0, 255]),
        "green" => Rgba([0, 255, 0, 255]),
        "blue" => Rgba([0, 0, 255, 255]),
        "yellow" => Rgba([255, 255, 0, 255]),
        "gray" | "grey" => Rgba([128, 128, 128, 255]),
        "lightgray" | "lightgrey" | "light gray" | "light grey" => Rgba([200, 200, 200, 255]),
        "darkgray" | "darkgrey" | "dark gray" | "dark grey" => Rgba([80, 80, 80, 255]),
        "orange" => Rgba([255, 165, 0, 255]),
        "purple" => Rgba([128, 0, 128, 255]),
        "brown" => Rgba([165, 42, 42, 255]),
        "cyan" => Rgba([0, 255, 255, 255]),
        "magenta" | "pink" => Rgba([255, 0, 255, 255]),
        _ => {
            println!("Parsing color: {}", color_str);
            // Handle hex color strings like "#RRGGBB" or "#RRGGBBAA"
            if color_str.starts_with('#') && (color_str.len() == 7 || color_str.len() == 9) {
                let r = u8::from_str_radix(&color_str[1..3], 16).unwrap_or(0);
                let g = u8::from_str_radix(&color_str[3..5], 16).unwrap_or(0);
                let b = u8::from_str_radix(&color_str[5..7], 16).unwrap_or(0);
                let a = if color_str.len() == 9 {
                    u8::from_str_radix(&color_str[7..9], 16).unwrap_or(255)
                } else {
                    255
                };
                println!("Parsed hex color to RGBA: [{}, {}, {}, {}]", r, g, b, a);
                Rgba([r, g, b, a])
            } else {
                // Return a visible color for unknown colors - use pink to make it obvious
                println!(
                    "WARNING: Unrecognized color '{}', defaulting to pink",
                    color_str
                );
                Rgba([255, 0, 255, 255])
            }
        }
    }
}

// Create an anti-aliased version of a color with adjustable alpha
fn blend_color(color: Rgba<u8>, intensity: f32) -> Rgba<u8> {
    // Calculate new alpha based on the original alpha and the intensity factor
    let alpha = (color[3] as f32 * intensity) as u8;
    Rgba([color[0], color[1], color[2], alpha])
}

// Blend a pixel with existing content for smooth anti-aliasing
fn blend_pixel(imgbuf: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>, alpha: f32) {
    if x < 0 || x >= imgbuf.width() as i32 || y < 0 || y >= imgbuf.height() as i32 {
        return;
    }

    // Get the existing pixel color
    let existing = imgbuf.get_pixel(x as u32, y as u32);

    // Alpha blending formula: new = alpha * src + (1 - alpha) * dst
    let blend_alpha = alpha.max(0.0).min(1.0);
    let inv_alpha = 1.0 - blend_alpha;

    let r = (color[0] as f32 * blend_alpha + existing[0] as f32 * inv_alpha) as u8;
    let g = (color[1] as f32 * blend_alpha + existing[1] as f32 * inv_alpha) as u8;
    let b = (color[2] as f32 * blend_alpha + existing[2] as f32 * inv_alpha) as u8;

    // Final alpha is combined alpha from both sources
    let a = (color[3] as f32 * blend_alpha + existing[3] as f32 * inv_alpha) as u8;

    imgbuf.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
}

// Draw a pixel with bounds checking
fn safe_put_pixel(imgbuf: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x >= 0 && x < imgbuf.width() as i32 && y >= 0 && y < imgbuf.height() as i32 {
        imgbuf.put_pixel(x as u32, y as u32, color);
    }
}

// Draw an anti-aliased line between two points with a given thickness
fn draw_anti_aliased_line(
    imgbuf: &mut RgbaImage,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
    thickness: f32,
) {
    // Use Bresenham's algorithm for the core line
    // Convert i32 to isize for the Bresenham algorithm
    let x0_isize = x0 as isize;
    let y0_isize = y0 as isize;
    let x1_isize = x1 as isize;
    let y1_isize = y1 as isize;

    for (x_isize, y_isize) in Bresenham::new((x0_isize, y0_isize), (x1_isize, y1_isize)) {
        // Convert back to i32 for our drawing functions
        let x = x_isize as i32;
        let y = y_isize as i32;

        // Draw a "thick" point at each position along the line
        let radius = (thickness / 2.0).ceil() as i32;
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                // Calculate alpha based on distance from the line center
                let alpha = if dist <= thickness / 2.0 {
                    // Full opacity for inner pixels
                    1.0
                } else if dist <= thickness / 2.0 + 1.0 {
                    // Fade out for anti-aliasing at the edge (smooth transition)
                    1.0 - (dist - thickness / 2.0)
                } else {
                    // Outside the line's radius
                    0.0
                };

                // Only draw if there's some opacity
                if alpha > 0.0 {
                    // Use blend_pixel for smoother edges
                    blend_pixel(imgbuf, x + dx, y + dy, color, alpha);
                }
            }
        }
    }
}

// Helper function to calculate the exact rendered width of a text string
// Esta funcionaba, es la misma que se usa en measure text?
fn get_text_width(text: &str, font: &Font, scale: Scale) -> f32 {
    // Calculate the width using font metrics with kerning
    let mut caret = 0.0f32;
    let mut prev_glyph_id = None;

    for c in text.chars() {
        // Get the glyph
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();

        // Add kerning if we have a previous glyph
        if let Some(prev_id) = prev_glyph_id {
            caret += font.pair_kerning(scale, prev_id, glyph_id);
        }

        // Get metrics for this glyph and add its advance width
        let advance_width = font.glyph(c).scaled(scale).h_metrics().advance_width;
        caret += advance_width;

        // Track previous glyph for kerning
        prev_glyph_id = Some(glyph_id);
    }

    // Return the final width
    caret
}

// Draw an anti-aliased ellipse with a given thickness
fn draw_anti_aliased_ellipse(
    imgbuf: &mut RgbaImage,
    cx: i32,
    cy: i32,
    a: i32,
    b: i32,
    color: Rgba<u8>,
    thickness: f32,
) {
    // For very small ellipses, use a simple algorithm
    if a <= 2 || b <= 2 {
        for angle_deg in 0..360 {
            let rad = angle_deg as Float * std::f32::consts::PI / 180.0;
            let x = cx + (a as f32 * rad.cos()).round() as i32;
            let y = cy + (b as f32 * rad.sin()).round() as i32;
            safe_put_pixel(imgbuf, x, y, color);
        }
        return;
    }

    // Improved ellipse drawing using line segments
    // Using more segments for smoother appearance - scale with radius for higher quality
    let num_segments = (a.max(b) * 8).max(120);

    // Calculate first point
    let first_angle: Float = 0.0;
    let first_x = cx + (a as f32 * first_angle.cos()).round() as i32;
    let first_y = cy + (b as f32 * first_angle.sin()).round() as i32;

    let mut prev_x = first_x;
    let mut prev_y = first_y;

    // Draw segments connecting points along the ellipse
    for i in 1..=num_segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32 / num_segments as f32);
        let x = cx + (a as f32 * angle.cos()).round() as i32;
        let y = cy + (b as f32 * angle.sin()).round() as i32;

        // Draw anti-aliased line segment between consecutive points
        draw_anti_aliased_line(imgbuf, prev_x, prev_y, x, y, color, thickness);

        prev_x = x;
        prev_y = y;
    }

    // Close the ellipse by connecting back to the first point
    draw_anti_aliased_line(imgbuf, prev_x, prev_y, first_x, first_y, color, thickness);
}
