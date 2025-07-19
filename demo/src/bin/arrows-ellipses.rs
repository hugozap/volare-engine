// src/bin/arrows_ellipses_demo.rs
// Demo program to test arrows and ellipses positioning

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏹 Arrows and Ellipses Demo Starting...\n");

    // Create output directory
    let output_dir = std::env::temp_dir().join("arrows_ellipses_demo");
    std::fs::create_dir_all(&output_dir)?;

    // Generate various demos to test positioning
    generate_basic_arrows_demo(&output_dir)?;
    generate_basic_ellipses_demo(&output_dir)?;
    generate_mixed_positioning_demo(&output_dir)?;
    generate_free_container_test(&output_dir)?;

    println!("\n✅ All arrows and ellipses demos generated successfully!");
    println!("📁 Files saved in: {}", output_dir.display());

    Ok(())
}

// Demo 1: Basic Arrows in Different Layouts
fn generate_basic_arrows_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","arrows_hstack","arrows_free"],"h_align":"center"}
{"id":"title","type":"text","content":"🏹 Arrow Positioning Test","font_size":24,"color":"darkblue"}
{"id":"arrows_hstack","type":"vstack","children":["hstack_label","hstack_arrows"],"h_align":"center"}
{"id":"hstack_label","type":"text","content":"Arrows in HStack","font_size":16,"color":"#333"}
{"id":"hstack_arrows","type":"hstack","children":["arrow1","arrow2","arrow3"],"v_align":"center"}
{"id":"arrow1","type":"line","start_x":0,"start_y":0,"end_x":50,"end_y":0,"stroke_color":"red","stroke_width":3}
{"id":"arrow2","type":"line","start_x":0,"start_y":0,"end_x":0,"end_y":50,"stroke_color":"green","stroke_width":3}
{"id":"arrow3","type":"line","start_x":0,"start_y":0,"end_x":50,"end_y":50,"stroke_color":"blue","stroke_width":3}
{"id":"arrows_free","type":"vstack","children":["free_label","free_container"],"h_align":"center"}
{"id":"free_label","type":"text","content":"Arrows in Free Container","font_size":16,"color":"#333"}
{"id":"free_container","type":"free_container","width":300,"height":200,"children":["arrow4","arrow5","arrow6"]}
{"id":"arrow4","type":"line","start_x":50,"start_y":50,"end_x":100,"end_y":50,"stroke_color":"red","stroke_width":3,"x":0,"y":0}
{"id":"arrow5","type":"line","start_x":150,"start_y":50,"end_x":150,"end_y":100,"stroke_color":"green","stroke_width":3,"x":0,"y":0}
{"id":"arrow6","type":"line","start_x":200,"start_y":50,"end_x":250,"end_y":100,"stroke_color":"blue","stroke_width":3,"x":0,"y":0}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("01_basic_arrows.svg"), "Basic Arrows Test")
}

// Demo 2: Basic Ellipses in Different Layouts
fn generate_basic_ellipses_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","ellipses_hstack","ellipses_free"],"h_align":"center"}
{"id":"title","type":"text","content":"⭕ Ellipse Positioning Test","font_size":24,"color":"darkgreen"}
{"id":"ellipses_hstack","type":"vstack","children":["hstack_label","hstack_ellipses"],"h_align":"center"}
{"id":"hstack_label","type":"text","content":"Ellipses in HStack","font_size":16,"color":"#333"}
{"id":"hstack_ellipses","type":"hstack","children":["ellipse1","ellipse2","ellipse3"],"v_align":"center"}
{"id":"ellipse1","type":"ellipse","cx":20,"cy":20,"rx":20,"ry":20,"fill":"red","stroke":"darkred","stroke_width":2}
{"id":"ellipse2","type":"ellipse","cx":15,"cy":30,"rx":15,"ry":30,"fill":"green","stroke":"darkgreen","stroke_width":2}
{"id":"ellipse3","type":"ellipse","cx":30,"cy":15,"rx":30,"ry":15,"fill":"blue","stroke":"darkblue","stroke_width":2}
{"id":"ellipses_free","type":"vstack","children":["free_label","free_ellipses_container"],"h_align":"center"}
{"id":"free_label","type":"text","content":"Ellipses in Free Container","font_size":16,"color":"#333"}
{"id":"free_ellipses_container","type":"free_container","width":300,"height":200,"children":["ellipse4","ellipse5","ellipse6"]}
{"id":"ellipse4","type":"ellipse","cx":30,"cy":30,"rx":25,"ry":25,"fill":"red","stroke":"darkred","stroke_width":2,"x":20,"y":20}
{"id":"ellipse5","type":"ellipse","cx":30,"cy":30,"rx":20,"ry":35,"fill":"green","stroke":"darkgreen","stroke_width":2,"x":120,"y":20}
{"id":"ellipse6","type":"ellipse","cx":30,"cy":30,"rx":35,"ry":20,"fill":"blue","stroke":"darkblue","stroke_width":2,"x":220,"y":20}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("02_basic_ellipses.svg"), "Basic Ellipses Test")
}

