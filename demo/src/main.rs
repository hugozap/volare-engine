// Create an SVG file with all supported elements


pub mod measure_text;

//import svg_renderer
use svg_renderer::*;
//import layout
use volare_engine_layout::{layout::layout_tree_node, Session, TextOptions, TableOptions, session::DiagramTreeNode};
//import io modules to write to file
use std::{fs::File};
use measure_text::measure_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = Session::new();
    let text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
    };
    session.set_measure_text_fn(measure_text);

    //Create a list of 10 texts
    let mut texts = Vec::new();
    for i in 0..10 {
        let text = session.new_text(&format!("Text hey ☣ {} \nthis is a multiline text", i), text_options.clone());
        texts.push(text);
        //texts.push(get_test_table(&mut session));
    }
    //texts.push(get_test_table(&mut session));
    //Create a table for the texts with 2 columns
    let table = session.new_table(texts, 5, TableOptions::default());

    // Calculate layout
    layout_tree_node(&mut session, &table);

    //create writer to file ~/temp/svg-render-test.svg
    //get path for ~/temp
    let temp_dir = std::env::temp_dir();
    //create path for ~/temp/svg-render-test.svg
    let mut path = temp_dir.clone();
    path.push("svg-render-test.svg");
    let mut file = File::create(path).unwrap();
    let res = render(&session, &table, &mut file);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
        //exit with error code 1
        std::process::exit(1);
    }
    
    //print file contents to console stdout
    let mut path = temp_dir.clone();
    path.push("svg-render-test.svg");
    //let contents = std::fs::read_to_string(path).unwrap();
    //print path name to stdout
    println!("SVG file written to: {}", path.to_str().unwrap());

    Ok(())
}

//function that returns a sample table with 10 elements and 3 columns

fn get_test_table(session: &mut Session) -> DiagramTreeNode {
    let text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
    };
    //Create a list of 10 texts
    let mut texts = Vec::new();
    for i in 0..10 {
        let text = session.new_text(&format!("Text hey {} \nthis is a multiline text", i), text_options.clone());
        texts.push(text);
    }
    //Create a table for the texts with 2 columns
    let table = session.new_table(texts, 3, TableOptions::default());
    table
}