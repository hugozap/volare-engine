use volare_engine_layout::DiagramLayout;
use volare_engine_layout::ShapeText;
use volare_engine_layout::PositionableWithBoundingBox;
use std::env;

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
    layout.children.push(Box::new(text));
    //call trait method get_bounding_box for layout
    let bb = layout.get_bounding_box();
    println!("bounding box: x: {}, y: {}, width: {}, height: {}", bb.x, bb.y, bb.width, bb.height);
   
    
}