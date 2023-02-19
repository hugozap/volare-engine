// Create an SVG file with all supported elements


//import svg_renderer
use svg_renderer::*;
//import layout
use volare_engine_layout::{layout::*, Session, TextOptions};
//import io modules to write to file
use std::{fs::File, cell::RefCell};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = Session::new();
    let textOptions = TextOptions {
        font_family: "Arial".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
    };
    session.set_measure_text_fn(|text, options| -> (f64, f64) {
        (100.0,100.0)
    });

    let mut text = session.new_text("Hello World", textOptions);
    let mut group = session.new_group(vec![text]);

    //create writer to file ~/temp/svg-render-test.svg
    //get path for ~/temp
    let temp_dir = std::env::temp_dir();
    //create path for ~/temp/svg-render-test.svg
    let mut path = temp_dir.clone();
    path.push("svg-render-test.svg");
    let mut file = File::create(path).unwrap();
    render(RefCell::new(session), &group, &mut file);
    
    //print file contents to console stdout
    let mut path = temp_dir.clone();
    path.push("svg-render-test.svg");
    //let contents = std::fs::read_to_string(path).unwrap();
    //print path name to stdout
    println!("SVG file written to: {}", path.to_str().unwrap());

    Ok(())
}