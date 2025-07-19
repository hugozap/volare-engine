use std::io::Write;
use volare_engine_layout::diagram_builder::DiagramTreeNode;
use volare_engine_layout::*;
use volare_engine_layout::FreeContainer;
//use error
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
    let root_size = session.get_size(diagram_node.entity_id.clone());
    let root_pos = session.get_position(diagram_node.entity_id.clone());

    svg.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>"
    ));

    svg.push_str(&format!(
        r#"<svg width="{}" height="{}" viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
        root_size.0, root_size.1,
        root_pos.0, root_pos.1, root_size.0, root_size.1
    ));

    svg.push_str(render_node(diagram_node, session).as_str());
    //close svg tag
    svg.push_str("</svg>");
    svg.push_str("\n");
    
    // Properly handle the result from writing to the stream
    stream.write_all(svg.as_bytes())
        .map_err(|e| RendererError::new(&e.to_string()))?;
    
    Ok(())
}
}



// Render a node and its children
fn render_node<'a>(node: &DiagramTreeNode, session: &DiagramBuilder) -> String {
    let mut svg = String::new();

    let entity_id = node.entity_id.clone();
    

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session, &mut svg, entity_id,  node);
        }
        EntityType::BoxShape => {
            render_box(session, &mut svg, entity_id, node);
        }
        EntityType::TextShape => {
            render_text(session, &mut svg, entity_id, node);
        }
        EntityType::VerticalStackShape => {
            render_vertical_stack(session, &mut svg, entity_id, node);
        }
        EntityType::HorizontalStackShape => {
            render_horizontal_stack(session, &mut svg, entity_id, node);
        }
        EntityType::LineShape => {
            render_line(session, &mut svg, entity_id, node);
        }
        EntityType::ArrowShape => {
            render_arrow(session, &mut svg, entity_id, node);
        }
        EntityType::EllipseShape => {
            render_ellipse(session, &mut svg, entity_id, node);
        }
        //table
        EntityType::TableShape => {
            render_table(session, &mut svg, entity_id, node);
        }
        //Image
        EntityType::ImageShape => {
            render_image(session, &mut svg, entity_id, node);
        }
        
        EntityType:: PolyLine => {
            render_polyline(session, &mut svg, entity_id, node);
        }
        
        EntityType::FreeContainer => {
            render_free_container(session, &mut svg, entity_id, node);
        }

        EntityType::RectShape => {
            render_rectangle(session, &mut svg, entity_id, node);
        }

        EntityType::ArcShape => {
            render_arc(session, &mut svg, entity_id, node);
        }

        _ => {}
        
    }

    svg
}


fn render_polyline(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let polyline_shape = session.get_polyline(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());

    // Convert points to a space-separated string
    let points_str = polyline_shape.points.iter()
        .map(|&(x, y)| format!("{},{}", x, y))
        .collect::<Vec<_>>().join(" ");

    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<polyline points="{}" stroke="{}" stroke-width="{}" fill="none" />"#,
        points_str,
        polyline_shape.line_options.stroke_color,
        polyline_shape.line_options.stroke_width));
    svg.push_str("</g>");
}

fn render_free_container(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let container = session.get_free_container(entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    // Open a group for the container with the correct position
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // If there's a background color, draw a rectangle with it
    if let Some(bg_color) = &container.background_color {
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" "#,
            size.0, size.1, bg_color
        ));
        
        // Add border if specified
        if let Some(border_color) = &container.border_color {
            if container.border_width > 0.0 {
                svg.push_str(&format!(
                    r#"stroke="{}" stroke-width="{}" "#,
                    border_color, container.border_width
                ));
            }
        }
        
        svg.push_str("/>");
    } 
    // If there's only a border but no background, draw just the outline
    else if let Some(border_color) = &container.border_color {
        if container.border_width > 0.0 {
            svg.push_str(&format!(
                r#"<rect x="0" y="0" width="{}" height="{}" fill="none" stroke="{}" stroke-width="{}" />"#,
                size.0, size.1, border_color, container.border_width
            ));
        }
    }
    
    // Render children
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str());
    }
    
    // Close the container group
    svg.push_str("</g>");
}


