// Create an SVG file with all supported elements

pub mod measure_text;

//import svg_renderer
use image_renderer::PNGRenderer;
use svg_renderer::SVGRenderer;
use volare_engine_layout::{renderer_base::Renderer, BoxOptions, GradientStop, LineOptions};

//import layout
use volare_engine_layout::{
    diagram_builder::DiagramTreeNode, layout::layout_tree_node, DiagramBuilder, EllipseOptions,
    TableOptions, TextOptions, Fill,
};
//import io modules to write to file
use measure_text::measure_text;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create session
    let mut session = DiagramBuilder::new();
    let text_options = TextOptions {
        font_family: "AnonymicePro Nerd Font".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
        line_spacing: 0.0,
    };
    session.set_measure_text_fn(measure_text);

    //Create a polyline for a star
    let mut points = Vec::new();
    // Outer vertex
    points.push((10.0, 0.0));
    // Inner vertex
    points.push((16.0, 16.0));
    // Outer vertex
    points.push((0.0, 6.0));
    // Inner vertex
    points.push((20.0, 6.0));
    // Outer vertex
    points.push((4.0, 16.0));
    // Closing the shape by returning to the first point
    points.push((10.0, 0.0));

    let polyline = session.new_polyline(
        points,
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
        },
    );
    
    // Create a more visible polyline - hexagon
    let mut hex_points = Vec::new();
    let hex_size = 50.0;
    for i in 0..6 {
        let angle = (i as f64) * std::f64::consts::PI / 3.0;
        let x = hex_size * angle.cos() + hex_size;
        let y = hex_size * angle.sin() + hex_size;
        hex_points.push((x, y));
    }
    // Close the shape
    hex_points.push(hex_points[0]);
    
    let hexagon = session.new_polyline(
        hex_points,
        LineOptions {
            stroke_color: "blue".to_string(),
            stroke_width: 2.0,
        },
    );

    //Create a table with 10 ellipses
    let mut table_items_ellipses: Vec<DiagramTreeNode> = Vec::new();
    for i in 0..10 {
        let ellipse = session.new_elipse(
            (0.0, 0.0),
            (10.0, 10.0),
            EllipseOptions {
                fill_color: "red".to_string(),
                stroke_color: "black".to_string(),
                stroke_width: 1.0,
            },
        );
        table_items_ellipses.push(ellipse);
    }
    let tableEllipses = session.new_table(table_items_ellipses, 5, TableOptions::default());

    //Create a list of 10 texts
     let mut table_items = Vec::new();
    // table_items.push(tableEllipses);
    // table_items.push(polyline);
    // table_items.push(hexagon);
    // for i in 0..10 {
    //     let text = session.new_text(
    //         &format!("Text hey ☣ {} \nthis is a multiline text", i),
    //         text_options.clone(),
    //     );
    //     table_items.push(text);
    //     //texts.push(get_test_table(&mut session));
    // }
    //Add a couple of ellipses

    //Create an ellipse and wrap it with a box
    let ellipse = session.new_elipse(
        (0.0, 0.0),
        (10.0, 10.0),
        EllipseOptions {
            fill_color: "red".to_string(),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
        },
    );

    //table_items.push(ellipse);

    //Now add 10 ellipses
    for i in 0..10 {
        let ellipse = session.new_elipse(
            (0.0, 0.0),
            (10.0, 10.0),
            EllipseOptions {
                fill_color: "red".to_string(),
                stroke_color: "black".to_string(),
                stroke_width: 1.0,
            },
        );
       // table_items.push(ellipse);
    }

    //create a paragraph of lorem ipsum
    let lorem_ipsum = br#"
  THE adjustment \u{f1878} factor (currently font_size * 0.05) slightly shifts the text vertically to achieve better visual centering. It's a small empirical correction that helps the
  text appear more naturally centered to the human eye, rather than strictly mathematically centered.

  Without this adjustment, the text might appear slightly too high in the box, even when it's mathematically centered according to its metrics. This is particularly noticeable
  with certain fonts or at larger font sizes.

  In essence, it's an optical adjustment that helps the text look properly centered, compensating for the inherent asymmetry in font design and the way our eyes perceive text
  positioning."#;

    //create text shape
    let text = session.new_text(
        std::str::from_utf8(lorem_ipsum).unwrap(),
        text_options.clone(),
    );
    table_items.push(text);

    //Add sample image from file (first instance)
    let sampleImage = session.new_image_from_file("demo/assets/sample.png", (200.0, 200.0));
    table_items.push(sampleImage);
    
    //Add sample image from file
    // The path is relative to where the binary is run
    let file_image = session.new_image_from_file("demo/assets/sample.png", (150.0, 150.0));
    table_items.push(file_image);
    
    // Create a FreeContainer with multiple visual elements at specific positions using the new method
    
    // Create all elements first
    let title_text = session.new_text(
        "FreeContainer Demo",
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 18.0,
            line_width: 100,
            text_color: "#000000".to_string(),
            line_spacing: 0.0,
        },
    );
    
    let red_circle = session.new_elipse(
        (0.0, 0.0),  // center position (will be positioned by container)
        (15.0, 15.0),  // radius
        EllipseOptions {
            fill_color: "#FF0000".to_string(),  // bright red
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        }
    );

    let thetext =  format!(r#"
    The adjustment factor {} (currently font_size * 0.05) slightly shifts the text vertically to achieve better visual centering. It's a small empirical correction that helps the
    text appear more naturally centered to the human eye, rather than strictly mathematically centered.
  
    Without this adjustment, the text might appear slightly too high in the box, even when it's mathematically centered according to its metrics. This is particularly noticeable
    with certain fonts or at larger font sizes.
  
    In essence, it's an optical adjustment that helps the text look properly centered, compensating for the inherent asymmetry in font design and the way our eyes perceive text
    positioning."#, "\u{f1878}");
    
    let blue_text = session.new_text(
        &thetext,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 16.0,
            line_width: 100,
            text_color: "white".to_string(),  // white text
            line_spacing: 0.0,
        },
    );
    
    // Create a box around the blue text
    let box_options = BoxOptions {
        fill_color: Fill::Color("#0000FF".to_string()),  // blue background
        stroke_color: "green".to_string(),  // dark blue border
        stroke_width: 1.0,
        padding: 0.0,
        border_radius: 3.0,
    };
    let blue_box = session.new_box(blue_text, box_options);
    
    let green_ellipse = session.new_elipse(
        (0.0, 0.0),
        (30.0, 20.0),
        EllipseOptions {
            fill_color: "#00CC00".to_string(),  // green
            stroke_color: "#006600".to_string(),  // dark green
            stroke_width: 2.0,
        }
    );
    
    let subtitle = session.new_text(
        "Absolute positioning of elements",
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 12.0,
            line_width: 100,
            text_color: "#555555".to_string(),  // dark gray
            line_spacing: 0.0,
        },
    );
    
    // Create a container with all children at once
    let container_with_elements = session.new_free_container_with_children(vec![
        (title_text, (30.0, 10.0)),
        (red_circle, (40.0, 50.0)),
        (blue_box, (80.0, 40.0)),
        (green_ellipse, (150.0, 70.0)),
        (subtitle, (30.0, 120.0)),
    ]);
    
    // Add styling to the container with more vibrant colors
    let free_container = session.get_free_container_mut(container_with_elements.entity_id);
    free_container.background_color = Some("#FFDDDD".to_string()); // Light red background (more visible)
    free_container.border_color = Some("#FF0000".to_string()); // Bright red border
    free_container.border_width = 5.0; // Thicker border
    
    // Add the FreeContainer to the table
    table_items.push(container_with_elements);
    //texts.push(get_test_table(&mut session));
    //Create a table for the texts with 2 columns
    let mut toptions = TableOptions::default();
    toptions.cell_padding = 2;
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

// Sample image loading is now handled directly through file loading

//function that returns a sample table with 10 elements and 3 columns

/// .
fn get_test_table(session: &mut DiagramBuilder) -> DiagramTreeNode {
    let text_options = TextOptions {
        font_family: "AnonymicePro Nerd Font".to_string(),
        font_size: 12.0,
        line_width: 100,
        text_color: "black".to_string(),
    };
    //Create a list of 10 texts
    let mut texts = Vec::new();
    for i in 0..10 {
        let text = session.new_text(
            &format!("Text hey {} \nthis is a multiline text", i),
            text_options.clone(),
        );
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
