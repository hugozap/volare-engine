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
    let transformStr = format!("translate({},{})", text.location.x, text.location.y);
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

