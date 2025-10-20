use std::io::Write;
use volare_engine_layout::diagram_builder::DiagramTreeNode;
use volare_engine_layout::FreeContainer;
use volare_engine_layout::*;
//use error
use crate::transform::Transform;
use std::io::Error;

pub struct SVGRenderer;

impl<W: Write> Renderer<W> for SVGRenderer {
    fn render(
        &self,
        session: &DiagramBuilder,
        diagram_node: &DiagramTreeNode,
        stream: &mut W,
    ) -> Result<(), RendererError> {
        let mut svg = String::new();
        let root_bounds = session.get_effective_bounds(diagram_node.entity_id.clone());
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
            root_bounds.width, root_bounds.height,
            root_bounds.x, root_bounds.y, root_bounds.width, root_bounds.height
        ));

         svg.push_str(r#"<defs>"#);
        add_arrow_markers(&mut svg, session, diagram_node);
        svg.push_str(r#"</defs>"#);

        svg.push_str(render_node(diagram_node, session).as_str());

        // Render all connectors in a separate group at the end (on top)
        svg.push_str(r#"<g id="connectors-layer">"#);
        render_all_connectors(diagram_node, session, &mut svg);
        svg.push_str("</g>");

        //close svg tag
        svg.push_str("</svg>");
        svg.push_str("\n");

        // Properly handle the result from writing to the stream
        stream
            .write_all(svg.as_bytes())
            .map_err(|e| RendererError::new(&e.to_string()))?;

        Ok(())
    }
}

/// Renders a node and its children, but skips connectors
fn render_node(node: &DiagramTreeNode, session: &DiagramBuilder) -> String {
    // Skip connectors - they'll be rendered in a separate pass
    if node.entity_type == EntityType::ConnectorShape {
        return String::new();
    }
    
    // Render this node normally
    let entity_id = node.entity_id.clone();
    let mut result = String::new();

    match node.entity_type {
        EntityType::TextShape => render_text(session, &mut result, entity_id.clone(), node),
        EntityType::BoxShape => render_box(session, &mut result, entity_id.clone(), node),
        EntityType::RectShape => render_rectangle(session, &mut result, entity_id.clone(), node),
        EntityType::LineShape => render_line(session, &mut result, entity_id.clone(), node),
        EntityType::ArrowShape => render_arrow(session, &mut result, entity_id.clone(), node),
        EntityType::EllipseShape => render_ellipse(session, &mut result, entity_id.clone(), node),
        EntityType::GroupShape => render_group(session, &mut result, entity_id.clone(), node),
        EntityType::TableShape => render_table(session, &mut result, entity_id.clone(), node),
        EntityType::ImageShape => render_image(session, &mut result, entity_id.clone(), node),
        EntityType::PolyLine => render_polyline(session, &mut result, entity_id.clone(), node),
        EntityType::FreeContainer => render_free_container(session, &mut result, entity_id.clone(), node),
        EntityType::ArcShape => render_arc(session, &mut result, entity_id.clone(), node),
        EntityType::VerticalStackShape => render_vertical_stack(session, &mut result, entity_id.clone(), node),
        EntityType::HorizontalStackShape => render_horizontal_stack(session, &mut result, entity_id.clone(), node),
        EntityType::ConstraintLayoutContainer => render_constraint_layout_container(session, &mut result, entity_id.clone(), node),
        EntityType::ConnectorShape => {}, // Skip - handled separately
        _ => {}
    }

    result
}

/// Collects all unique arrow configurations and creates marker definitions
fn add_arrow_markers(svg: &mut String, session: &DiagramBuilder, node: &DiagramTreeNode) {
    let mut markers = std::collections::HashSet::new();
    collect_connector_markers(node, session, &mut markers);
    
    for marker_id in markers {
        svg.push_str(&marker_id);
    }
}

/// Recursively collects all connector marker definitions
fn collect_connector_markers(
    node: &DiagramTreeNode,
    session: &DiagramBuilder,
    markers: &mut std::collections::HashSet<String>,
) {
    if node.entity_type == EntityType::ConnectorShape {
        let connector = session.get_connector(node.entity_id.clone());
        
        if connector.options.arrow_start || connector.options.arrow_end {
            let marker_id = format!("arrow-{}", node.entity_id);
            let color = &connector.options.stroke_color;
            let size = connector.options.arrow_size;
            
            if connector.options.arrow_end {
                markers.insert(format!(
                    r#"<marker id="{}-end" markerWidth="{}" markerHeight="{}" refX="{}" refY="{}" orient="auto" markerUnits="strokeWidth">
                        <path d="M0,0 L0,{} L{},{} z" fill="{}" />
                    </marker>"#,
                    marker_id, size, size, size, size / 2.0,
                    size, size, size / 2.0, color
                ));
            }
            
            if connector.options.arrow_start {
                markers.insert(format!(
                    r#"<marker id="{}-start" markerWidth="{}" markerHeight="{}" refX="0" refY="{}" orient="auto" markerUnits="strokeWidth">
                        <path d="M{},0 L{},{} L{},{} z" fill="{}" />
                    </marker>"#,
                    marker_id, size, size, size / 2.0,
                    size, size, size / 2.0, size, size, color
                ));
            }
        }
    }
    
    for child in &node.children {
        collect_connector_markers(child, session, markers);
    }
}

/// Recursively finds and renders all connectors in the tree
fn render_all_connectors(node: &DiagramTreeNode, session: &DiagramBuilder, svg: &mut String) {
    // If this is a connector, render it
    if node.entity_type == EntityType::ConnectorShape {
        render_connector(session, svg, node.entity_id.clone(), node);
    }
    
    // Recursively process all children
    for child in &node.children {
        render_all_connectors(child, session, svg);
    }
}


fn render_box(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let box_shape = session.get_box(node.entity_id.clone());

    let mut box_content = String::new();

    let stroke_val = if box_shape.box_options.stroke_width == 0.0 {
        String::from("0")
    } else {
        box_shape.box_options.stroke_width.to_string()
    };

    // Draw box rectangle
    match &box_shape.box_options.fill_color {
        Fill::Color(color) => {
            
            box_content.push_str(&format!(
                r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
                size.0, size.1, color,
                box_shape.box_options.stroke_color,
                stroke_val.as_str(),
                box_shape.box_options.border_radius,
                box_shape.box_options.border_radius
            ));
        }
        _ => {
            // TODO: Handle gradients
        }
    }

    // Render children
    if !node.children.is_empty() {
        let first_child = &node.children[0];
        box_content.push_str(&render_node(first_child, session));
    }

    render_with_transform(session, svg, entity_id, &box_content);
}

fn render_vertical_stack(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let mut stack_content = String::new();

    for child in node.children.iter() {
        stack_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &stack_content);
}

fn render_horizontal_stack(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let mut stack_content = String::new();

    for child in node.children.iter() {
        stack_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &stack_content);
}

fn render_polyline(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let polyline_shape = session.get_polyline(node.entity_id.clone());

    let points_str = polyline_shape
        .points
        .iter()
        .map(|&(x, y)| format!("{},{}", x, y))
        .collect::<Vec<_>>()
        .join(" ");

    let polyline_content = format!(
        r#"<polyline points="{}" stroke="{}" stroke-width="{}" fill="none" />"#,
        points_str,
        polyline_shape.line_options.stroke_color,
        polyline_shape.line_options.stroke_width
    );

    render_with_transform(session, svg, entity_id, &polyline_content);
}

fn render_free_container(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let container = session.get_free_container(entity_id.clone());

    let mut container_content = String::new();

    // If there's a background color, draw a rectangle with it
    if let Some(bg_color) = &container.background_color {
        container_content.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" />"#,
            size.0, size.1, bg_color
        ));
    }

    // Render all children
    for child in node.children.iter() {
        container_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &container_content);
}

