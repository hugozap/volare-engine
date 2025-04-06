use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use image::{Rgba, RgbaImage, GenericImageView, DynamicImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use bresenham::Bresenham;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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
        
        // Use a scaling factor for higher resolution output but don't scale too much
        // 1.5 is a good balance between quality and maintaining layout proportions
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
        render_node(diagram_node, session, &mut imgbuf, (0.0, 0.0), scaling_factor);

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
fn render_node(node: &DiagramTreeNode, session: &DiagramBuilder, imgbuf: &mut RgbaImage, parent_offset: (f64, f64), scale: f64) {
    let entity_id = node.entity_id;
    let pos = session.get_position(entity_id);
    
    // Calculate absolute position by adding parent offset, then apply scaling
    let abs_pos = (
        (parent_offset.0 + pos.0) * scale, 
        (parent_offset.1 + pos.1) * scale
    );
    
    // Debug output to track node positioning
    let size = session.get_size(entity_id);
    println!("Rendering node type: {:?}, id: {}, pos: ({:.1}, {:.1}), size: ({:.1}, {:.1})",
             node.entity_type, entity_id, abs_pos.0, abs_pos.1, size.0, size.1);

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::BoxShape => {
            render_box(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::TextShape => {
            render_text(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::VerticalStackShape => {
            render_vertical_stack(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::HorizontalStackShape => {
            render_horizontal_stack(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::ImageShape => {
            render_image(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        EntityType::TableShape => {
            // Get table properties
            let table_shape = session.get_table(entity_id);
            let size = session.get_size(entity_id);
            
            // Apply scaling factor to dimensions
            let width = (size.0 * scale).ceil() as u32;
            let height = (size.1 * scale).ceil() as u32;
            let x = abs_pos.0.round() as i32;
            let y = abs_pos.1.round() as i32;
            
            if x >= 0 && y >= 0 && width > 0 && height > 0 &&
               x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
                // Draw table outer border with specified color
                let border_color = parse_color(&table_shape.table_options.border_color);
                let border_width = (table_shape.table_options.border_width as f64 * scale) as u32;
                
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
                let header_height = (header_rect.1 * scale).ceil() as u32;
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
                    
                    // Calculate the absolute x position with scaling
                    let line_x = (abs_pos.0 + line_pos.0 * scale as f64).round() as i32;
                    
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
                    
                    // Calculate the absolute y position with scaling
                    let line_y = (abs_pos.1 + line_pos.1 * scale as f64).round() as i32;
                    
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
            let ellipse_shape = session.get_ellipse(entity_id);
            let size = session.get_size(entity_id);
            
            // Apply scaling factor to dimensions
            let width = (size.0 * scale).ceil() as u32;
            let height = (size.1 * scale).ceil() as u32;
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
                
                // Draw the ellipse border using a modified Bresenham algorithm for smoother outlines
                // This implementation gives much higher quality anti-aliased edges
                draw_anti_aliased_ellipse(
                    imgbuf,
                    center_x,
                    center_y,
                    radius_x,
                    radius_y,
                    stroke_color,
                    (ellipse_shape.ellipse_options.stroke_width * scale as f64) as f32
                );
            }
        }
        EntityType::PolyLine => {
            render_polyline(session, imgbuf, entity_id, node, abs_pos, scale);
        }
        // For this initial implementation, we'll skip other shapes
        _ => {}
    }
}

fn render_group(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_box(session: &DiagramBuilder, imgbuf: &mut RgbaImage, entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    let size = session.get_size(entity_id);
    let box_shape = session.get_box(node.entity_id);
    
    // Convert to i32 for drawing functions with scaling
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    
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
    
    // Draw border/stroke with scaled width
    let stroke_color = parse_color(&box_shape.box_options.stroke_color);
    let stroke_width = (box_shape.box_options.stroke_width * scale).ceil() as u32;
    
    // Draw border with proper thickness
    for i in 0..stroke_width {
        if i < stroke_width {
            let inner_rect = Rect::at(x + i as i32, y + i as i32)
                .of_size(width - 2 * i, height - 2 * i);
            draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
        }
    }
    
    // Render children inside the box
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_text(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64), scaling_factor: f64) {
    let text_shape = session.get_text(node.entity_id);
    let font_data = include_bytes!("../../demo/assets/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    
    // Convert text color string to Rgba
    let text_color = parse_color(&text_shape.text_options.text_color);
    
    // Calculate font scaling - be careful not to over-scale to maintain proper text layout
    // We'll use a modest DPI increase for quality, but scale the font more conservatively
    let dpi = 120.0; // Better quality without excessive scaling
    let dpi_scale_factor = dpi / 72.0;
    
    // Ensure we use a scaling factor that matches the layout measurements
    let text_scaling = (scaling_factor * 0.95) as f32; // Increased to 0.95 to match measurement DPI better
    let font_size = text_shape.text_options.font_size * dpi_scale_factor * text_scaling;
    let font_scale = Scale::uniform(font_size);
    
    // Render each text line with custom anti-aliasing
    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(*line_id);
        let line_pos = session.get_position(line.entity);
        
        // Convert to i32 for our draw functions with scaling
        // Add a small margin for visual spacing
        let text_margin_horizontal = 4; // Small margin for visual spacing
        let text_margin_vertical = 2;   // Small vertical margin for better spacing
        let line_x = (pos.0 + line_pos.0 * scaling_factor).round() as i32 + text_margin_horizontal;
        let line_y = (pos.1 + line_pos.1 * scaling_factor).round() as i32 + text_margin_vertical;
        
        // Skip text if it would be drawn outside the image bounds
        if line_x < 0 || line_x >= imgbuf.width() as i32 || 
           line_y < 0 || line_y >= imgbuf.height() as i32 {
            continue;
        }
        
        // Get the entity size that contains this text to use as the max width
        // The table layout module should have already determined the optimal cell size
        let containing_entity_size = session.get_size(node.entity_id);
        
        // Calculate the effective width available for text
        // We don't need a large safety margin since we're not truncating
        let max_text_width = (containing_entity_size.0 * scaling_factor).round() as i32 - 
                             (text_margin_horizontal * 2);
        
        // Draw high quality anti-aliased text without truncation
        draw_high_quality_text(
            imgbuf,
            &line.text,
            line_x,
            line_y,
            &font,
            font_scale,
            text_color,
            max_text_width
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
    _max_width: i32  // We keep this parameter for API compatibility but don't use it
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

fn render_vertical_stack(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(_entity_id);
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    
    if x >= 0 && y >= 0 && width > 0 && height > 0 &&
       x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 0, 255, 128]));
    }
    
    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_horizontal_stack(session: &DiagramBuilder, imgbuf: &mut RgbaImage, _entity_id: EntityID, node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    // Draw a debug rectangle to show the stack bounds
    let size = session.get_size(_entity_id);
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    
    if x >= 0 && y >= 0 && width > 0 && height > 0 &&
       x + width as i32 <= imgbuf.width() as i32 && y + height as i32 <= imgbuf.height() as i32 {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_hollow_rect_mut(imgbuf, rect, Rgba([0, 255, 0, 128]));
    }
    
    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, pos, scale);
    }
}

fn render_polyline(session: &DiagramBuilder, imgbuf: &mut RgbaImage, entity_id: EntityID, _node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    // Get polyline properties
    let polyline = session.get_polyline(entity_id);
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
            stroke_width
        );
    }
    
    // If it's a closed path (first point == last point), we're already done
    // Otherwise, check if the polyline should be closed by connecting last point to first
    if polyline.points.len() > 2 && polyline.points[0] != polyline.points[polyline.points.len() - 1] {
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
                stroke_width
            );
        }
    }
}

fn render_image(session: &DiagramBuilder, imgbuf: &mut RgbaImage, entity_id: EntityID, _node: &DiagramTreeNode, pos: (f64, f64), scale: f64) {
    // Get image properties
    let image_shape = session.get_image(entity_id);
    let size = session.get_size(entity_id);
    
    // Apply scaling factor to dimensions
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;
    
    // Skip if outside bounds
    if x < 0 || y < 0 || width == 0 || height == 0 || 
       x + width as i32 > imgbuf.width() as i32 || 
       y + height as i32 > imgbuf.height() as i32 {
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
        if dest_x >= 0 && dest_x < imgbuf.width() as i32 && 
           dest_y >= 0 && dest_y < imgbuf.height() as i32 {
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
                println!("WARNING: Unrecognized color '{}', defaulting to pink", color_str);
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
    thickness: f32
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

// Draw an anti-aliased ellipse with a given thickness
fn draw_anti_aliased_ellipse(
    imgbuf: &mut RgbaImage,
    cx: i32,
    cy: i32,
    a: i32,
    b: i32,
    color: Rgba<u8>,
    thickness: f32
) {
    // For very small ellipses, use a simple algorithm
    if a <= 2 || b <= 2 {
        for angle_deg in 0..360 {
            let rad = angle_deg as f64 * std::f64::consts::PI / 180.0;
            let x = cx + (a as f64 * rad.cos()).round() as i32;
            let y = cy + (b as f64 * rad.sin()).round() as i32;
            safe_put_pixel(imgbuf, x, y, color);
        }
        return;
    }

    // Improved ellipse drawing using line segments
    // Using more segments for smoother appearance - scale with radius for higher quality
    let num_segments = (a.max(b) * 8).max(120);
    
    // Calculate first point
    let first_angle: f64 = 0.0;
    let first_x = cx + (a as f64 * first_angle.cos()).round() as i32;
    let first_y = cy + (b as f64 * first_angle.sin()).round() as i32;
    
    let mut prev_x = first_x;
    let mut prev_y = first_y;
    
    // Draw segments connecting points along the ellipse
    for i in 1..=num_segments {
        let angle = 2.0 * std::f64::consts::PI * (i as f64 / num_segments as f64);
        let x = cx + (a as f64 * angle.cos()).round() as i32;
        let y = cy + (b as f64 * angle.sin()).round() as i32;
        
        // Draw anti-aliased line segment between consecutive points
        draw_anti_aliased_line(imgbuf, prev_x, prev_y, x, y, color, thickness);
        
        prev_x = x;
        prev_y = y;
    }
    
    // Close the ellipse by connecting back to the first point
    draw_anti_aliased_line(imgbuf, prev_x, prev_y, first_x, first_y, color, thickness);
}