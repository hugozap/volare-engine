use volare_engine_layout::*;
use volare_engine_layout::session::DiagramTreeNode;
use std::cell::RefCell;
use std::io::Write;
//use error
use std::io::Error;


// Entry point for the SVG renderer
// The renderer will write the SVG to the output stream
pub fn render<W: Write>(session_ref: RefCell<Session>, diagram_node: &DiagramTreeNode, stream: &mut W) -> Result<(), Error> {
    
    let mut svg = String::new();
    let session = session_ref.borrow();
    let entity_id = session.get_entity_id(diagram_node.entity_type, diagram_node.index);
    let root_size = session.get_size(entity_id);
    let root_pos = session.get_position(entity_id);

    svg.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>"));

    svg.push_str(&format!(r#"<svg viewBox="{} {} {} {}">"#, root_pos.0, root_pos.1, root_size.0, root_size.1));
    

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
    let pos = session.get_position(entity_id);

    match node.entity_type {
        EntityType::GroupShape => {
            svg.push_str(&format!(r#"<g transform="translate({} {})" >"#, pos.0, pos.1));
            for child in node.children.iter() {
                print!("render_node recursive");
                svg.push_str(render_node(child, session_ref).as_str());
            } 
            svg.push_str("</g>");
        },
        EntityType::BoxShape => {
            let size = session.get_size(entity_id);
            svg.push_str(&format!(r#"<g transform="translate({} {})" >"#, pos.0, pos.1));
            svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" />"#, size.0, size.1));
            //Paint the inner node
            for child in node.children.iter() {
                svg.push_str(render_node(child, session_ref).as_str())
            }
            svg.push_str("</g>");
        },

        _ => {
            svg.push_str("");
        }
    }

    svg

}

// All the layout elements already have their positions updated
// We just need to render them, that is, create the SVG elements.
// fn render_entity(entity: &dyn Entity, session: &Session) -> String {

//     let entity_type = entity.get_type();
//     let entity_id = entity.get_id();
//     let entity = session.get_entity(entity_type, get_entity_index_from_id(entity_id));
//     let svg = match entity_type {
//         EntityType::GroupShape => {
//             let group_shape = entity.as_any().downcast_ref::<ShapeGroup>().unwrap();
//             render_group(session, group_shape)
//         },
//         _ => {
//             String::from("")
//         }
//     };

//     svg
// }


// fn render_group(session: &Session, group: &ShapeGroup) -> String {
//     let mut svg = String::new();
//     let pos = session.get_position(group.entity);
//     let size = session.get_size(group.entity);
//     svg.push_str(&format!(r#"<g transform="translate({} {})" >"#, pos.0, pos.1));
//     for child_id in &group.elements {
//         let entity_type = get_entity_type_from_id(*child_id);
//         let entity_index = get_entity_index_from_id(*child_id);
//         let entity = session.get_entity(entity_type, entity_index);
//         svg.push_str(&render_entity(entity, session));
//     }
//     svg.push_str("</g>");
//     svg
// }


/* 
fn render_text(entity_id: EntityID, session: &Session) -> String {
    let mut svg = String::new();
    //get index from entity id
    let entity_index = get_entity_index_from_id(entity_id);
    //TODO: get_entity should only take the entity id.
    let entity = session.get_entity(EntityType::TextShape, entity_index);
    let text = session.get_text(entity);
    let pos = session.get_position(entity);
    let size = session.get_size(entity);
    let transformStr = format!("translate({},{})", pos.0, pos.1);
    let lines = text.text.lines();
    //replace space and tabs with character u00a0 for each line
    let lines = lines.map(|line| line.replace(" ", "\u{00a0}").replace("\t", "\u{00a0}\u{00A0}"));
    //for each line create a tspan element, with x=0, dy=font size, font family and line text
    let tspan = lines.map(|line| format!(r#"<tspan x="0" dy="{}" font-family="{}">{}</tspan>"#, text.text_options.font_size, text.text_options.font_family, line));

    //concatenate all tspan elements
    let tspan = tspan.collect::<Vec<String>>().join("");
    svg.push_str(&format!(r#"<text fill="{}" transform="{}" font-family="{}" font-size="{}">{}</text>"#,
        text.text_options.text_color,
        transformStr,
        text.text_options.font_family,
        text.text_options.font_size,
        tspan));

    svg
}




fn render_box(box_: &ShapeBox) -> String {
    let mut svg = String::new();
    let transformStr = format!("translate({},{})", box_.location.x, box_.location.y);
    //add a group
    svg.push_str(&format!(r#"<g transform="{}">"#, transformStr));
   
    //if the box has an elem, ignore the width and height and use the elemen bounding box
    if let Some(elem) = &box_.elem {

        let elem_bbox = get_shape_type_bounding_box(elem);
         //add a rectangle:
    svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
        elem_bbox.width + 2.0 * box_.padding,
        elem_bbox.height + 2.0 * box_.padding,
        box_.box_options.fill_color,
        box_.box_options.stroke_color,
        box_.box_options.stroke_width));

        //add a group for the contents, get the position from the box
        let elem_pos = box_.get_element_position();
        let transformStr = format!("translate({},{})", elem_pos.x, elem_pos.y);
        svg.push_str(&format!(r#"<g transform="{}">"#, transformStr));
        svg.push_str(&render_shape(elem));
        svg.push_str("</g>");
    } else {
        //if the box has no elem, render a rectangle using the provided width and height
        svg.push_str(&format!(r#"<rect x="0" y="0" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
        box_.width,
        box_.height,
        box_.box_options.fill_color,
        box_.box_options.stroke_color,
        box_.box_options.stroke_width));
    }
    svg.push_str("</g>");

    svg
}

fn render_vertical_stack(stack: &VerticalStack) -> String {
    let mut svg = String::new();
    let transformStr = format!("translate({},{})", stack.location.x, stack.location.y);
    svg.push_str(&format!(r#"<g transform="{}">"#, transformStr));
    for child in &stack.children {
        svg.push_str(&render_shape(child));
    }
    svg.push_str("</g>");
    svg
}

*/

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
    let box_ = session.new_box(group,  BoxOptions::default());
    let node = render_node(&box_, &RefCell::new(session));
    assert_eq!(node, r#"<g transform="translate(0 0)" ><rect x="0" y="0" width="0" height="0" fill="white" stroke="black" stroke-width="1" /><g transform="translate(0 0)" ></g></g>"#);
}
