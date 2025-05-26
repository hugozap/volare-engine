// Create an SVG file with all supported elements

//import svg_renderer
use image_renderer::PNGRenderer;
use svg_renderer::SVGRenderer;
use volare_engine_layout::{renderer_base::Renderer, BoxOptions, GradientStop, LineOptions};
use demo::measure_text::measure_text_svg;

//import layout
use volare_engine_layout::{
    diagram_builder::DiagramTreeNode, layout::layout_tree_node, DiagramBuilder, EllipseOptions,
    TableOptions, TextOptions, Fill,
};
//import io modules to write to file
use std::fs::File;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = DiagramBuilder::new();
  
    session.set_measure_text_fn(measure_text_svg);

    let mut table_items = Vec::new();

    let thetext =  br#"
    The adjustment factor (currently font_size * 0.05) slightly shifts the text vertically to achieve better visual centering. It's a small empirical correction that helps the
    text appear more naturally centered to the human eye, rather than strictly mathematically centered.

  
    Without this adjustment, the text might appear slightly too high in the box, even when it's mathematically centered according to its metrics. This is particularly noticeable
    with certain fonts or at larger font sizes.

  
    In essence, it's an optical adjustment that helps the text look properly centered, compensating for the inherent asymmetry in font design and the way our eyes perceive text
    positioning."#;
    
    let blue_text = session.new_text(
        std::str::from_utf8(thetext).unwrap(),
        TextOptions {
            font_family: "Roboto".to_string(),
            font_size: 16.0,
            line_width: 100,
            text_color: "white".to_string(),  // white text
        },
    );

    let box_options = BoxOptions {
        fill_color: Fill::Color("#0000FF".to_string()),  // blue background
        stroke_color: "#000066".to_string(),  // dark blue border
        stroke_width: 2.0,
        padding: 0.0,
        border_radius: 3.0,
    };

    let box1 = session.new_box(blue_text, box_options);

    
    // Add the FreeContainer to the table
    // table_items.push(container_with_elements);

    table_items.push(box1);
    //texts.push(get_test_table(&mut session));
    //Create a table for the texts with 2 columns
    let mut toptions = TableOptions::default();
    toptions.cell_padding = 5;
    let table = session.new_table(table_items, 5, toptions);

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
    let mut png_path = temp_dir.clone();
    png_path.push("png-render-test.png");
    let png_renderer = PNGRenderer {};
    let mut png_file = File::create(&png_path).unwrap();
    let png_res = png_renderer.render(&session, &table, &mut png_file);
    if png_res.is_err() {
        println!("PNG Render Error: {}", png_res.err().unwrap());
        std::process::exit(1);
    }
    println!("PNG file written to: {}", png_path.to_str().unwrap());

    Ok(())
}
