use std::io::Write;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

use volare_engine_layout::{
    diagram_builder::DiagramTreeNode, 
    Renderer, 
    DiagramBuilder, 
    RendererError,
    EntityType,
    EntityID,
    Fill
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
        let root_size = session.get_size(diagram_node.entity_id);
        
        // Ensure reasonable image dimensions
        let width = (root_size.0.ceil() as u32).max(100);
        let height = (root_size.1.ceil() as u32).max(100);
        
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

        // Render the diagram to the image buffer
        render_node(diagram_node, session, &mut imgbuf, (0.0, 0.0));

        // Write the PNG image to the stream
        let encoder = image::png::PngEncoder::new(stream);
        encoder.encode(
            imgbuf.as_raw(),
            imgbuf.width(),
            imgbuf.height(),
            image::ColorType::Rgba8,
        ).map_err(|e| RendererError::new(&e.to_string()))?;

        Ok(())
    }
}

// Render a node and its children
fn render_node(node: &DiagramTreeNode, session: &DiagramBuilder, imgbuf: &mut RgbaImage, parent_offset: (f64, f64)) {
    let entity_id = node.entity_id;
    let pos = session.get_position(entity_id);
    
    // Calculate absolute position by adding parent offset
    let abs_pos = (parent_offset.0 + pos.0, parent_offset.1 + pos.1);
    
    // Debug output to track node positioning
    let size = session.get_size(entity_id);
    println!("Rendering node type: {:?}, id: {}, pos: ({:.1}, {:.1}), size: ({:.1}, {:.1})",
             node.entity_type, entity_id, abs_pos.0, abs_pos.1, size.0, size.1);

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session, imgbuf, entity_id, node, abs_pos);
        }
        EntityType::BoxShape => {
            render_box(session, imgbuf, entity_id, node, abs_pos);
        }
        EntityType::TextShape => {
            render_text(session, imgbuf, entity_id, node, abs_pos);
        }
        EntityType::VerticalStackShape => {
            render_vertical_stack(session, imgbuf, entity_id, node, abs_pos);
        }
        EntityType::HorizontalStackShape => {
            render_horizontal_stack(session, imgbuf, entity_id, node, abs_pos);
        }
        EntityType::TableShape => {
            // Get table properties
            let table_shape = session.get_table(entity_id);
            let size = session.get_size(entity_id);
            let width = size.0.ceil() as u32;
            let height = size.1.ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;
            
            if x >= 0 && y >= 0 && width > 0 && height > 0 &&
               x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
                // Draw table outer border with specified color
                let border_color = parse_color(&table_shape.table_options.border_color);
                let border_width = table_shape.table_options.border_width as u32;
                
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
                let header_rect = session.get_size(table_shape.header_rect);
                let header_height = header_rect.1.ceil() as u32;
                if header_height > 0 {
                    // Get the exact header color from the options
                    let header_fill_color = parse_color(&table_shape.table_options.header_fill_color);
                    
                    // Debug print the header color
                    println!("Table header color: {}", table_shape.table_options.header_fill_color);
                    
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
                    let line_pos = session.get_position(*col_line_id);
                    let line_size = session.get_size(*col_line_id);
                    
                    // Calculate the absolute x position
                    let line_x = (abs_pos.0 + line_pos.0).round() as i32;
                    
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
                    let line_pos = session.get_position(*row_line_id);
                    let line_size = session.get_size(*row_line_id);
                    
                    // Calculate the absolute y position
                    let line_y = (abs_pos.1 + line_pos.1).round() as i32;
                    
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
                    render_node(child, session, imgbuf, abs_pos);
                }
            }
        }
        EntityType::EllipseShape => {
            // Get ellipse properties
            let ellipse_shape = session.get_ellipse(entity_id);
            let size = session.get_size(entity_id);
            let width = size.0.ceil() as u32;
            let height = size.1.ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;
            
            if x >= 0 && y >= 0 && width > 0 && height > 0 &&
               x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
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
                        continue;  // Skip if outside vertical bounds
                    }
                    
                    for px in x..x + width as i32 {
                        if px < 0 || px >= imgbuf.width() as i32 {
                            continue;  // Skip if outside horizontal bounds
                        }
                        
                        // Calculate if this pixel is inside the ellipse using floating point
                        // for higher precision: (x/a)² + (y/b)² <= 1
                        let dx = (px - center_x) as f64;
                        let dy = (py - center_y) as f64;
                        let rx = radius_x as f64;
                        let ry = radius_y as f64;
                        
                        let eq_value = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry);
                        
                        if eq_value <= 1.0 {
                            imgbuf.put_pixel(px as u32, py as u32, fill_color);
                        }
                    }
                }
                
                // Draw the ellipse border with higher angle resolution for smoother appearance
                for angle_deg in 0..720 {  // Use twice as many points for smoother outline
                    let angle = angle_deg as f64 / 2.0;  // Convert to 0-360 range in half-degree steps
                    let rad = angle * std::f64::consts::PI / 180.0;
                    
                    // Calculate border point position
                    let border_x = center_x + (radius_x as f64 * rad.cos()) as i32;
                    let border_y = center_y + (radius_y as f64 * rad.sin()) as i32;
                    
                    // Only draw if point is within image bounds
                    if border_x >= 0 && border_x < imgbuf.width() as i32 && 
                       border_y >= 0 && border_y < imgbuf.height() as i32 {
                        imgbuf.put_pixel(border_x as u32, border_y as u32, stroke_color);
                    }
                }
            }
        }
        // For this initial implementation, we'll skip other shapes
        _ => {}
    }
}

