use volare_engine_layout::DiagramLayout;
use volare_engine_layout::ShapeText;
use volare_engine_layout::PositionableWithBoundingBox;
use svg_renderer::*;
use std::env;
use volare_engine_layout::diagram_layout::ShapeType;
/* main function, reads argument and creates a layou text shape and adds it to the diagram layout */

fn main() {
    //get session instance and set text measure method
    let mut session = volare_engine_layout::session::Session::get_instance();
    session.set_measure_text_fn(|text, font_size| {
        return (text.len() as f64 * font_size, font_size);
    });
    let args: Vec<String> = env::args().collect();
    let mut layout = DiagramLayout::new();
    let mut text = ShapeText::new();
    text.text = args[1].clone();
    layout.children.push(ShapeType::ShapeText(text));
    //call trait method get_bounding_box for layout
    let bb = layout.get_bounding_box();
    println!("bounding box: x: {}, y: {}, width: {}, height: {}", bb.x, bb.y, bb.width, bb.height);
    
    // Create stream to file "/tmp/test.svg"

    let mut stream = std::fs::File::create("/tmp/test.svg").unwrap();
    //call render function
    svg_renderer::render(&layout, &mut stream).unwrap();

    //print contents of file
    let contents = std::fs::read_to_string("/tmp/test.svg").unwrap();
    println!("Contents of file: {}", contents);
    
}



