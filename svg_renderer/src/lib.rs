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
    let root_size = session.get_size(diagram_node.entity_id);
    let root_pos = session.get_position(diagram_node.entity_id);

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
    stream.write_all(svg.as_bytes()).map_err(|e| RendererError::new(&e.to_string()));
    Ok(())
}
}



// Render a node and its children
fn render_node<'a>(node: &DiagramTreeNode, session: &DiagramBuilder) -> String {
    let mut svg = String::new();

    let entity_id = node.entity_id;
    

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

        _ => {}
        
    }

    svg
}
fn render_polyline(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let polyline_shape = session.get_polyline(node.entity_id);
    let pos = session.get_position(entity_id);

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
    let size = session.get_size(entity_id);
    let container = session.get_free_container(entity_id);
    let pos = session.get_position(entity_id);
    
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
    let size = session.get_size(entity_id);
    let image_shape = session.get_image(node.entity_id);
    let pos = session.get_position(entity_id);
    
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

fn render_line(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let line_shape = session.get_line(node.entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<line x1="0" y1="0" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
     size.0, 
     size.1, 
     line_shape.line_options.stroke_color,
     line_shape.line_options.stroke_width));
    svg.push_str("</g>");
}
fn render_arrow(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let arrow_shape = session.get_arrow(node.entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<line x1="0" y1="0" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
     size.0, 
     size.1, 
     arrow_shape.arrow_options.stroke_color,
     arrow_shape.arrow_options.stroke_width));
    //TODO: paint arrow head
    svg.push_str("</g>");
}

fn render_ellipse(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let ellipse_shape = session.get_ellipse(node.entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" stroke="{}" stroke-width="{}" fill="{}" />"#,
     size.0/2.0, 
     size.1/2.0, 
     size.0/2.0, 
     size.1/2.0, 
     ellipse_shape.ellipse_options.stroke_color,
     ellipse_shape.ellipse_options.stroke_width,
     ellipse_shape.ellipse_options.fill_color));
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
    let size = session.get_size(entity_id);
    let table_shape = session.get_table(node.entity_id);
    let pos = session.get_position(entity_id);
    
    let header_size = session.get_size(table_shape.header_rect);


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

    //render header element
    svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
        header_size.0, 
        header_size.1, 
        table_shape.table_options.header_fill_color,
        table_shape.table_options.border_color,
        table_shape.table_options.border_width
    ));

    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str());
    }
    svg.push_str("</g>");
}


fn render_box(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let box_shape = session.get_box(node.entity_id);
    let pos = session.get_position(entity_id);
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



    //Paint the inner node
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str())
    }

    svg.push_str("</g>");
}

//Same as box but with support for fill gradients


fn render_text(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let text_shape = session.get_text(node.entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));

    //render parent text container
    svg.push_str(&format!(r#"<text x="0" y="{}" fill="{}" font-size="{}" font-family="{}" >"#,
        0,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family));
   
    //render lines
    for line_id in text_shape.lines.iter() {
        let line = session.get_text_line(*line_id);
        let pos = session.get_position(line.entity);
        let transform = format!("translate({} {})", pos.0, pos.1);
        svg.push_str(&format!(r#"<tspan x="0" dy="{}" transform="{}" fill="{}" font-size="{}" font-family="{}" >"#,
        text_shape.text_options.font_size, 
        transform,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family));
        svg.push_str(&line.text.as_str());
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


//test that groups are rendered correctly
#[test]
fn test_render_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group(Vec::new());
    let node = render_node(&group, &session);
    assert_eq!(node, r#"<g transform="translate(0 0)" ></g>"#);
}

//test that BoxShape with wrapped group is rendered correctly
#[test]
fn test_render_box_with_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group(Vec::new());
    let box_ = session.new_box(group, BoxOptions{
        fill_color:Fill::Color("white".to_string()),
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
        padding: 0.0,
        border_radius: 0.0,
    });
    let node = render_node(&box_, &session);
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" rx="0" ry="0" /><g transform="translate(0 0)" ></g></g>"#
    );
}

//test line
#[test]
fn test_render_line() {
    let options = LineOptions{
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
    };
    let mut session = DiagramBuilder::new();
    let line = session.new_line(options);
    
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
    let ellipse = session.new_elipse((0.0, 0.0), (0.0, 0.0), options);
    
    let node = render_node(&ellipse, &session);
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><ellipse cx="0" cy="0" rx="0" ry="0" stroke="black" stroke-width="1" fill="white" /></g>"#
    );
}

#[test]
fn test_render_box_rounded_corners_with_group() {
    let mut session = DiagramBuilder::new();
    let group = session.new_group(Vec::new());
    let box_ = session.new_box(group, BoxOptions{
        fill_color: Fill::Color("white".to_string()),
        stroke_color: "black".to_string(),
        stroke_width: 1.0,
        padding: 0.0,
        border_radius: 5.5,
    });
    let node = render_node(&box_, &session);
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" rx="5.5" ry="5.5" /><g transform="translate(0 0)" ></g></g>"#
    );
}
