// Create an SVG file with all supported elements

//import svg_renderer
// use image_renderer::PNGRenderer;
use demo::measure_text::measure_text_ultra_tight;
use svg_renderer::SVGRenderer;
use volare_engine_layout::{renderer_base::Renderer, BoxOptions, GradientStop, LineOptions};

//import layout
use volare_engine_layout::{
    diagram_builder::DiagramTreeNode, layout::layout_tree_node, DiagramBuilder, EllipseOptions,
    Fill, TableOptions, TextOptions,
};
//import io modules to write to file
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = DiagramBuilder::new();

    session.set_measure_text_fn(measure_text_ultra_tight);

    let mut table_items = Vec::new();

    let thetext =  br#""Lorem ipsum dolor sit amet, consectetur adipiscing elit.
otra linea texto
otra mas...
It could also be compensating for the fact that pos includes
cumulative scaling from parent transforms, 
and you need to pass down the "base" position to child renderers.
The comment "Calculate absolute position without any scaling" suggests the goal is to get back to unscaled coordinates that the child rendering pipeline expects.""#;

    let textOpts = TextOptions {
        font_family: "AnonymicePro Nerd Font".to_string(),
        font_size: 16.0,
        line_width: 100,
        text_color: "white".to_string(), // white text
        line_spacing: 5.0,
        ..Default::default()
    };

    let blue_text = session.new_text(
        "blue_text".to_string(),
        std::str::from_utf8(thetext).unwrap(),
        textOpts.clone(),
    );

    // debug_text_measurement(std::str::from_utf8(thetext).unwrap(), &textOpts.clone());

    let box_options = BoxOptions {
        fill_color: Fill::Color("#0000FF".to_string()), // blue background
        stroke_color: "#000066".to_string(),            // dark blue border
        stroke_width: 1.0,
        //TODO: falta tener en cuenta padding al momento de hacer layout de elementos de box
        padding: 50.0,
        border_radius: 0.0,
        width_behavior: volare_engine_layout::SizeBehavior::Fixed(200.0), // fixed width
        height_behavior: volare_engine_layout::SizeBehavior::Content,     // auto height
    };

    let box1 = session.new_box("box1".to_string(), blue_text, box_options);

    // Add the FreeContainer to the table
    // table_items.push(container_with_elements);

    table_items.push(box1);
    //texts.push(get_test_table(&mut session));
    //Create a table for the texts with 2 columns
    let mut toptions = TableOptions::default();
    toptions.cell_padding = 5;
    let table = session.new_table("text_table".to_string(), table_items, 5, toptions);

    // Calculate layout
    layout_tree_node(&mut session, &table);

    //create writer to file ~/temp/svg-render-test.svg
    //get path for ~/temp
    let temp_dir = std::env::temp_dir();
    //create path for ~/temp/svg-render-test.svg
    // Render SVG

    let mut svg_path = temp_dir.clone();
    svg_path.push("svg-render-test.svg");
    let svg_renderer = SVGRenderer {};
    let mut svg_file = File::create(&svg_path).unwrap();
    let svg_res = svg_renderer.render(&session, &table, &mut svg_file);
    if svg_res.is_err() {
        println!("SVG Render Error: {}", svg_res.err().unwrap());
        std::process::exit(1);
    }
    println!("SVG file written to: {}", svg_path.to_str().unwrap());

    // Render PNG
    // let mut png_path = temp_dir.clone();
    // png_path.push("png-render-test.png");
    // let png_renderer = PNGRenderer {};
    // let mut png_file = File::create(&png_path).unwrap();
    // let png_res = png_renderer.render(&session, &table, &mut png_file);
    // if png_res.is_err() {
    //     println!("PNG Render Error: {}", png_res.err().unwrap());
    //     std::process::exit(1);
    // }
    // println!("PNG file written to: {}", png_path.to_str().unwrap());

    Ok(())
}
