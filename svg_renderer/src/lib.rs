use volare_engine_layout::DiagramLayout;
use volare_engine_layout::*;
use volare_engine_layout::utils::*;
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
        ShapeType::VerticalStack(stack) => svg.push_str(&render_stack(stack)),
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



//test for render_text

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape_text::ShapeText;
    use crate::bounding_box::{BoundingBox};
    use crate::location::{Location, PositionableWithBoundingBox};
    use crate::shape_text::TextOptions;


    #[test]
    fn render_text_test() {
         let mut session = crate::session::Session::get_instance();
        session.set_measure_text_fn(|text, _textOptions| {
           (200f64,20f64) 
        });


        let mut text = ShapeText::new();
        text.text = String::from("Hello World\nThis is a test");
        text.location.x = 10.0;
        text.location.y = 20.0;
        text.text_options.font_family = String::from("Arial");
        text.text_options.font_size = 12.0;
        text.text_options.text_color = String::from("black");
        let svg = render_text(&text);

        assert_eq!(svg, "<text fill=\"black\" transform=\"translate(10,20)\" font-family=\"Arial\" font-size=\"12\"><tspan x=\"0\" dy=\"12\" font-family=\"Arial\">Hello\u{a0}World</tspan><tspan x=\"0\" dy=\"12\" font-family=\"Arial\">This\u{a0}is\u{a0}a\u{a0}test</tspan></text>");
    }

    #[test]
    fn render_box_test() {

        //set test text measure fn

        let mut session = crate::session::Session::get_instance();
        session.set_measure_text_fn(|text, _textOptions| {
           (200f64,20f64) 
        });

        let mut box_ = ShapeBox::new();
        box_.location.x = 10.0;
        box_.location.y = 20.0;
        box_.padding = 10.0;
        box_.box_options.fill_color = String::from("white");
        box_.box_options.stroke_color = String::from("black");
        box_.box_options.stroke_width = 1.0;

        let mut text = ShapeText::new();
        text.text = String::from("Hola");
        text.text_options.font_family = String::from("Arial");
        text.text_options.font_size = 12.0;
        text.text_options.text_color = String::from("black");
        box_.elem = Some(Box::new(ShapeType::ShapeText(text)));

        let svg = render_box(&box_);

        assert_eq!(svg, r#"<g transform="translate(10,20)"><rect x="0" y="0" width="220" height="40" fill="white" stroke="black" stroke-width="1" /><g transform="translate(10,10)"><text fill="black" transform="translate(0,0)" font-family="Arial" font-size="12"><tspan x="0" dy="12" font-family="Arial">Hola</tspan></text></g></g>"#);
    }
}

