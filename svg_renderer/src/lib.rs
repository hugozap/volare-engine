use std::cell::RefCell;
use std::io::Write;
use volare_engine_layout::session::DiagramTreeNode;
use volare_engine_layout::*;
//use error
use std::io::Error;

// Entry point for the SVG renderer
// The renderer will write the SVG to the output stream
pub fn render<W: Write>(
    session: &Session,
    diagram_node: &DiagramTreeNode,
    stream: &mut W,
) -> Result<(), Error> {
    let mut svg = String::new();
    let entity_id = session.get_entity_id(diagram_node.entity_type, diagram_node.index);
    let root_size = session.get_size(entity_id);
    let root_pos = session.get_position(entity_id);

    svg.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>"
    ));

    svg.push_str(&format!(
        r#"<svg viewBox="{} {} {} {}">"#,
        root_pos.0, root_pos.1, root_size.0, root_size.1
    ));

    svg.push_str(render_node(diagram_node, session).as_str());
    //close svg tag
    svg.push_str("</svg>");
    svg.push_str("\n");
    stream.write_all(svg.as_bytes())?;
    Ok(())
}

// Render a node and its children
fn render_node<'a>(node: &DiagramTreeNode, session: &Session) -> String {
    let mut svg = String::new();

    let entity_id = session.get_entity_id(node.entity_type, node.index);
    

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
        _ => {
            svg.push_str("");
        }
    }

    svg
}

fn render_line(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let line_shape = session.get_line(node.index);
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
fn render_arrow(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let arrow_shape = session.get_arrow(node.index);
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

fn render_ellipse(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let ellipse_shape = session.get_ellipse(node.index);
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


fn render_group(session: &Session, svg: &mut String, entity_id: u64,  node: &DiagramTreeNode ) {
  
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


fn render_box(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let box_shape = session.get_box(node.index);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" rx="{}" ry="{}" />"#,
     size.0, 
     size.1, 
     box_shape.box_options.fill_color,
     box_shape.box_options.stroke_color,
     box_shape.box_options.stroke_width,
     box_shape.box_options.border_radius,
     box_shape.box_options.border_radius));
    //Paint the inner node
    for child in node.children.iter() {
        svg.push_str(render_node(child, session).as_str())
    }
    svg.push_str("</g>");
}

fn render_text(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let text_shape = session.get_text(node.index);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    //render lines
    for line in text_shape.lines.iter() {
        let pos = session.get_position(line.entity);
        svg.push_str(&format!(r#"<text x="0" y="{}" fill="{}" font-size="{}" font-family="{}" >"#,
        pos.1,
        text_shape.text_options.text_color,
        text_shape.text_options.font_size,
        text_shape.text_options.font_family));
        svg.push_str(&line.text.as_str());
        svg.push_str("</text>");
    
    }
    svg.push_str("</g>");
}

fn render_vertical_stack(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
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

fn render_horizontal_stack(session: &Session, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
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
    let mut session = Session::new();
    let group = session.new_group(Vec::new());
    let node = render_node(&group, &session);
    assert_eq!(node, r#"<g transform="translate(0 0)" ></g>"#);
}

//test that BoxShape with wrapped group is rendered correctly
#[test]
fn test_render_box_with_group() {
    let mut session = Session::new();
    let group = session.new_group(Vec::new());
    let box_ = session.new_box(group, BoxOptions{
        fill_color: "white".to_string(),
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
    let mut session = Session::new();
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
    let mut session = Session::new();
    let ellipse = session.new_elipse((0.0, 0.0), (0.0, 0.0), options);
    
    let node = render_node(&ellipse, &session);
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><ellipse cx="0" cy="0" rx="0" ry="0" stroke="black" stroke-width="1" fill="white" /></g>"#
    );
}

#[test]
fn test_render_box_rounded_corners_with_group() {
    let mut session = Session::new();
    let group = session.new_group(Vec::new());
    let box_ = session.new_box(group, BoxOptions{
        fill_color: "white".to_string(),
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