fn render_image(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let image_shape = session.get_image(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    // Determine the image source
    let image_src = if let Some(file_path) = &image_shape.file_path {
        // For file-based images, we'll embed them as data URLs in the SVG
        // We need to read the file, encode it to base64, and create a data URL
        match read_image_file_as_data_url(file_path) {
            Ok(data_url) => data_url,
            Err(err) => {
                eprintln!("Error loading image from file '{}': {}", file_path, err);
                // Fallback to a placeholder for errors
                "data:image/svg+xml;charset=utf-8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22100%22%20height%3D%22100%22%3E%3Crect%20width%3D%22100%22%20height%3D%22100%22%20fill%3D%22%23ddd%22%2F%3E%3Ctext%20x%3D%2250%22%20y%3D%2250%22%20font-family%3D%22sans-serif%22%20font-size%3D%2220%22%20text-anchor%3D%22middle%22%20alignment-baseline%3D%22middle%22%3EImage%20Error%3C%2Ftext%3E%3C%2Fsvg%3E".to_string()
            }
        }
    } else {
        // For base64 images, use the stored image data
        // If it's already a data URL, use it as-is
        image_shape.image.clone()
    };
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // Add the SVG image element with the appropriate source
    svg.push_str(&format!(r#"<image x="0" y="0" width="{}" height="{}" xlink:href="{}" />"#,
        size.0, 
        size.1, 
        image_src));
    
    svg.push_str("</g>");
}

// Helper function to read an image file and convert it to a data URL
fn read_image_file_as_data_url(file_path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;
    use std::path::Path;
    
    // Read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Determine mime type based on file extension
    let mime_type = match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
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

fn render_line(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let line_shape = session.get_line(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // FIXED: Use the actual line start and end coordinates, not size
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
        line_shape.start.0,   // Use actual start point
        line_shape.start.1,
        line_shape.end.0,     // Use actual end point  
        line_shape.end.1,
        line_shape.line_options.stroke_color,
        line_shape.line_options.stroke_width
    ));
    
    svg.push_str("</g>");
}

fn render_rectangle(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let rect_shape = session.get_rectangle(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
     size.0, 
     size.1, 
     rect_shape.rect_options.fill_color,
     rect_shape.rect_options.stroke_color,
     rect_shape.rect_options.stroke_width,
     rect_shape.rect_options.border_radius,
     rect_shape.rect_options.border_radius));
    svg.push_str("</g>");
}
fn render_arrow(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let arrow_shape = session.get_arrow(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // FIXED: Use actual arrow start/end coordinates, not size
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
        arrow_shape.start.0,   // FIXED: was using size.0, size.1
        arrow_shape.start.1,
        arrow_shape.end.0,     
        arrow_shape.end.1,
        arrow_shape.arrow_options.stroke_color,
        arrow_shape.arrow_options.stroke_width
    ));
    
    // TODO: Add actual arrow head rendering
    svg.push_str("</g>");
}

fn render_ellipse(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let ellipse_shape = session.get_ellipse(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // POTENTIAL ISSUE: Using size/2 instead of actual ellipse center and radius
    // Current code: cx=size.0/2, cy=size.1/2, rx=size.0/2, ry=size.1/2
    // Should use: cx=ellipse_shape.center.0, cy=ellipse_shape.center.1, rx=ellipse_shape.radius.0, ry=ellipse_shape.radius.1
    
    svg.push_str(&format!(
        r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" stroke="{}" stroke-width="{}" fill="{}" />"#,
        ellipse_shape.center.0,    // FIXED: Use actual center
        ellipse_shape.center.1,    // FIXED: Use actual center
        ellipse_shape.radius.0,    // FIXED: Use actual radius
        ellipse_shape.radius.1,    // FIXED: Use actual radius
        ellipse_shape.ellipse_options.stroke_color,
        ellipse_shape.ellipse_options.stroke_width,
        ellipse_shape.ellipse_options.fill_color
    ));
    
    svg.push_str("</g>");
}


fn render_group(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID,  node: &DiagramTreeNode ) {
  
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    for child in node.children.iter() {
        print!("render_node recursive");
        svg.push_str(render_node(child, session).as_str());
    }
    svg.push_str("</g>");
}

fn render_table(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let table_shape = session.get_table(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    
    let header_size = session.get_size(table_shape.header_rect.clone());


    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
     //render container rect element
    svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
        size.0, 
        size.1, 
        table_shape.table_options.fill_color,
        table_shape.table_options.border_color,
        table_shape.table_options.border_width
    ));

    // render header, cells, lines, etc. Should have been added to the node
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str());
    }
    svg.push_str("</g>");
}


