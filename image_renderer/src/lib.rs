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
    diagram_builder::DiagramTreeNode, ConnectorType, DiagramBuilder, EntityID, EntityType, Fill,
    LinePointReference, OrthogonalRoutingStrategy, Point, Renderer, RendererError,
};

/**
 * PNG Renderer - uses absolute_positions cache from layout engine
 *
 * COORDINATE SYSTEM:
 * - Layout engine populates absolute_positions HashMap during layout
 * - Each entity has its absolute position cached
 * - PNG renderer reads from cache and applies scale factor
 * - No manual offset accumulation needed
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

        // Use a scaling factor for higher resolution output
        let scaling_factor = 1.5;

        // Calculate image dimensions with scaling
        let width = ((root_size.0 * scaling_factor).ceil() as u32).max(200);
        let height = ((root_size.1 * scaling_factor).ceil() as u32).max(200);

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

        // Render using absolute positions from cache
        render_node(diagram_node, session, &mut imgbuf, scaling_factor);

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

// Render a node using its absolute position from cache
fn render_node(
    node: &DiagramTreeNode,
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    scale: Float,
) {
    let entity_id = node.entity_id.clone();

    // Get absolute position from layout cache
    let (abs_x, abs_y) = session
        .absolute_positions
        .get(&entity_id)
        .copied()
        .unwrap_or_else(|| {
            println!(
                "⚠️ No absolute position cached for {}, using local",
                entity_id
            );
            session.get_local_position(entity_id.clone())
        });

    // Apply PNG scaling factor
    let screen_x = abs_x * scale;
    let screen_y = abs_y * scale;

    println!(
        "Rendering {:?} id={} at screen pos ({:.1}, {:.1})",
        node.entity_type, entity_id, screen_x, screen_y
    );

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session, imgbuf, node, scale);
        }
        EntityType::BoxShape => {
            render_box(
                session,
                imgbuf,
                entity_id.clone(),
                node,
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::RectShape => {
            render_rectangle(
                session,
                imgbuf,
                entity_id.clone(),
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::TextShape => {
            render_text(
                session,
                imgbuf,
                entity_id.clone(),
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::VerticalStackShape => {
            render_stack(session, imgbuf, node, scale);
        }
        EntityType::HorizontalStackShape => {
            render_stack(session, imgbuf, node, scale);
        }
        EntityType::ImageShape => {
            render_image(
                session,
                imgbuf,
                entity_id.clone(),
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::LineShape => {
            render_line(session, imgbuf, entity_id.clone(), scale);
        }
        EntityType::ArrowShape => {
            render_arrow(session, imgbuf, entity_id.clone(), scale);
        }
        EntityType::ConnectorShape => {
            render_connector(session, imgbuf, entity_id.clone(), scale);
        }
        EntityType::TableShape => {
            render_table(
                session,
                imgbuf,
                entity_id.clone(),
                node,
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::EllipseShape => {
            render_ellipse(
                session,
                imgbuf,
                entity_id.clone(),
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::PolyLine => {
            render_polyline(session, imgbuf, entity_id.clone(), scale);
        }
        EntityType::FreeContainer => {
            render_free_container(
                session,
                imgbuf,
                entity_id.clone(),
                node,
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::ArcShape => {
            render_arc(
                session,
                imgbuf,
                entity_id.clone(),
                screen_x,
                screen_y,
                scale,
            );
        }
        EntityType::ConstraintLayoutContainer => {
            render_constraint_layout(session, imgbuf, node, scale);
        }
        _ => {}
    }
}

fn render_group(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    node: &DiagramTreeNode,
    scale: Float,
) {
    // Groups just render their children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_box(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let size = session.get_size(entity_id.clone());
    let box_shape = session.get_box(entity_id.clone());

    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        return;
    }

    let rect = Rect::at(x, y).of_size(width, height);

    // Fill
    match &box_shape.box_options.fill_color {
        Fill::Color(color) => {
            let rgba = parse_color(color);
            draw_filled_rect_mut(imgbuf, rect, rgba);
        }
        _ => {
            draw_filled_rect_mut(imgbuf, rect, Rgba([255, 255, 255, 255]));
        }
    }

    // Stroke
    let stroke_color = parse_color(&box_shape.box_options.stroke_color);
    let stroke_width = (box_shape.box_options.stroke_width * scale).ceil() as u32;

    for i in 0..stroke_width {
        let inner_rect = Rect::at(x + i as i32, y + i as i32)
            .of_size(width.saturating_sub(2 * i), height.saturating_sub(2 * i));
        draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
    }

    // Render children (they have their own absolute positions)
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_rectangle(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let size = session.get_size(entity_id.clone());
    let rect_shape = session.get_rectangle(entity_id.clone());

    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        return;
    }

    let rect = Rect::at(x, y).of_size(width, height);

    match &rect_shape.rect_options.fill_color {
        Fill::Color(color) => {
            let rgba = parse_color(color);
            draw_filled_rect_mut(imgbuf, rect, rgba);
        }
        _ => {
            draw_filled_rect_mut(imgbuf, rect, Rgba([255, 255, 255, 255]));
        }
    }

    let stroke_color = parse_color(&rect_shape.rect_options.stroke_color);
    let stroke_width = (rect_shape.rect_options.stroke_width * scale).ceil() as u32;

    for i in 0..stroke_width {
        let inner_rect = Rect::at(x + i as i32, y + i as i32)
            .of_size(width.saturating_sub(2 * i), height.saturating_sub(2 * i));
        draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
    }
}
fn render_text(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    screen_x: Float, // ← Text entity's absolute position
    screen_y: Float,
    scale: Float,
) {
    let text_shape = session.get_text(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let font_data = include_bytes!("../../demo/assets/AnonymiceProNerdFont-Regular.ttf");
  
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let text_color = parse_color(&text_shape.text_options.text_color);
    let font_size = text_shape.text_options.font_size;
    let font_scale = Scale::uniform(font_size * scale);

    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(line_id.clone());

        // ✅ FIXED: Get line's LOCAL position
        let line_local_pos = session.get_local_position(line_id.clone());

        // ✅ FIXED: Add parent absolute + child local
        let line_screen_x = (screen_x + line_local_pos.0 * scale) as i32;
        let line_screen_y = (screen_y + line_local_pos.1 * scale) as i32;

        draw_high_quality_text(
            imgbuf,
            &line.text,
            line_screen_x,
            line_screen_y,
            &font,
            font_scale,
            text_color,
            (size.0 * scale) as i32,
        );
    }
}

fn draw_high_quality_text(
    imgbuf: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    font: &Font,
    scale: Scale,
    color: Rgba<u8>,
    _max_width: i32,
) {
    let v_metrics = font.v_metrics(scale);
    let offset_y = v_metrics.ascent;
    let mut caret = rusttype::point(0.0, offset_y);
    let mut last_glyph_id = None;
    let mut glyphs: Vec<rusttype::PositionedGlyph> = Vec::new();

    for c in text.chars() {
        let base_glyph = font.glyph(c);

        if let Some(previous) = last_glyph_id {
            caret.x += font.pair_kerning(scale, previous, base_glyph.id());
        }

        last_glyph_id = Some(base_glyph.id());
        let advance_width = base_glyph.scaled(scale).h_metrics().advance_width;
        let positioned_glyph = font.glyph(c).scaled(scale).positioned(caret);
        glyphs.push(positioned_glyph);
        caret.x += advance_width;
    }

    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, glyph_opacity| {
                let px = x + bounding_box.min.x + gx as i32;
                let py = y + bounding_box.min.y + gy as i32;

                if px >= 0 && px < imgbuf.width() as i32 && py >= 0 && py < imgbuf.height() as i32 {
                    let alpha = (glyph_opacity * color[3] as f32) as u8;
                    let antialiased_color = Rgba([color[0], color[1], color[2], alpha]);
                    blend_pixel(imgbuf, px, py, antialiased_color, glyph_opacity);
                }
            });
        }
    }
}

fn render_stack(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    node: &DiagramTreeNode,
    scale: Float,
) {
    // Stacks just render their children (which have their own absolute positions)
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_line(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    scale: Float,
) {
    let line_shape = session.get_line(entity_id.clone());

    let mut p_start = Point::new(0.0, 0.0);
    let mut p_end = Point::new(0.0, 0.0);

    match line_shape.start.clone() {
        LinePointReference::Value(x, y) => {
            p_start.x = x;
            p_start.y = y;
        }
        LinePointReference::PointID(id) => {
            let pos = session.get_local_position(id.clone());
            let line_pos = session.get_local_position(line_shape.entity.clone());
            p_start.x = pos.0 - line_pos.0;
            p_start.y = pos.1 - line_pos.1;
        }
    }

    match line_shape.end.clone() {
        LinePointReference::Value(x, y) => {
            p_end.x = x;
            p_end.y = y;
        }
        LinePointReference::PointID(id) => {
            let pos = session.get_local_position(id.clone());
            let line_pos = session.get_local_position(line_shape.entity.clone());
            p_end.x = pos.0 - line_pos.0;
            p_end.y = pos.1 - line_pos.1;
        }
    }

    // Get line's absolute position
    let (line_abs_x, line_abs_y) = session
        .absolute_positions
        .get(&entity_id)
        .copied()
        .unwrap_or_else(|| session.get_local_position(entity_id.clone()));

    let x1 = ((line_abs_x + p_start.x) * scale) as i32;
    let y1 = ((line_abs_y + p_start.y) * scale) as i32;
    let x2 = ((line_abs_x + p_end.x) * scale) as i32;
    let y2 = ((line_abs_y + p_end.y) * scale) as i32;

    let stroke_color = parse_color(&line_shape.line_options.stroke_color);
    let stroke_width = line_shape.line_options.stroke_width * scale;

    draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, stroke_color, stroke_width);
}

fn render_arrow(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    scale: Float,
) {
    let arrow_shape = session.get_arrow(entity_id.clone());

    // Get arrow's absolute position
    let (arrow_abs_x, arrow_abs_y) = session
        .absolute_positions
        .get(&entity_id)
        .copied()
        .unwrap_or_else(|| session.get_local_position(entity_id.clone()));

    let x1 = ((arrow_abs_x + arrow_shape.start.0) * scale) as i32;
    let y1 = ((arrow_abs_y + arrow_shape.start.1) * scale) as i32;
    let x2 = ((arrow_abs_x + arrow_shape.end.0) * scale) as i32;
    let y2 = ((arrow_abs_y + arrow_shape.end.1) * scale) as i32;

    let stroke_color = parse_color(&arrow_shape.arrow_options.stroke_color);
    let stroke_width = arrow_shape.arrow_options.stroke_width * scale;

    draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, stroke_color, stroke_width);

    // Draw arrowhead
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;
    let angle = dy.atan2(dx);
    let arrow_size = 10.0 * scale;

    let arrow_angle = 0.5;
    let left_x = x2 - (arrow_size * (angle - arrow_angle).cos()) as i32;
    let left_y = y2 - (arrow_size * (angle - arrow_angle).sin()) as i32;
    let right_x = x2 - (arrow_size * (angle + arrow_angle).cos()) as i32;
    let right_y = y2 - (arrow_size * (angle + arrow_angle).sin()) as i32;

    draw_anti_aliased_line(imgbuf, x2, y2, left_x, left_y, stroke_color, stroke_width);
    draw_anti_aliased_line(imgbuf, x2, y2, right_x, right_y, stroke_color, stroke_width);
}

fn render_connector(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    scale: Float,
) {
    let connector = session.get_connector(entity_id.clone());

    // Get absolute positions of start and end points
    let (start_x, start_y) = session
        .absolute_positions
        .get(&connector.start_point_id)
        .copied()
        .unwrap_or_else(|| session.get_local_position(connector.start_point_id.clone()));

    let (end_x, end_y) = session
        .absolute_positions
        .get(&connector.end_point_id)
        .copied()
        .unwrap_or_else(|| session.get_local_position(connector.end_point_id.clone()));

    // Apply PNG scale
    let x1 = (start_x * scale) as i32;
    let y1 = (start_y * scale) as i32;
    let x2 = (end_x * scale) as i32;
    let y2 = (end_y * scale) as i32;

    let stroke_color = parse_color(&connector.options.stroke_color);
    let stroke_width = connector.options.stroke_width * scale;

    match connector.options.connector_type {
        ConnectorType::Straight => {
            draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, stroke_color, stroke_width);
        }
        ConnectorType::Curved => {
            let curve_offset = connector.options.curve_offset.unwrap_or(50.0) * scale;
            let dx = x2 - x1;
            let dy = y2 - y1;
            let is_horizontal = dx.abs() > dy.abs();

            if is_horizontal {
                let cp1_x = x1 + dx / 2;
                let cp1_y = y1 + curve_offset as i32;
                let cp2_x = x2 - dx / 2;
                let cp2_y = y2 + curve_offset as i32;

                draw_bezier_curve(
                    imgbuf,
                    x1,
                    y1,
                    cp1_x,
                    cp1_y,
                    cp2_x,
                    cp2_y,
                    x2,
                    y2,
                    stroke_color,
                    stroke_width,
                );
            } else {
                let cp1_x = x1 + curve_offset as i32;
                let cp1_y = y1 + dy / 2;
                let cp2_x = x2 + curve_offset as i32;
                let cp2_y = y2 - dy / 2;

                draw_bezier_curve(
                    imgbuf,
                    x1,
                    y1,
                    cp1_x,
                    cp1_y,
                    cp2_x,
                    cp2_y,
                    x2,
                    y2,
                    stroke_color,
                    stroke_width,
                );
            }
        }
        ConnectorType::Orthogonal => {
            draw_orthogonal_path(
                imgbuf,
                x1,
                y1,
                x2,
                y2,
                connector.options.routing_strategy,
                stroke_color,
                stroke_width,
            );
        }
    }

    if connector.options.arrow_end {
        draw_arrow_marker(
            imgbuf,
            x1,
            y1,
            x2,
            y2,
            connector.options.arrow_size * scale,
            stroke_color,
            stroke_width,
        );
    }
    if connector.options.arrow_start {
        draw_arrow_marker(
            imgbuf,
            x2,
            y2,
            x1,
            y1,
            connector.options.arrow_size * scale,
            stroke_color,
            stroke_width,
        );
    }
}

fn draw_bezier_curve(
    imgbuf: &mut RgbaImage,
    x0: i32,
    y0: i32,
    cp1_x: i32,
    cp1_y: i32,
    cp2_x: i32,
    cp2_y: i32,
    x3: i32,
    y3: i32,
    color: Rgba<u8>,
    thickness: f32,
) {
    let num_segments = 50;
    let mut prev_x = x0;
    let mut prev_y = y0;

    for i in 1..=num_segments {
        let t = i as f32 / num_segments as f32;
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        let x = (mt3 * x0 as f32
            + 3.0 * mt2 * t * cp1_x as f32
            + 3.0 * mt * t2 * cp2_x as f32
            + t3 * x3 as f32) as i32;
        let y = (mt3 * y0 as f32
            + 3.0 * mt2 * t * cp1_y as f32
            + 3.0 * mt * t2 * cp2_y as f32
            + t3 * y3 as f32) as i32;

        draw_anti_aliased_line(imgbuf, prev_x, prev_y, x, y, color, thickness);
        prev_x = x;
        prev_y = y;
    }
}

fn draw_orthogonal_path(
    imgbuf: &mut RgbaImage,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    strategy: OrthogonalRoutingStrategy,
    color: Rgba<u8>,
    thickness: f32,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    match strategy {
        OrthogonalRoutingStrategy::HV => {
            draw_anti_aliased_line(imgbuf, x1, y1, x2, y1, color, thickness);
            draw_anti_aliased_line(imgbuf, x2, y1, x2, y2, color, thickness);
        }
        OrthogonalRoutingStrategy::VH => {
            draw_anti_aliased_line(imgbuf, x1, y1, x1, y2, color, thickness);
            draw_anti_aliased_line(imgbuf, x1, y2, x2, y2, color, thickness);
        }
        OrthogonalRoutingStrategy::HVH => {
            let mid_x = (x1 + x2) / 2;
            draw_anti_aliased_line(imgbuf, x1, y1, mid_x, y1, color, thickness);
            draw_anti_aliased_line(imgbuf, mid_x, y1, mid_x, y2, color, thickness);
            draw_anti_aliased_line(imgbuf, mid_x, y2, x2, y2, color, thickness);
        }
        OrthogonalRoutingStrategy::VHV => {
            if dx.abs() < 5 || dy.abs() < 5 {
                draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, color, thickness);
            } else {
                let mid_y = (y1 + y2) / 2;
                draw_anti_aliased_line(imgbuf, x1, y1, x1, mid_y, color, thickness);
                draw_anti_aliased_line(imgbuf, x1, mid_y, x2, mid_y, color, thickness);
                draw_anti_aliased_line(imgbuf, x2, mid_y, x2, y2, color, thickness);
            }
        }
        OrthogonalRoutingStrategy::Auto => {
            if dy.abs() > dx.abs() {
                if dx.abs() < 5 {
                    draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, color, thickness);
                } else {
                    let mid_y = (y1 + y2) / 2;
                    draw_anti_aliased_line(imgbuf, x1, y1, x1, mid_y, color, thickness);
                    draw_anti_aliased_line(imgbuf, x1, mid_y, x2, mid_y, color, thickness);
                    draw_anti_aliased_line(imgbuf, x2, mid_y, x2, y2, color, thickness);
                }
            } else {
                if dy.abs() < 5 {
                    draw_anti_aliased_line(imgbuf, x1, y1, x2, y2, color, thickness);
                } else {
                    let mid_x = (x1 + x2) / 2;
                    draw_anti_aliased_line(imgbuf, x1, y1, mid_x, y1, color, thickness);
                    draw_anti_aliased_line(imgbuf, mid_x, y1, mid_x, y2, color, thickness);
                    draw_anti_aliased_line(imgbuf, mid_x, y2, x2, y2, color, thickness);
                }
            }
        }
    }
}

fn draw_arrow_marker(
    imgbuf: &mut RgbaImage,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    size: Float,
    color: Rgba<u8>,
    thickness: f32,
) {
    let dx = (to_x - from_x) as f32;
    let dy = (to_y - from_y) as f32;
    let angle = dy.atan2(dx);

    let arrow_angle = 0.5;
    let left_x = to_x - (size * (angle - arrow_angle).cos()) as i32;
    let left_y = to_y - (size * (angle - arrow_angle).sin()) as i32;
    let right_x = to_x - (size * (angle + arrow_angle).cos()) as i32;
    let right_y = to_y - (size * (angle + arrow_angle).sin()) as i32;

    draw_anti_aliased_line(imgbuf, to_x, to_y, left_x, left_y, color, thickness);
    draw_anti_aliased_line(imgbuf, to_x, to_y, right_x, right_y, color, thickness);
    draw_anti_aliased_line(imgbuf, left_x, left_y, right_x, right_y, color, thickness);
}

fn render_table(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let table_shape = session.get_table(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        return;
    }

    let border_color = parse_color(&table_shape.table_options.border_color);
    let border_width = (table_shape.table_options.border_width as f32 * scale).ceil() as u32;

    // Outer border
    for i in 0..border_width {
        let inner_rect = Rect::at(x + i as i32, y + i as i32)
            .of_size(width.saturating_sub(2 * i), height.saturating_sub(2 * i));
        draw_hollow_rect_mut(imgbuf, inner_rect, border_color);
    }

    // Header
    if table_shape.header_rect.is_some() {
        let header_rect_size = session.get_size(table_shape.clone().header_rect.unwrap());
        let header_height = (header_rect_size.1 * scale).ceil() as u32;
        if header_height > 0 {
            let header_fill = parse_color(&table_shape.table_options.header_fill_color);
            let header_rect = Rect::at(x, y).of_size(width, header_height);
            draw_filled_rect_mut(imgbuf, header_rect, header_fill);
            draw_hollow_rect_mut(imgbuf, header_rect, border_color);
        }
    }

    // Column lines
    for col_line_id in &table_shape.col_lines {
        let (line_abs_x, _) = session
            .absolute_positions
            .get(col_line_id)
            .copied()
            .unwrap_or_else(|| session.get_local_position(col_line_id.clone()));

        let line_x = (line_abs_x * scale).round() as i32;

        if line_x >= 0 && line_x < imgbuf.width() as i32 {
            for i in 0..height {
                let y_pos = y + i as i32;
                if y_pos >= 0 && y_pos < imgbuf.height() as i32 {
                    imgbuf.put_pixel(line_x as u32, y_pos as u32, border_color);
                }
            }
        }
    }

    // Row lines
    for row_line_id in &table_shape.row_lines {
        let (_, line_abs_y) = session
            .absolute_positions
            .get(row_line_id)
            .copied()
            .unwrap_or_else(|| session.get_local_position(row_line_id.clone()));

        let line_y = (line_abs_y * scale).round() as i32;

        if line_y >= 0 && line_y < imgbuf.height() as i32 {
            for i in 0..width {
                let x_pos = x + i as i32;
                if x_pos >= 0 && x_pos < imgbuf.width() as i32 {
                    imgbuf.put_pixel(x_pos as u32, line_y as u32, border_color);
                }
            }
        }
    }

    // Render children
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_ellipse(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let ellipse_shape = session.get_ellipse(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;

    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        return;
    }

    let fill_color = parse_color(&ellipse_shape.ellipse_options.fill_color);
    let stroke_color = parse_color(&ellipse_shape.ellipse_options.stroke_color);

    let center_x = x + (width / 2) as i32;
    let center_y = y + (height / 2) as i32;
    let radius_x = (width / 2) as i32;
    let radius_y = (height / 2) as i32;

    // Fill
    for py in y..y + height as i32 {
        if py < 0 || py >= imgbuf.height() as i32 {
            continue;
        }

        for px in x..x + width as i32 {
            if px < 0 || px >= imgbuf.width() as i32 {
                continue;
            }

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

    // Stroke
    draw_anti_aliased_ellipse(
        imgbuf,
        center_x,
        center_y,
        radius_x,
        radius_y,
        stroke_color,
        ellipse_shape.ellipse_options.stroke_width * scale,
    );
}

fn render_arc(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    use std::f32::consts::PI;

    let arc_shape = session.get_arc(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let scaled_radius = arc_shape.radius * scale;

    // Center the arc within its bounding box
    let center_x = screen_x as i32 + (size.0 * scale / 2.0) as i32;
    let center_y = screen_y as i32 + (size.1 * scale / 2.0) as i32;

    let stroke_color = parse_color(&arc_shape.arc_options.stroke_color);
    let fill_color = parse_color(&arc_shape.arc_options.fill_color);
    let stroke_width = arc_shape.arc_options.stroke_width * scale;

    let (start_angle, end_angle) = arc_shape.normalize_angles();
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;

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

    for y in (center_y - radius_i)..=(center_y + radius_i) {
        for x in (center_x - radius_i)..=(center_x + radius_i) {
            if x >= 0 && x < imgbuf.width() as i32 && y >= 0 && y < imgbuf.height() as i32 {
                let dx = (x - center_x) as Float;
                let dy = (y - center_y) as Float;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    let angle = dy.atan2(dx);
                    let angle_deg = angle * 180.0 / std::f32::consts::PI;
                    let normalized_angle = if angle_deg < 0.0 {
                        angle_deg + 360.0
                    } else {
                        angle_deg
                    };

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

    render_arc_curve(
        imgbuf,
        center_x,
        center_y,
        radius,
        start_rad,
        end_rad,
        stroke_color,
        stroke_width,
    );

    let start_x = center_x + (radius * start_rad.cos()) as i32;
    let start_y = center_y + (radius * start_rad.sin()) as i32;
    let end_x = center_x + (radius * end_rad.cos()) as i32;
    let end_y = center_y + (radius * end_rad.sin()) as i32;

    draw_anti_aliased_line(
        imgbuf,
        center_x,
        center_y,
        start_x,
        start_y,
        stroke_color,
        stroke_width,
    );
    draw_anti_aliased_line(
        imgbuf,
        center_x,
        center_y,
        end_x,
        end_y,
        stroke_color,
        stroke_width,
    );
}

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
    let num_steps = (radius * 2.0).max(60.0) as i32;
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

        draw_anti_aliased_line(
            imgbuf,
            prev_x,
            prev_y,
            current_x,
            current_y,
            stroke_color,
            stroke_width,
        );

        prev_x = current_x;
        prev_y = current_y;
    }
}

fn render_polyline(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    scale: Float,
) {
    let polyline = session.get_polyline(entity_id.clone());

    let stroke_color = parse_color(&polyline.line_options.stroke_color);
    let stroke_width = polyline.line_options.stroke_width * scale;

    if polyline.points.len() < 2 {
        return;
    }

    // Get polyline's absolute position
    let (polyline_abs_x, polyline_abs_y) = session
        .absolute_positions
        .get(&entity_id)
        .copied()
        .unwrap_or_else(|| session.get_local_position(entity_id.clone()));

    for i in 0..polyline.points.len() - 1 {
        let (x1, y1) = polyline.points[i];
        let (x2, y2) = polyline.points[i + 1];

        let x1_scaled = ((polyline_abs_x + x1) * scale) as i32;
        let y1_scaled = ((polyline_abs_y + y1) * scale) as i32;
        let x2_scaled = ((polyline_abs_x + x2) * scale) as i32;
        let y2_scaled = ((polyline_abs_y + y2) * scale) as i32;

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

fn render_free_container(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    node: &DiagramTreeNode,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let size = session.get_size(entity_id.clone());
    let container = session.get_free_container(entity_id.clone());

    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;
    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;

    // Background
    if let Some(bg_color) = &container.background_color {
        let fill_color = parse_color(bg_color);
        let rect = Rect::at(x, y).of_size(width, height);
        draw_filled_rect_mut(imgbuf, rect, fill_color);
    }

    // Border
    if let Some(border_color) = &container.border_color {
        if container.border_width > 0.0 {
            let stroke_color = parse_color(border_color);
            let stroke_width = (container.border_width * scale).ceil() as u32;

            for i in 0..stroke_width {
                let inner_rect = Rect::at(x + i as i32, y + i as i32)
                    .of_size(width.saturating_sub(2 * i), height.saturating_sub(2 * i));
                draw_hollow_rect_mut(imgbuf, inner_rect, stroke_color);
            }
        }
    }

    // Render children (they have their own absolute positions)
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_constraint_layout(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    node: &DiagramTreeNode,
    scale: Float,
) {
    // Render all children (they have their own absolute positions)
    for child in node.children.iter() {
        render_node(child, session, imgbuf, scale);
    }
}

fn render_image(
    session: &DiagramBuilder,
    imgbuf: &mut RgbaImage,
    entity_id: EntityID,
    screen_x: Float,
    screen_y: Float,
    scale: Float,
) {
    let image_shape = session.get_image(entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let width = (size.0 * scale).ceil() as u32;
    let height = (size.1 * scale).ceil() as u32;
    let x = screen_x.round() as i32;
    let y = screen_y.round() as i32;

    if x < 0
        || y < 0
        || width == 0
        || height == 0
        || x + width as i32 > imgbuf.width() as i32
        || y + height as i32 > imgbuf.height() as i32
    {
        return;
    }

    let loaded_img = if let Some(file_path) = &image_shape.file_path {
        match load_image_from_file(file_path) {
            Ok(img) => img,
            Err(_) => {
                draw_placeholder_image(imgbuf, x, y, width, height);
                return;
            }
        }
    } else if !image_shape.image.is_empty() {
        match load_image_from_base64(&image_shape.image) {
            Ok(img) => img,
            Err(_) => {
                draw_placeholder_image(imgbuf, x, y, width, height);
                return;
            }
        }
    } else {
        draw_placeholder_image(imgbuf, x, y, width, height);
        return;
    };

    let resized_img = loaded_img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img_rgba = resized_img.to_rgba8();

    for (ix, iy, pixel) in img_rgba.enumerate_pixels() {
        let dest_x = x + ix as i32;
        let dest_y = y + iy as i32;

        if dest_x >= 0
            && dest_x < imgbuf.width() as i32
            && dest_y >= 0
            && dest_y < imgbuf.height() as i32
        {
            imgbuf.put_pixel(dest_x as u32, dest_y as u32, *pixel);
        }
    }

    let border_color = Rgba([80, 80, 80, 255]);
    let rect = Rect::at(x, y).of_size(width, height);
    draw_hollow_rect_mut(imgbuf, rect, border_color);
}

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

fn load_image_from_base64(base64_str: &str) -> Result<DynamicImage, String> {
    let img_data = match BASE64.decode(base64_str) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to decode base64: {}", e)),
    };

    match image::load_from_memory(&img_data) {
        Ok(img) => Ok(img),
        Err(e) => Err(format!("Failed to load image from memory: {}", e)),
    }
}

fn draw_placeholder_image(imgbuf: &mut RgbaImage, x: i32, y: i32, width: u32, height: u32) {
    let fill_color = Rgba([220, 220, 220, 255]);
    let rect = Rect::at(x, y).of_size(width, height);
    draw_filled_rect_mut(imgbuf, rect, fill_color);

    let border_color = Rgba([150, 150, 150, 255]);
    draw_hollow_rect_mut(imgbuf, rect, border_color);

    if width > 10 && height > 10 {
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
                Rgba([255, 0, 255, 255])
            }
        }
    }
}

fn blend_pixel(imgbuf: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>, alpha: f32) {
    if x < 0 || x >= imgbuf.width() as i32 || y < 0 || y >= imgbuf.height() as i32 {
        return;
    }

    let existing = imgbuf.get_pixel(x as u32, y as u32);
    let blend_alpha = alpha.max(0.0).min(1.0);
    let inv_alpha = 1.0 - blend_alpha;

    let r = (color[0] as f32 * blend_alpha + existing[0] as f32 * inv_alpha) as u8;
    let g = (color[1] as f32 * blend_alpha + existing[1] as f32 * inv_alpha) as u8;
    let b = (color[2] as f32 * blend_alpha + existing[2] as f32 * inv_alpha) as u8;
    let a = (color[3] as f32 * blend_alpha + existing[3] as f32 * inv_alpha) as u8;

    imgbuf.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
}

fn draw_anti_aliased_line(
    imgbuf: &mut RgbaImage,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
    thickness: f32,
) {
    let x0_isize = x0 as isize;
    let y0_isize = y0 as isize;
    let x1_isize = x1 as isize;
    let y1_isize = y1 as isize;

    for (x_isize, y_isize) in Bresenham::new((x0_isize, y0_isize), (x1_isize, y1_isize)) {
        let x = x_isize as i32;
        let y = y_isize as i32;

        let radius = (thickness / 2.0).ceil() as i32;
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                let alpha = if dist <= thickness / 2.0 {
                    1.0
                } else if dist <= thickness / 2.0 + 1.0 {
                    1.0 - (dist - thickness / 2.0)
                } else {
                    0.0
                };

                if alpha > 0.0 {
                    blend_pixel(imgbuf, x + dx, y + dy, color, alpha);
                }
            }
        }
    }
}

fn draw_anti_aliased_ellipse(
    imgbuf: &mut RgbaImage,
    cx: i32,
    cy: i32,
    a: i32,
    b: i32,
    color: Rgba<u8>,
    thickness: f32,
) {
    if a <= 2 || b <= 2 {
        for angle_deg in 0..360 {
            let rad = angle_deg as Float * std::f32::consts::PI / 180.0;
            let x = cx + (a as f32 * rad.cos()).round() as i32;
            let y = cy + (b as f32 * rad.sin()).round() as i32;
            if x >= 0 && x < imgbuf.width() as i32 && y >= 0 && y < imgbuf.height() as i32 {
                imgbuf.put_pixel(x as u32, y as u32, color);
            }
        }
        return;
    }

    let num_segments = (a.max(b) * 8).max(120);

    let first_angle: Float = 0.0;
    let first_x = cx + (a as f32 * first_angle.cos()).round() as i32;
    let first_y = cy + (b as f32 * first_angle.sin()).round() as i32;

    let mut prev_x = first_x;
    let mut prev_y = first_y;

    for i in 1..=num_segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32 / num_segments as f32);
        let x = cx + (a as f32 * angle.cos()).round() as i32;
        let y = cy + (b as f32 * angle.sin()).round() as i32;

        draw_anti_aliased_line(imgbuf, prev_x, prev_y, x, y, color, thickness);

        prev_x = x;
        prev_y = y;
    }

    draw_anti_aliased_line(imgbuf, prev_x, prev_y, first_x, first_y, color, thickness);
}
