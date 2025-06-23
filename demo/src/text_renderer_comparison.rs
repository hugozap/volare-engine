// Text Renderer Comparison Test
// This file demonstrates the differences in text positioning and spacing
// between the SVG and PNG renderers

use image_renderer::PNGRenderer;
use svg_renderer::SVGRenderer;
use volare_engine_layout::{
    renderer_base::Renderer,
    BoxOptions,
    TextOptions,
    Fill,
    DiagramBuilder,
    layout::layout_tree_node,
};

// Import measurement function
use crate::measure_text::measure_text;
use std::fs::File;

pub fn run_comparison_test() -> Result<(), Box<dyn std::error::Error>> {
    // Create session
    let mut session = DiagramBuilder::new();
    session.set_measure_text_fn(measure_text);

    // Create options for the test
    let title_text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 18.0,
        line_width: 500,
        text_color: "black".to_string(),
    };

    let regular_text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 14.0,
        line_width: 500,
        text_color: "black".to_string(),
    };

    let small_text_options = TextOptions {
        font_family: "Roboto".to_string(),
        font_size: 12.0,
        line_width: 500,
        text_color: "black".to_string(),
    };

    let box_options = BoxOptions {
        fill_color: Fill::Color("#EEEEEE".to_string()),
        stroke_color: "#999999".to_string(),
        stroke_width: 1.0,
        padding: 10.0,
        border_radius: 0.0,
    };

    let box_options_no_padding = BoxOptions {
        fill_color: Fill::Color("#EEEEEE".to_string()),
        stroke_color: "#999999".to_string(),
        stroke_width: 1.0,
        padding: 0.0,
        border_radius: 0.0,
    };

    // Create title
    let title = session.new_text(
        "Text Rendering Comparison",
        title_text_options,
    );

    // Create a simple single-line text
    let single_line_text = session.new_text(
        "This is a single line of text.",
        regular_text_options.clone(),
    );
    
    // Create a simple single-line text with a box around it
    let single_line_text_boxed = session.new_text(
        "This is a single line with a box.",
        regular_text_options.clone(),
    );
    let single_line_box = session.new_box(single_line_text_boxed, box_options.clone());

    // Create a simple single-line text with a box around it (no padding)
    let single_line_text_boxed_no_padding = session.new_text(
        "Single line box without padding.",
        regular_text_options.clone(),
    );
    let single_line_box_no_padding = session.new_box(single_line_text_boxed_no_padding, box_options_no_padding.clone());

    // Create a multi-line text example
    let multi_line_text = session.new_text(
        "This is a multi-line text example.\nSecond line of text.\nThird line for testing purposes.",
        regular_text_options.clone(),
    );

    // Create a multi-line text with a box around it
    let multi_line_text_boxed = session.new_text(
        "This is a multi-line text with a box.\nSecond line of text.\nThird line demonstrates the gap.",
        regular_text_options.clone(),
    );
    let multi_line_box = session.new_box(multi_line_text_boxed, box_options.clone());

    // Create a multi-line text with a box (no padding) around it
    let multi_line_text_boxed_no_padding = session.new_text(
        "This is a multi-line text box no padding.\nSecond line without padding.\nThird line gap is more pronounced.",
        regular_text_options.clone(),
    );
    let multi_line_box_no_padding = session.new_box(multi_line_text_boxed_no_padding, box_options_no_padding.clone());

    // Text with linebreaking that causes varying line widths
    let varying_width_text = session.new_text(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eget ligula eu lectus lobortis condimentum. Aliquam nonummy auctor massa.",
        small_text_options.clone(),
    );
    let varying_width_box = session.new_box(varying_width_text, box_options.clone());

    // Create explanation text
    let explanation = session.new_text(
        "The images above demonstrate differences in text positioning between SVG and PNG renderers. Key issues to observe:\n\n1. Vertical spacing between lines in multi-line text\n2. Text centering in boxes with and without padding\n3. Vertical alignment issues with text baseline",
        small_text_options,
    );

    // Arrange all elements in a vertical stack
    let elements = vec![
        title,
        single_line_text,
        single_line_box,
        single_line_box_no_padding,
        multi_line_text,
        multi_line_box,
        multi_line_box_no_padding,
        varying_width_box,
        explanation,
    ];

    let stack = session.ne(elements);

    // Calculate layout
    layout_tree_node(&mut session, &stack);

    // Render to SVG
    let temp_dir = std::env::temp_dir();
    let mut svg_path = temp_dir.clone();
    svg_path.push("text-comparison-test.svg");
    let svg_renderer = SVGRenderer {};
    let mut svg_file = File::create(&svg_path)?;
    svg_renderer.render(&session, &stack, &mut svg_file)?;
    println!("SVG file written to: {}", svg_path.to_str().unwrap());

    // Render to PNG
    let mut png_path = temp_dir.clone();
    png_path.push("text-comparison-test.png");
    let png_renderer = PNGRenderer {};
    let mut png_file = File::create(&png_path)?;
    png_renderer.render(&session, &stack, &mut png_file)?;
    println!("PNG file written to: {}", png_path.to_str().unwrap());

    Ok(())
}