fn render_box(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id.clone());
    let box_shape = session.get_box(node.entity_id.clone());
   
    let pos = session.get_position(entity_id.clone());
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    
    // svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
    //  size.0, 
    //  size.1, 
    //  box_shape.box_options.fill_color,
    //  box_shape.box_options.stroke_color,
    //  box_shape.box_options.stroke_width,
    //  box_shape.box_options.border_radius,
    //  box_shape.box_options.border_radius));

    match &box_shape.box_options.fill_color {
        Fill::Color(color) => {
            svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
            size.0, 
            size.1, 
            color,
            box_shape.box_options.stroke_color,
            box_shape.box_options.stroke_width,
            box_shape.box_options.border_radius,
            box_shape.box_options.border_radius));
        }
        ,
        Fill::LinearGradient(_linearGrad) => {
            todo!()
        },
        Fill::RadialGradient(_) => todo!(),
    }
     if node.children.len() > 0 {
        // we only render the first child for now
        // The layout phase already positions the wrapped entity with padding, so no need to add extra translate
        let first_child = &node.children[0];
        svg.push_str(render_node(&first_child, session).as_str());
        
    }
    svg.push_str("</g>");
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// Then update the render_text function to use it:

fn render_text(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let text_shape = session.get_text(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    let size = session.get_size(entity_id.clone());
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" data-debug="{}" >"#,
        pos.0, pos.1,
        format!("size: {}, {}, pos: {}, {}", 
            size.0, size.1,
            pos.0, pos.1)
    ));

    svg.push_str(&format!(r#"<text x="0" y="{}" fill="{}" font-size="{}px" font-family="{}" >"#,
        0,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family));
   
    //render lines
    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(line_id.clone());
        let pos = session.get_position(line.entity.clone());
        let lineSize = session.get_size(line.entity.clone());
        
        svg.push_str(&format!(r#"<tspan x="{}" y="{}" fill="{}" font-size="{}px" font-family="{}" alignment-baseline="hanging" data-debug="{}" >"#,
        pos.0,
        pos.1,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family,
        format!("size: {}, {}, pos: {}, {}",
            lineSize.0, lineSize.1,
            pos.0, pos.1)
        ));
        
        // FIXED: Escape XML special characters in text content
        let escaped_text = if line.text.trim().is_empty() {
            "&#8203;".to_string()  // Zero-width space for empty lines
        } else {
            escape_xml(&line.text)  // Escape XML characters
        };
        
        svg.push_str(&escaped_text);
        svg.push_str("</tspan>");
    }
    svg.push_str("</text>");
    svg.push_str("</g>");
}

fn render_vertical_stack(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));

    //render items
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str())
    }
    svg.push_str("</g>");
}

fn render_horizontal_stack(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));

    //render items
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str())
    }
    svg.push_str("</g>");
}
// Fixed render_arc function for svg_renderer/src/lib.rs
fn render_arc(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    use std::f32::consts::PI;
    
    let arc_shape = session.get_arc(node.entity_id.clone());
    let pos = session.get_position(entity_id.clone());
    let size = session.get_size(entity_id.clone());
    
    // Get normalized angles
    let (start_angle, end_angle) = arc_shape.normalize_angles();
    let sweep = arc_shape.angle_sweep();
    
    // Convert to radians for calculations
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;
    
    // Calculate start and end points RELATIVE TO THE CENTER (not absolute)
    let start_x = arc_shape.center.0 + arc_shape.radius * start_rad.cos();
    let start_y = arc_shape.center.1 + arc_shape.radius * start_rad.sin();
    let end_x = arc_shape.center.0 + arc_shape.radius * end_rad.cos();
    let end_y = arc_shape.center.1 + arc_shape.radius * end_rad.sin();
    
    // Determine if this is a large arc (> 180 degrees)
    let large_arc_flag = if sweep > 180.0 { 1 } else { 0 };
    
    // Always sweep in positive direction (clockwise in SVG coordinates)
    let sweep_flag = 1;
    
    svg.push_str(&format!(
        r#"<g transform="translate({} {})">"#,
        pos.0, pos.1  // This translates to the layout position
    ));
    
    if arc_shape.arc_options.filled {
        // For filled arcs, create a path that includes the center (pie slice)
        svg.push_str(&format!(
            r#"<path d="M {} {} L {} {} A {} {} 0 {} {} {} {} Z" fill="{}" stroke="{}" stroke-width="{}" />"#,
            arc_shape.center.0, arc_shape.center.1, // Move to center (relative to translated position)
            start_x, start_y, // Line to start point
            arc_shape.radius, arc_shape.radius, // Arc radii
            large_arc_flag, sweep_flag, // Arc flags
            end_x, end_y, // Arc end point
            arc_shape.arc_options.fill_color,
            arc_shape.arc_options.stroke_color,
            arc_shape.arc_options.stroke_width
        ));
    } else {
        // For unfilled arcs, just draw the arc curve
        svg.push_str(&format!(
            r#"<path d="M {} {} A {} {} 0 {} {} {} {}" fill="none" stroke="{}" stroke-width="{}" />"#,
            start_x, start_y, // Move to start point (NO subtraction)
            arc_shape.radius, arc_shape.radius, // Arc radii
            large_arc_flag, sweep_flag, // Arc flags
            end_x, end_y, // Arc end point (NO subtraction)
            arc_shape.arc_options.stroke_color,
            arc_shape.arc_options.stroke_width
        ));
    }
    
    svg.push_str("</g>");
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
        (50.0, 50.0), // center
        30.0,         // radius
        0.0,          // start angle
        90.0,         // end angle
        options
    );
    
    let node = render_node(&arc, &session);


    assertIsSameSVG(
        &node,
        r##"<g transform="translate(0 0)">
            <path d="M 80 50 A 30 30 0 0 1 50 80" fill="none" stroke="black" stroke-width="2" />
        </g>"##
    );
    assert!(node.contains("stroke=\"black\""));
}

