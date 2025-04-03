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
            // Basic table support - just render it as a box for now
            let size = session.get_size(entity_id);
            let width = size.0.ceil() as u32;
            let height = size.1.ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;
            
            if x >= 0 && y >= 0 && width > 0 && height > 0 &&
               x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
                let rect = Rect::at(x, y).of_size(width, height);
                draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 0, 0, 255]));
                
                // Render children (cells)
                for child in node.children.iter() {
                    render_node(child, session, imgbuf, abs_pos);
                }
            }
        }
        EntityType::EllipseShape => {
            // Basic support - just render a box outline for ellipses
            let size = session.get_size(entity_id);
            let width = size.0.ceil() as u32;
            let height = size.1.ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;
            
            if x >= 0 && y >= 0 && width > 0 && height > 0 &&
               x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
                let rect = Rect::at(x, y).of_size(width, height);
                draw_hollow_rect_mut(imgbuf, rect, Rgba([255, 0, 0, 255]));
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
    match color_str {
        "black" => Rgba([0, 0, 0, 255]),
        "white" => Rgba([255, 255, 255, 255]),
        "red" => Rgba([255, 0, 0, 255]),
        "green" => Rgba([0, 255, 0, 255]),
        "blue" => Rgba([0, 0, 255, 255]),
        "yellow" => Rgba([255, 255, 0, 255]),
        _ => {
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
                Rgba([r, g, b, a])
            } else {
                // Default to black if unrecognized color
                Rgba([0, 0, 0, 255])
            }
        }
    }
}