fn render_constraint_layout_container(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let container = session.get_constraint_layout(entity_id.clone());

    //No background rectangle

    let mut container_content = String::new();

    // Render all children
    for child in node.children.iter() {
        container_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &container_content);
}

fn render_image(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let image_shape = session.get_image(node.entity_id.clone());

    // Handle the Option<String> file_path
    let image_src = match &image_shape.file_path {
        Some(file_path) => {
            if file_path.starts_with("data:") {
                file_path.clone()
            } else {
                match read_image_file_as_data_url(file_path) {
                    Ok(data_url) => data_url,
                    Err(_) => {
                        // Fallback to file path if reading fails
                        file_path.clone()
                    }
                }
            }
        }
        None => {
            // No file path provided, use empty string or placeholder
            String::new()
        }
    };

    let image_content = format!(
        r#"<image x="0" y="0" width="{}" height="{}" xlink:href="{}" />"#,
        size.0, size.1, image_src
    );

    render_with_transform(session, svg, entity_id, &image_content);
}

// Helper function to read an image file and convert it to a data URL
fn read_image_file_as_data_url(file_path: &str) -> Result<String, std::io::Error> {
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    // Read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Determine mime type based on file extension
    let mime_type = match Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream", // Default mime type
    };

    // Encode as base64 and create data URL
    let base64_data = BASE64.encode(&buffer);
    let data_url = format!("data:{};base64,{}", mime_type, base64_data);

    Ok(data_url)
}
// Fixed render_line function for svg_renderer/src/lib.rs
fn render_line(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let line_shape = session.get_line(node.entity_id.clone());
    let mut p_start = Point::new(0.0, 0.0);
    let mut p_end = Point::new(0.0, 0.0);

    match line_shape.start.clone() {
        LinePointReference::Value(x, y) => {
            p_start.x = x;
            p_start.y = y;
        }

        LinePointReference::PointID(id) => {
            let pos = session.get_local_position(id);
            let line_pos = session.get_local_position(line_shape.entity.clone());

            // This is required to avoid applying transformation twice
            p_start.x = pos.0 - line_pos.0; // Convert to line-relative coords
            p_start.y = pos.1 - line_pos.1;
        }
    }

    match line_shape.end.clone() {
        LinePointReference::Value(x, y) => {
            p_end.x = x;
            p_end.y = y;
        }

        LinePointReference::PointID(id) => {
            let pos = session.get_local_position(id);
            let line_pos = session.get_local_position(line_shape.entity.clone());

            // This is required to avoid applying transformation twice
            p_end.x = pos.0 - line_pos.0; // Convert to line-relative coords
            p_end.y = pos.1 - line_pos.1;
        }
    }

    print!("Rendering line!!!!");
    let line_content = format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
        p_start.x,
        p_start.y,
        p_end.x,
        p_end.y,
        line_shape.line_options.stroke_color,
        line_shape.line_options.stroke_width
    );

    render_with_transform(session, svg, entity_id, &line_content);
}
fn render_connector(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let connector = session.get_connector(node.entity_id.clone());
    
    // Get point positions
    let start_pos = session.get_local_position(connector.start_point_id.clone());
    let end_pos = session.get_local_position(connector.end_point_id.clone());
    let connector_pos = session.get_local_position(connector.entity.clone());
    
    // Convert to relative coordinates
    let rel_start_x = start_pos.0 - connector_pos.0;
    let rel_start_y = start_pos.1 - connector_pos.1;
    let rel_end_x = end_pos.0 - connector_pos.0;
    let rel_end_y = end_pos.1 - connector_pos.1;
    
    let mut line_content = String::new();
    
    // Add arrow marker definitions if needed
    if connector.options.arrow_start || connector.options.arrow_end {
        let marker_id = format!("arrow-{}", entity_id);
        line_content.push_str(&format!(
            r#"<defs>
                <marker id="{}-end" markerWidth="{}" markerHeight="{}" refX="{}" refY="{}" orient="auto" markerUnits="strokeWidth">
                    <path d="M0,0 L0,{} L{},{}  z" fill="{}" />
                </marker>
                <marker id="{}-start" markerWidth="{}" markerHeight="{}" refX="0" refY="{}" orient="auto" markerUnits="strokeWidth">
                    <path d="M{},0 L{},{} L{},{}  z" fill="{}" />
                </marker>
            </defs>"#,
            marker_id, 
            connector.options.arrow_size, 
            connector.options.arrow_size,
            connector.options.arrow_size,
            connector.options.arrow_size / 2.0,
            connector.options.arrow_size,
            connector.options.arrow_size,
            connector.options.arrow_size / 2.0,
            connector.options.stroke_color,
            marker_id,
            connector.options.arrow_size,
            connector.options.arrow_size,
            connector.options.arrow_size / 2.0,
            connector.options.arrow_size,
            connector.options.arrow_size,
            connector.options.arrow_size / 2.0,
            connector.options.arrow_size,
            connector.options.arrow_size,
            connector.options.stroke_color
        ));
    }
    
    // Create the line with optional arrow markers
    let marker_start = if connector.options.arrow_start {
        format!(r#" marker-start="url(#arrow-{}-start)""#, entity_id)
    } else {
        String::new()
    };
    
    let marker_end = if connector.options.arrow_end {
        format!(r#" marker-end="url(#arrow-{}-end)""#, entity_id)
    } else {
        String::new()
    };
    
    line_content.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}"{}{} />"#,
        rel_start_x,
        rel_start_y,
        rel_end_x,
        rel_end_y,
        connector.options.stroke_color,
        connector.options.stroke_width,
        marker_start,
        marker_end
    ));
    
    render_with_transform(session, svg, entity_id, &line_content);
}

