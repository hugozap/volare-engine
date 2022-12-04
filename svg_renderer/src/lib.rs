use std::cell::RefCell;
use std::io::Write;
use volare_engine_layout::session::DiagramTreeNode;
use volare_engine_layout::*;
//use error
use std::io::Error;

// Entry point for the SVG renderer
// The renderer will write the SVG to the output stream
pub fn render<W: Write>(
    session_ref: RefCell<Session>,
    diagram_node: &DiagramTreeNode,
    stream: &mut W,
) -> Result<(), Error> {
    let mut svg = String::new();
    let session = session_ref.borrow();
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

    svg.push_str(render_node(diagram_node, &session_ref).as_str());
    //close svg tag
    svg.push_str("</svg>");
    svg.push_str("\n");
    stream.write_all(svg.as_bytes())?;
    Ok(())
}

/*
 The reason why we get an error with the session lifetime
 even if session is not a static variable, is because the
   render_node function is recursive and the compiler cannot
   infer the lifetime of the session variable.
   The solution is to use the static lifetime for the session.

   We can solve it with refcell doing this:
   let session = RefCell::new(session);
   let session = session.borrow();
   render_node(&session, diagramNode, stream);
   but it is not a good solution because it is not thread safe.

   To receive a refcell we need to define arguments like this:


*/
fn render_node<'a>(node: &DiagramTreeNode, session_ref: &RefCell<Session>) -> String {
    let mut svg = String::new();
    let session = session_ref.borrow();

    let entity_id = session.get_entity_id(node.entity_type, node.index);
    

    match node.entity_type {
        EntityType::GroupShape => {
            render_group(session_ref, &mut svg, entity_id,  node);
        }
        EntityType::BoxShape => {
            render_box(session_ref, &mut svg, entity_id, node);
        }
        EntityType::TextShape => {
            render_text(session_ref, &mut svg, entity_id, node);
        }
        EntityType::VerticalStackShape => {
            render_vertical_stack(session_ref, &mut svg, entity_id, node);
        }
        EntityType::HorizontalStackShape => {
            render_horizontal_stack(session_ref, &mut svg, entity_id, node);
        }
        _ => {
            svg.push_str("");
        }
    }

    svg
}

fn render_group(session_ref: &RefCell<Session>, svg: &mut String, entity_id: u64,  node: &DiagramTreeNode ) {
    let session = session_ref.borrow();
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    for child in node.children.iter() {
        print!("render_node recursive");
        svg.push_str(render_node(child, session_ref).as_str());
    }
    svg.push_str("</g>");
}


fn render_box(session_ref: &RefCell<Session>, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let session = session_ref.borrow();
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
        svg.push_str(render_node(child, session_ref).as_str())
    }
    svg.push_str("</g>");
}

fn render_text(session_ref: &RefCell<Session>, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let session = session_ref.borrow();
    let size = session.get_size(entity_id);
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

fn render_vertical_stack(session_ref: &RefCell<Session>, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let session = session_ref.borrow();
    let size = session.get_size(entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));

    //render items
    for child in node.children.iter() {
        svg.push_str(render_node(child, session_ref).as_str())
    }
    svg.push_str("</g>");
}

fn render_horizontal_stack(session_ref: &RefCell<Session>, svg: &mut String, entity_id: u64, node: &DiagramTreeNode) {
    let session = session_ref.borrow();
    let size = session.get_size(entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));

    //render items
    for child in node.children.iter() {
        svg.push_str(render_node(child, session_ref).as_str())
    }
    svg.push_str("</g>");
}


//test that groups are rendered correctly
#[test]
fn test_render_group() {
    let mut session = Session::new();
    let group = session.new_group(Vec::new());
    let node = render_node(&group, &RefCell::new(session));
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
    let node = render_node(&box_, &RefCell::new(session));
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" rx="0" ry="0" /><g transform="translate(0 0)" ></g></g>"#
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
    let node = render_node(&box_, &RefCell::new(session));
    assert_eq!(
        node,
        r#"<g transform="translate(0 0)" ><rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" rx="5.5" ry="5.5" /><g transform="translate(0 0)" ></g></g>"#
    );
}
