use std::io::Write;
use volare_engine_layout::diagram_builder::DiagramTreeNode;
use volare_engine_layout::*;
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
        r#"<svg width="{}" height="{}" viewBox="{} {} {} {}">"#,root_size.0, root_size.1,
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
        _ => {
            svg.push_str("");
        }
    }

    svg
}

fn render_image(session: &DiagramBuilder, svg: &mut String, entity_id: EntityID, node: &DiagramTreeNode) {
    let size = session.get_size(entity_id);
    let image_shape = session.get_image(node.entity_id);
    let pos = session.get_position(entity_id);
    svg.push_str(&format!(
        r#"<g transform="translate({} {})" >"#,
        pos.0, pos.1
    ));
    //Render base64 image
    //Note: the base64 should start with data:image/png;base64,
    svg.push_str(&format!(r#"<image x="0" y="0" width="{}" height="{}" xlink:href="{}" />"#,
     size.0, 
     size.1, 
     image_shape.image));
    svg.push_str("</g>");
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