fn render_rectangle(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let rect_shape = session.get_rectangle(node.entity_id.clone());


    let rect_content = format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
        size.0,
        size.1,
        match &rect_shape.rect_options.fill_color {
            Fill::Color(color) => color.clone(),
            _ => "white".to_string(), // TODO: Handle gradients
        },
        rect_shape.rect_options.stroke_color,
        rect_shape.rect_options.stroke_width,
        rect_shape.rect_options.border_radius,
        rect_shape.rect_options.border_radius
    );

    render_with_transform(session, svg, entity_id, &rect_content);
}

fn render_arrow(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let arrow_shape = session.get_arrow(node.entity_id.clone());

    let arrow_content = format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
        arrow_shape.start.0,
        arrow_shape.start.1,
        arrow_shape.end.0,
        arrow_shape.end.1,
        arrow_shape.arrow_options.stroke_color,
        arrow_shape.arrow_options.stroke_width
    );

    render_with_transform(session, svg, entity_id, &arrow_content);
}

fn render_ellipse(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let ellipse_shape = session.get_ellipse(node.entity_id.clone());

    // Center the ellipse within its own coordinate space
    // The ellipse should be centered within its bounding box
    let cx = ellipse_shape.radius.0; // Center X = radius X
    let cy = ellipse_shape.radius.1; // Center Y = radius Y

    let ellipse_content = format!(
        r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" stroke="{}" stroke-width="{}" fill="{}" />"#,
        cx,
        cy,
        ellipse_shape.radius.0,
        ellipse_shape.radius.1,
        ellipse_shape.ellipse_options.stroke_color,
        ellipse_shape.ellipse_options.stroke_width,
        ellipse_shape.ellipse_options.fill_color
    );

    render_with_transform(session, svg, entity_id, &ellipse_content);
}

