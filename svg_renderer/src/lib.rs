use volare_engine_layout::DiagramLayout;
use volare_engine_layout::*;
use volare_engine_layout::diagram_layout::ShapeType;
use std::io::Write;
//use error
use std::io::Error;

pub fn render<W: Write>(layout: &DiagramLayout, stream: &mut W) -> Result<(), Error> {
    let mut svg = String::new();
    let bbox = layout.get_bounding_box();

    svg.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>"));

    svg.push_str(&format!(r#"<svg viewBox="{} {} {} {}">"#, bbox.x, bbox.y, bbox.width, bbox.height));

    for child in &layout.children {
        svg.push_str(&render_shape(child));
    }
    //close svg tag
    svg.push_str("</svg>");
    svg.push_str("\n");
    stream.write_all(svg.as_bytes())?;
    Ok(())
}

fn render_shape(shape: &ShapeType) -> String {
    let mut svg = String::new();
    //call a separate function for each type
    match shape {
        ShapeType::ShapeText(text) => svg.push_str(&render_text(text)),
        ShapeType::ShapeGroup(group) => svg.push_str(&render_group(group)),
        ShapeType::ShapeBox(box_) => svg.push_str(&render_box(box_)),
    }
    svg
}

fn render_text(text: &ShapeText) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(r#"<text x="{}" y="{}" font-size="{}">"#, text.location.x, text.location.y, text.font_size));
    svg.push_str(&text.text);
    svg.push_str("</text>");
    svg
}

fn render_group(group: &ShapeGroup) -> String {
    let mut svg = String::new();
    svg.push_str("<g>");
    for child in &group.children {
        svg.push_str(&render_shape(child));
    }
    svg.push_str("</g>");
    svg
}

fn render_box(box_: &ShapeBox) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" />"#, box_.location.x, box_.location.y, box_.width, box_.height));
    svg
}