//test that groups are rendered correctly
#[test]
fn test_render_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group(
        "group".to_string(),
        Vec::new());
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
         fill_color:Fill::Color("black".to_string()),
         stroke_color: String::from("magenta"),
         stroke_width: 1.0, border_radius: 1.0 });

    let group = session.new_group(
        "group".to_string(),
        vec![rect]);

    let box_ = session.new_box(
        "box_".to_string(),
        group, BoxOptions{
        fill_color:Fill::Color("white".to_string()),
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
        padding: 2.0,
        border_radius: 0.0,
        width_behavior: SizeBehavior::Content, // 100 + 2*2 (padding)
        height_behavior: SizeBehavior::Content, // 50 + 2
    });

    layout_tree_node(&mut session, &box_);

    let node = render_node(&box_, &session);
    assertIsSameSVG(&node, r#"
    <g transform="translate(0 0)" >
        <rect x="0" y="0" width="104" height="54" fill="white" stroke="black" stroke-width="1" rx="0" ry="0" />
            <g transform="translate(2 2)" >
                <g transform="translate(0 0)">
                    <rect x="0" y="0" width="100" height="50" fill="black" stroke="magenta" stroke-width="1" rx="1" ry="1" />
                </g>
            </g>
     </g>"#);
}

fn assertIsSameSVG(a: &str, b: &str) {
    // Normalize whitespace and compare
    let strA = a.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let strB = b.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert_eq!(strA.replace('\n', "").replace('\r', ""),  strB.replace('\n', "").replace('\r', ""));
}

//test line
#[test]
fn test_render_line() {
    let options = LineOptions{
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
    };
    let mut session = DiagramBuilder::new();
    let line = session.new_line(
        "line".to_string(),
        (0.0,0.0),(0.0,0.0),options);
    
    let node = render_node(&line, &session);
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><line x1="0" y1="0" x2="0" y2="0" stroke="black" stroke-width="1" /></g>"#
    );
}

//test eclipse
#[test]
fn test_render_ellipse() {
    let options = EllipseOptions{
        fill_color: "white".to_string(),
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
    };
    let mut session = DiagramBuilder::new();
    let ellipse = session.new_elipse(
        "ellipse".to_string(),
        (0.0, 0.0), (0.0, 0.0), options);
    
    let node = render_node(&ellipse, &session);
    assertIsSameSVG(
        &node,
        r#"
        <g transform="translate(0 0)" >
            <ellipse cx="0" cy="0" rx="0" ry="0" stroke="black" stroke-width="1" fill="white" />
        </g>"#
    );
}

#[test]
fn test_render_box_rounded_corners_with_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group(
        "group".to_string(),
        Vec::new());
    let box_ = session.new_box(
        "box".to_string(),
        group, BoxOptions{
        fill_color: Fill::Color("white".to_string()),
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
        padding: 0.0,
        border_radius: 5.5,
        width_behavior: SizeBehavior::Content, // 0 + 2*0 (padding)
        height_behavior: SizeBehavior::Content, // 0 + 2*0  
    });
    let node = render_node(&box_, &session);
    assertIsSameSVG(
        &node,
        r#"<g transform="translate(0 0)" >
            <rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" rx="5.5" ry="5.5" />
            <g transform="translate(0 0)" >
            </g>
        </g>"#
    );
}