fn render_group(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let mut group_content = String::new();

    for child in node.children.iter() {
        group_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &group_content);
}

fn render_table(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let size = session.get_size(entity_id.clone());
    let table_shape = session.get_table(node.entity_id.clone());

    let mut table_content = String::new();

    // Render container rect element
    table_content.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
        size.0,
        size.1,
        table_shape.table_options.fill_color,
        table_shape.table_options.border_color,
        table_shape.table_options.border_width
    ));

    // Render header, cells, lines, etc. Should have been added to the node
    for child in node.children.iter() {
        table_content.push_str(&render_node(child, session));
    }

    render_with_transform(session, svg, entity_id, &table_content);
}

// Then update the render_text function to use it:
fn render_text(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    let text_shape = session.get_text(node.entity_id.clone());
    let size = session.get_size(entity_id.clone());

    let mut text_content = format!(
        r#"<text x="0" y="0" fill="{}" font-size="{}px" font-family="{}">"#,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family
    );

    // Render lines
    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(line_id.clone());
        let line_pos = session.get_local_position(line.entity.clone());

        text_content.push_str(&format!(
            r#"<tspan x="{}" y="{}" alignment-baseline="hanging">"#,
            line_pos.0, line_pos.1
        ));

        let escaped_text = if line.text.trim().is_empty() {
            "&#8203;".to_string()
        } else {
            escape_xml(&line.text)
        };

        text_content.push_str(&escaped_text);
        text_content.push_str("</tspan>");
    }
    text_content.push_str("</text>");

    render_with_transform(session, svg, entity_id, &text_content);
}
fn render_arc(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    node: &DiagramTreeNode,
) {
    use std::f32::consts::PI;

    let arc_shape = session.get_arc(node.entity_id.clone());
    let size = session.get_size(entity_id.clone());

    // Center the arc within its bounding box (like ellipses do)
    let center_x = size.0 / 2.0; // Same as ellipse: radius
    let center_y = size.1 / 2.0; // Same as ellipse: radius

    // Get normalized angles
    let (start_angle, end_angle) = arc_shape.normalize_angles();
    let sweep = arc_shape.angle_sweep();

    // Convert to radians for calculations
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;

    // Calculate start and end points relative to the centered position
    let start_x = center_x + arc_shape.radius * start_rad.cos();
    let start_y = center_y + arc_shape.radius * start_rad.sin();
    let end_x = center_x + arc_shape.radius * end_rad.cos();
    let end_y = center_y + arc_shape.radius * end_rad.sin();

    // Determine if this is a large arc (> 180 degrees)
    let large_arc_flag = if sweep > 180.0 { 1 } else { 0 };

    // Always sweep in positive direction (clockwise in SVG coordinates)
    let sweep_flag = 1;

    let arc_content = if (sweep - 360.0).abs() < 0.1 {
        // Special case for full circles (360°)
        if arc_shape.arc_options.filled {
            // Filled circle
            format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                center_x,
                center_y,
                arc_shape.radius,
                arc_shape.arc_options.fill_color,
                arc_shape.arc_options.stroke_color,
                arc_shape.arc_options.stroke_width
            )
        } else {
            // Unfilled circle
            format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="{}" stroke-width="{}" />"#,
                center_x,
                center_y,
                arc_shape.radius,
                arc_shape.arc_options.stroke_color,
                arc_shape.arc_options.stroke_width
            )
        }
    } else if arc_shape.arc_options.filled {
        // For filled arcs, create a path that includes the center (pie slice)
        format!(
            r#"<path d="M {} {} L {} {} A {} {} 0 {} {} {} {} Z" fill="{}" stroke="{}" stroke-width="{}" />"#,
            center_x,
            center_y, // Move to center
            start_x,
            start_y, // Line to start point
            arc_shape.radius,
            arc_shape.radius, // Arc radii
            large_arc_flag,
            sweep_flag, // Arc flags
            end_x,
            end_y, // Arc end point
            arc_shape.arc_options.fill_color,
            arc_shape.arc_options.stroke_color,
            arc_shape.arc_options.stroke_width
        )
    } else {
        // For unfilled arcs, just draw the arc
        format!(
            r#"<path d="M {} {} A {} {} 0 {} {} {} {}" fill="none" stroke="{}" stroke-width="{}" />"#,
            start_x,
            start_y, // Move to start point
            arc_shape.radius,
            arc_shape.radius, // Arc radii
            large_arc_flag,
            sweep_flag, // Arc flags
            end_x,
            end_y, // Arc end point
            arc_shape.arc_options.stroke_color,
            arc_shape.arc_options.stroke_width
        )
    };

    render_with_transform(session, svg, entity_id, &arc_content);
}