fn render_group(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64)) {
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos);
    }
}

fn render_box(session: &DiagramBuilder, imgbuf: &mut RgbaImage, entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64)) {
    let size = session.get_size(entity_id);
    let box_shape = session.get_box(node.entity_id);
    
    // Convert to i32 for drawing functions
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = size.0.ceil() as u32;
    let height = size.1.ceil() as u32;
    
    // Safety check to avoid drawing outside the image bounds
    if x < 0 || y < 0 || width == 0 || height == 0 || 
       x + width as i32 > imgbuf.width() as i32 || 
       y + height as i32 > imgbuf.height() as i32 {
        // Skip this box if it's outside the bounds
        return;
    }
    
    let rect = Rect::at(x, y).of_size(width, height);

    // Handle fill color
    match &box_shape.box_options.fill_color {
        Fill::Color(color) => {
            let rgba = parse_color(color);
            draw_filled_rect_mut(imgbuf, rect, rgba);
        },
        // For now, we'll just use a default color for gradients
        _ => {
            draw_filled_rect_mut(imgbuf, rect, Rgba([255, 255, 255, 255]));
        }
    }
    
    // Draw border/stroke
    let stroke_color = parse_color(&box_shape.box_options.stroke_color);
    draw_hollow_rect_mut(imgbuf, rect, stroke_color);
    
    // Render children inside the box
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos);
    }
}

fn render_text(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64)) {
    let text_shape = session.get_text(node.entity_id);
    let font_data = include_bytes!("../../demo/assets/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    
    // Convert text color string to Rgba
    let text_color = parse_color(&text_shape.text_options.text_color);
    
    // Web browsers use 96 dpi by default, but rusttype uses 72 dpi
    let dpi = 96.0;
    let scale_factor = dpi / 72.0;
    let font_size = text_shape.text_options.font_size * scale_factor;
    let scale = Scale::uniform(font_size);
    
    // Render each text line
    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(*line_id);
        let line_pos = session.get_position(line.entity);
        
        // Convert to u32 for draw_text_mut
        let line_x = (pos.0 + line_pos.0).round() as u32;
        let line_y = (pos.1 + line_pos.1).round() as u32;
        
        // Skip text if it would be drawn outside the image bounds
        if line_x >= imgbuf.width() || line_y >= imgbuf.height() {
            continue;
        }
        
        draw_text_mut(
            imgbuf,
            text_color,
            line_x,
            line_y,
            scale,
            &font,
            &line.text
        );
    }
}

fn render_vertical_stack(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64)) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(_entity_id);
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = size.0.ceil() as u32;
    let height = size.1.ceil() as u32;
    
    if x >= 0 && y >= 0 && width > 0 && height > 0 &&
       x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 0, 255, 128]));
    }
    
    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos);
    }
}

fn render_horizontal_stack(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64)) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(_entity_id);
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = size.0.ceil() as u32;
    let height = size.1.ceil() as u32;
    
    if x >= 0 && y >= 0 && width > 0 && height > 0 &&
       x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 255, 0, 128]));
    }
    
    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos);
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
                println!("WARNING: Unrecognized color '{}', defaulting to pink", color_str);
                Rgba([255, 0, 255, 255])
            }
        }
    }
}