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
    //set the measure_text function
    session.set_measure_text_fn(|text, text_options| {
        //get lines, replace space and tab with utf8 space character
        let textv2 = text.replace(" ", "\u{00A0}").replace("\t", "\u{00A0}\u{00A0}");
        let lines = textv2.lines();
        
        let number_of_lines = lines.clone().count();
  
        
        //get max line length
        let max_line_length = lines.map(|line| line.len()).max().unwrap_or(0);
        let text_height = number_of_lines as f64 * text_options.font_size;
        (text.len() as f64 * text_options.font_size, text_options.font_size)
    });
    let args: Vec<String> = env::args().collect();
    let mut layout = DiagramLayout::new();
    let mut text = ShapeText::new();
    text.text = args[1].clone();
    let mut shape_box = volare_engine_layout::ShapeBox::new();
    shape_box.elem = Some(Box::new(ShapeType::ShapeText(text)));
    shape_box.padding = 10.0;

    layout.children.push(ShapeType::ShapeBox(shape_box));
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