#[test]
fn test_render_arc() {
    let mut session = DiagramBuilder::new();
    let options = ArcOptions {
        fill_color: "blue".to_string(),
        stroke_color: "black".to_string(),
        stroke_width: 2.0,
        filled: false,
    };

    let arc = session.new_arc(
        "arc".to_string(),
        30.0, // radius
        0.0,  // start angle
        90.0, // end angle
        options,
    );

    let node = render_node(&arc, &session);

    assertIsSameSVG(
        &node,
        r##"<g transform="translate(0 0)">
            <path d="M 80 50 A 30 30 0 0 1 50 80" fill="none" stroke="black" stroke-width="2" />
        </g>"##,
    );
    assert!(node.contains("stroke=\"black\""));
}

//test that groups are rendered correctly
#[test]
fn test_render_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group("group".to_string(), Vec::new());
    let node = render_node(&group, &session);
    assert_eq!(node, r#"<g transform="translate(0 0)" ></g>"#);
}

//test that BoxShape with wrapped group is rendered correctly
#[test]
fn test_render_box_with_rect_in_group() {
    let mut session = DiagramBuilder::new();

    let rect = session.new_rectangle(
        "rect1".to_string(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(100.0),
            height_behavior: SizeBehavior::Fixed(50.0),
            fill_color: Fill::Color("black".to_string()),
            stroke_color: String::from("magenta"),
            stroke_width: 1.0,
            border_radius: 1.0,
        },
    );

    let group = session.new_group("group".to_string(), vec![rect]);

    let box_ = session.new_box(
        "box_".to_string(),
        group,
        BoxOptions {
            fill_color: Fill::Color("white".to_string()),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
            padding: 2.0,
            border_radius: 0.0,
            width_behavior: SizeBehavior::Content, // 100 + 2*2 (padding)
            height_behavior: SizeBehavior::Content, // 50 + 2
        },
    );

    layout_tree_node(&mut session, &box_);

    let node = render_node(&box_, &session);
    assertIsSameSVG(
        &node,
        r#"
    <g transform="translate(0 0)" >
        <rect x="0" y="0" width="104" height="54" fill="white" stroke="black" stroke-width="1" rx="0" ry="0" />
            <g transform="translate(2 2)" >
                <g transform="translate(0 0)">
                    <rect x="0" y="0" width="100" height="50" fill="black" stroke="magenta" stroke-width="1" rx="1" ry="1" />
                </g>
            </g>
     </g>"#,
    );
}

// Add this helper function
fn render_with_transform(
    session: &DiagramBuilder,
    svg: &mut String,
    entity_id: EntityID,
    content: &str,
) {
    let transform = session.get_transform(entity_id.clone());
    let transform_str = transform.to_svg_string();
    println!(
        "transform for entity {} {}",
        entity_id.clone(),
        transform_str
    );

    if transform_str.is_empty() {
        // No transform, use simple group
        svg.push_str("<g>");
    } else {
        // Apply transform
        svg.push_str(&format!(r#"<g transform="{}">"#, transform_str));
    }

    svg.push_str(content);
    svg.push_str("</g>");
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn assertIsSameSVG(a: &str, b: &str) {
    // Normalize whitespace and compare
    let strA = a.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let strB = b.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert_eq!(
        strA.replace('\n', "").replace('\r', ""),
        strB.replace('\n', "").replace('\r', "")
    );
}