// Demo 3: Mixed Positioning - Arrows and Ellipses Together
fn generate_mixed_positioning_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","mixed_demo"],"h_align":"center"}
{"id":"title","type":"text","content":"Mixed Arrows & Ellipses","font_size":24,"color":"darkorange"}
{"id":"mixed_demo","type":"free_container","width":400,"height":300,"children":["target1","target2","target3","arrow_to_1","arrow_to_2","arrow_to_3","labels"]}
{"id":"target1","type":"ellipse","cx":25,"cy":25,"rx":20,"ry":20,"fill":"red","stroke":"darkred","stroke_width":3,"x":50,"y":50}
{"id":"target2","type":"ellipse","cx":25,"cy":25,"rx":20,"ry":20,"fill":"green","stroke":"darkgreen","stroke_width":3,"x":200,"y":50}
{"id":"target3","type":"ellipse","cx":25,"cy":25,"rx":20,"ry":20,"fill":"blue","stroke":"darkblue","stroke_width":3,"x":125,"y":150}
{"id":"arrow_to_1","type":"line","start_x":10,"start_y":10,"end_x":65,"end_y":65,"stroke_color":"black","stroke_width":2,"x":10,"y":10}
{"id":"arrow_to_2","type":"line","start_x":10,"start_y":10,"end_x":75,"end_y":65,"stroke_color":"black","stroke_width":2,"x":150,"y":10}
{"id":"arrow_to_3","type":"line","start_x":10,"start_y":10,"end_x":40,"end_y":65,"stroke_color":"black","stroke_width":2,"x":100,"y":100}
{"id":"labels","type":"vstack","children":["label1","label2"],"h_align":"left","x":50,"y":250}
{"id":"label1","type":"text","content":"• Arrows should point toward ellipses","font_size":12,"color":"#333"}
{"id":"label2","type":"text","content":"• If positioning works, arrows point to targets","font_size":12,"color":"#333"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("03_mixed_positioning.svg"), "Mixed Arrows & Ellipses")
}

// Demo 4: Free Container Edge Cases
fn generate_free_container_test(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","edge_cases"],"h_align":"center"}
{"id":"title","type":"text","content":"🔍 Edge Case Testing","font_size":24,"color":"darkviolet"}
{"id":"edge_cases","type":"free_container","width":500,"height":400,"children":["corner_ellipses","center_cross","coordinate_test"]}
{"id":"corner_ellipses","type":"free_container","width":200,"height":200,"children":["tl_ellipse","tr_ellipse","bl_ellipse","br_ellipse"],"x":20,"y":20}
{"id":"tl_ellipse","type":"ellipse","cx":15,"cy":15,"rx":10,"ry":10,"fill":"red","stroke":"darkred","stroke_width":1,"x":0,"y":0}
{"id":"tr_ellipse","type":"ellipse","cx":15,"cy":15,"rx":10,"ry":10,"fill":"green","stroke":"darkgreen","stroke_width":1,"x":170,"y":0}
{"id":"bl_ellipse","type":"ellipse","cx":15,"cy":15,"rx":10,"ry":10,"fill":"blue","stroke":"darkblue","stroke_width":1,"x":0,"y":170}
{"id":"br_ellipse","type":"ellipse","cx":15,"cy":15,"rx":10,"ry":10,"fill":"orange","stroke":"darkorange","stroke_width":1,"x":170,"y":170}
{"id":"center_cross","type":"free_container","width":100,"height":100,"children":["h_line","v_line","center_dot"],"x":250,"y":50}
{"id":"h_line","type":"line","start_x":0,"start_y":50,"end_x":100,"end_y":50,"stroke_color":"black","stroke_width":2,"x":0,"y":0}
{"id":"v_line","type":"line","start_x":50,"start_y":0,"end_x":50,"end_y":100,"stroke_color":"black","stroke_width":2,"x":0,"y":0}
{"id":"center_dot","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"red","stroke":"darkred","stroke_width":1,"x":45,"y":45}
{"id":"coordinate_test","type":"vstack","children":["coord_label","coord_details"],"h_align":"left","x":50,"y":300}
{"id":"coord_label","type":"text","content":"Coordinate System Test:","font_size":14,"color":"#333"}
{"id":"coord_details","type":"vstack","children":["detail1","detail2","detail3"],"h_align":"left"}
{"id":"detail1","type":"text","content":"• Corner ellipses should be at actual corners","font_size":11,"color":"#666"}
{"id":"detail2","type":"text","content":"• Cross lines should intersect at red dot center","font_size":11,"color":"#666"}
{"id":"detail3","type":"text","content":"• All shapes should be positioned as specified","font_size":11,"color":"#666"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("04_edge_cases.svg"), "Edge Case Testing")
}

// Helper function to generate SVG from JSONL
fn generate_svg_from_jsonl(
    jsonl_input: &str,
    output_path: std::path::PathBuf,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📄 Generating: {}", description);

    // Parse the JSON Lines
    let mut parser = parser::JsonLinesParser::new();
    let root_id = parser.parse_string(jsonl_input)?;

    // Create a fresh builder for parsing
    let mut parse_builder = DiagramBuilder::new();
    parse_builder.set_measure_text_fn(measure_text_svg_character_advance);

    // Build the diagram
    let diagram = parser.build(&root_id, &mut parse_builder)?;

    // Calculate layout
    layout::layout_tree_node(&mut parse_builder, &diagram);

    // Render to SVG
    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&output_path)?;
    svg_renderer.render(&parse_builder, &diagram, &mut svg_file)?;

    println!("    ✅ Saved: {}", output_path.file_name().unwrap().to_str().unwrap());

    Ok(())
}
