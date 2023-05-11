// Create an SVG file with all supported elements


pub mod measure_text;

//import svg_renderer
use svg_renderer::SVGRenderer;
use volare_engine_layout::renderer_base::Renderer;
use image_renderer::PNGRenderer;

//import layout
use volare_engine_layout::{layout::layout_tree_node, DiagramBuilder, TextOptions, TableOptions, diagram_builder::DiagramTreeNode, EllipseOptions};
//import io modules to write to file
use std::{fs::File};
use measure_text::measure_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = DiagramBuilder::new();
    let text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
    };
    session.set_measure_text_fn(measure_text);

    //Create a list of 10 texts
    let mut table_items = Vec::new();
    for i in 0..10 {
        let text = session.new_text(&format!("Text hey ☣ {} \nthis is a multiline text", i), text_options.clone());
        table_items.push(text);
        //texts.push(get_test_table(&mut session));
    }
    //Add a couple of ellipses
    let ellipse = session.new_elipse((0.0,0.0), (10.0,10.0), EllipseOptions{
       fill_color: "red".to_string(),
       stroke_color: "black".to_string(),
       stroke_width: 1.0,
    });
    
    table_items.push(ellipse);
    
    //Now add 10 ellipses
    for i in 0..10 {
        let ellipse = session.new_elipse((0.0,0.0), (10.0,10.0), EllipseOptions{
            fill_color: "red".to_string(),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
         });
        table_items.push(ellipse);
    }

    //create a paragraph of lorem ipsum
    let lorem_ipsum = br#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla euismod, nunc eget aliquam ultricies, nunc nisl ultricies nunc, vitae aliquam nisl nisl vitae nisl. Nul
    sdfasdfadsfadsfasdfasdfasdfasdfasdfasd asdf asdjf; asdkfja k;sldjfalsjd fjas;kdlfjlasdfj; asdjf; asdfasdfasdlkfj;alksdjfajsdfkasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdf asdjf asdfjajsd fasdjfkajsdfasd
    asdfasdfasdflkja;ksdf asdf a"#;

    //create text shape
    let text = session.new_text(std::str::from_utf8(lorem_ipsum).unwrap(), text_options.clone());
    table_items.push(text);
    
    //Add sample image
    let sampleImage = session.new_image(&getSampleImage(), (200.0, 600.0));
    table_items.push(sampleImage);
    //texts.push(get_test_table(&mut session));
    //Create a table for the texts with 2 columns
    let table = session.new_table(table_items, 5, TableOptions::default());

    // Calculate layout
    layout_tree_node(&mut session, &table);

    //create writer to file ~/temp/svg-render-test.svg
    //get path for ~/temp
    let temp_dir = std::env::temp_dir();
    //create path for ~/temp/svg-render-test.svg
    let mut path = temp_dir.clone();
    //path.push("svg-render-test.svg");
    path.push("svg-render-test.svg");
    let image_renderer = SVGRenderer{};
    let mut file = File::create(path).unwrap();
    let res = image_renderer.render(&session, &table, &mut file);
    //let res = render(&session, &table, &mut file);
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

fn getSampleImage() -> String{
    //load from assets/sample-image.base64 included in the crate
    let image_base64 = include_str!("../assets/sample-image.base64");
    image_base64.to_string()
}

//function that returns a sample table with 10 elements and 3 columns

fn get_test_table(session: &mut DiagramBuilder) -> DiagramTreeNode {
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
    //create a table options object with all defaults except the header color
    let table_options = TableOptions {
        header_fill_color: "red".to_string(),
        ..Default::default()
    };
    //Create a table for the texts with 2 columns
    let table = session.new_table(texts, 3, table_options);
    table
}