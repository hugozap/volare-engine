// src/bin/arc_shapes_demo.rs
// Demo program showcasing Arc, Semicircle, and Quarter Circle shapes

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌀 Arc Shapes Demo Starting...\n");

    // Create output directory
    let output_dir = std::env::temp_dir().join("arc_shapes_demo");
    std::fs::create_dir_all(&output_dir)?;

    // Generate various arc shape demonstrations
    generate_basic_arcs_demo(&output_dir)?;
    generate_semicircles_demo(&output_dir)?;
    generate_quarter_circles_demo(&output_dir)?;
    generate_filled_arcs_demo(&output_dir)?;
    generate_complex_arc_patterns(&output_dir)?;
    generate_arc_dashboard(&output_dir)?;

    println!("\n✅ All arc shape demos generated successfully!");
    println!("📁 Files saved in: {}", output_dir.display());

    Ok(())
}

// Demo 1: Basic Arc Shapes
fn generate_basic_arcs_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","arcs_container"],"h_align":"center"}
{"id":"title","type":"text","content":"🌀 Basic Arc Shapes","font_size":24,"color":"darkblue"}
{"id":"arcs_container","type":"free_container","width":900,"height":400,"children":["arc_45","arc_90","arc_180","arc_270","arc_300","labels"]}
{"id":"arc_45","type":"arc","cx":0,"cy":0,"radius":40,"start_angle":0,"end_angle":45,"stroke_color":"#ff4444","stroke_width":3,"x":80,"y":120}
{"id":"arc_90","type":"arc","cx":0,"cy":0,"radius":40,"start_angle":0,"end_angle":90,"stroke_color":"#44ff44","stroke_width":3,"x":200,"y":120}
{"id":"arc_180","type":"arc","cx":0,"cy":0,"radius":40,"start_angle":0,"end_angle":180,"stroke_color":"#4444ff","stroke_width":3,"x":320,"y":120}
{"id":"arc_270","type":"arc","cx":0,"cy":0,"radius":40,"start_angle":0,"end_angle":270,"stroke_color":"#ff44ff","stroke_width":3,"x":440,"y":120}
{"id":"arc_300","type":"arc","cx":0,"cy":0,"radius":40,"start_angle":0,"end_angle":300,"stroke_color":"#44ffff","stroke_width":3,"x":560,"y":120}
{"id":"labels","type":"vstack","children":["label_45","label_90","label_180","label_270","label_300"],"h_align":"left","x":50,"y":320}
{"id":"label_45","type":"text","content":"• 45° Arc (Red)","font_size":14,"color":"#ff4444"}
{"id":"label_90","type":"text","content":"• 90° Arc (Green)","font_size":14,"color":"#44ff44"}
{"id":"label_180","type":"text","content":"• 180° Arc (Blue)","font_size":14,"color":"#4444ff"}
{"id":"label_270","type":"text","content":"• 270° Arc (Magenta)","font_size":14,"color":"#ff44ff"}
{"id":"label_300","type":"text","content":"• 300° Arc (Cyan)","font_size":14,"color":"#44ffff"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("01_basic_arcs.svg"), "Basic Arc Shapes")
}

// Demo 2: Semicircles in Different Orientations
fn generate_semicircles_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","semicircles_layout"],"h_align":"center"}
{"id":"title","type":"text","content":"🔄 Semicircle Variations","font_size":24,"color":"darkgreen"}
{"id":"semicircles_layout","type":"free_container","width":600,"height":350,"children":["semi_up","semi_down","semi_left","semi_right","descriptions"]}
{"id":"semi_up","type":"semicircle","cx":150,"cy":150,"radius":50,"facing_up":true,"stroke_color":"#e74c3c","stroke_width":4,"filled":false,"x":0,"y":0}
{"id":"semi_down","type":"semicircle","cx":450,"cy":150,"radius":50,"facing_up":false,"stroke_color":"#3498db","stroke_width":4,"filled":false,"x":0,"y":0}
{"id":"semi_left","type":"arc","cx":150,"cy":300,"radius":50,"start_angle":90,"end_angle":270,"stroke_color":"#f39c12","stroke_width":4,"x":0,"y":0}
{"id":"semi_right","type":"arc","cx":450,"cy":300,"radius":50,"start_angle":270,"end_angle":90,"stroke_color":"#9b59b6","stroke_width":4,"x":0,"y":0}
{"id":"descriptions","type":"vstack","children":["desc1","desc2","desc3","desc4"],"h_align":"left","x":50,"y":50}
{"id":"desc1","type":"text","content":"Top Semicircle (0°-180°)","font_size":12,"color":"#e74c3c"}
{"id":"desc2","type":"text","content":"Bottom Semicircle (180°-360°)","font_size":12,"color":"#3498db"}
{"id":"desc3","type":"text","content":"Left Semicircle (90°-270°)","font_size":12,"color":"#f39c12"}
{"id":"desc4","type":"text","content":"Right Semicircle (270°-90°)","font_size":12,"color":"#9b59b6"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("02_semicircles.svg"), "Semicircle Variations")
}

// Demo 3: Quarter Circles in All Quadrants
fn generate_quarter_circles_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","quarters_container"],"h_align":"center"}
{"id":"title","type":"text","content":"📐 Quarter Circle Quadrants","font_size":24,"color":"darkorchid"}
{"id":"quarters_container","type":"free_container","width":400,"height":400,"children":["q1","q2","q3","q4","center_point","quadrant_labels"]}
{"id":"q1","type":"quarter_circle","cx":200,"cy":200,"radius":80,"quadrant":1,"stroke_color":"#e74c3c","stroke_width":5,"filled":false,"x":0,"y":0}
{"id":"q2","type":"quarter_circle","cx":200,"cy":200,"radius":80,"quadrant":2,"stroke_color":"#2ecc71","stroke_width":5,"filled":false,"x":0,"y":0}
{"id":"q3","type":"quarter_circle","cx":200,"cy":200,"radius":80,"quadrant":3,"stroke_color":"#3498db","stroke_width":5,"filled":false,"x":0,"y":0}
{"id":"q4","type":"quarter_circle","cx":200,"cy":200,"radius":80,"quadrant":4,"stroke_color":"#f39c12","stroke_width":5,"filled":false,"x":0,"y":0}
{"id":"center_point","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"black","x":195,"y":195}
{"id":"quadrant_labels","type":"vstack","children":["q1_label","q2_label","q3_label","q4_label"],"h_align":"left","x":50,"y":50}
{"id":"q1_label","type":"text","content":"Q1: Top-Right (0°-90°)","font_size":12,"color":"#e74c3c"}
{"id":"q2_label","type":"text","content":"Q2: Top-Left (90°-180°)","font_size":12,"color":"#2ecc71"}
{"id":"q3_label","type":"text","content":"Q3: Bottom-Left (180°-270°)","font_size":12,"color":"#3498db"}
{"id":"q4_label","type":"text","content":"Q4: Bottom-Right (270°-360°)","font_size":12,"color":"#f39c12"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("03_quarter_circles.svg"), "Quarter Circle Quadrants")
}

// Demo 4: Filled vs Unfilled Arcs
fn generate_filled_arcs_demo(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","filled_comparison"],"h_align":"center"}
{"id":"title","type":"text","content":"🎨 Filled vs Unfilled Arcs","font_size":24,"color":"darkred"}
{"id":"filled_comparison","type":"hstack","children":["unfilled_section","filled_section"],"v_align":"center"}
{"id":"unfilled_section","type":"vstack","children":["unfilled_label","unfilled_arcs"],"h_align":"center"}
{"id":"unfilled_label","type":"text","content":"Outline Only","font_size":16,"color":"#333"}
{"id":"unfilled_arcs","type":"hstack","children":["arc_outline_1","arc_outline_2","arc_outline_3"],"v_align":"center"}
{"id":"arc_outline_1","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":90,"stroke_color":"#e74c3c","stroke_width":4,"filled":false}
{"id":"arc_outline_2","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":180,"stroke_color":"#3498db","stroke_width":4,"filled":false}
{"id":"arc_outline_3","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":270,"stroke_color":"#2ecc71","stroke_width":4,"filled":false}
{"id":"filled_section","type":"vstack","children":["filled_label","filled_arcs"],"h_align":"center"}
{"id":"filled_label","type":"text","content":"Filled Sectors","font_size":16,"color":"#333"}
{"id":"filled_arcs","type":"hstack","children":["arc_filled_1","arc_filled_2","arc_filled_3"],"v_align":"center"}
{"id":"arc_filled_1","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":90,"stroke_color":"#c0392b","stroke_width":2,"fill_color":"#e74c3c","filled":true}
{"id":"arc_filled_2","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":180,"stroke_color":"#2980b9","stroke_width":2,"fill_color":"#3498db","filled":true}
{"id":"arc_filled_3","type":"arc","cx":40,"cy":40,"radius":25,"start_angle":0,"end_angle":270,"stroke_color":"#27ae60","stroke_width":2,"fill_color":"#2ecc71","filled":true}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("04_filled_arcs.svg"), "Filled vs Unfilled Arcs")
}

// Demo 5: Complex Arc Patterns
fn generate_complex_arc_patterns(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","patterns_container"],"h_align":"center"}
{"id":"title","type":"text","content":"🌟 Complex Arc Patterns","font_size":24,"color":"darkcyan"}
{"id":"patterns_container","type":"free_container","width":800,"height":500,"children":["flower_pattern","spiral_pattern","clock_pattern","legend"]}
{"id":"flower_pattern","type":"free_container","width":200,"height":200,"children":["petal1","petal2","petal3","petal4","petal5","petal6","flower_center"],"x":50,"y":50}
{"id":"petal1","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":0,"end_angle":60,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#fce4ec","filled":true}
{"id":"petal2","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":60,"end_angle":120,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#f8bbd9","filled":true}
{"id":"petal3","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":120,"end_angle":180,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#fce4ec","filled":true}
{"id":"petal4","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":180,"end_angle":240,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#f8bbd9","filled":true}
{"id":"petal5","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":240,"end_angle":300,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#fce4ec","filled":true}
{"id":"petal6","type":"arc","cx":100,"cy":100,"radius":40,"start_angle":300,"end_angle":360,"stroke_color":"#e91e63","stroke_width":4,"fill_color":"#f8bbd9","filled":true}
{"id":"flower_center","type":"ellipse","cx":10,"cy":10,"rx":10,"ry":10,"fill":"#ffeb3b","stroke":"#ff9800","stroke_width":2,"x":90,"y":90}
{"id":"spiral_pattern","type":"free_container","width":200,"height":200,"children":["spiral1","spiral2","spiral3","spiral4","spiral5"],"x":300,"y":50}
{"id":"spiral1","type":"arc","cx":100,"cy":100,"radius":20,"start_angle":0,"end_angle":270,"stroke_color":"#9c27b0","stroke_width":6,"filled":false}
{"id":"spiral2","type":"arc","cx":100,"cy":100,"radius":35,"start_angle":45,"end_angle":315,"stroke_color":"#673ab7","stroke_width":5,"filled":false}
{"id":"spiral3","type":"arc","cx":100,"cy":100,"radius":50,"start_angle":90,"end_angle":360,"stroke_color":"#3f51b5","stroke_width":4,"filled":false}
{"id":"spiral4","type":"arc","cx":100,"cy":100,"radius":65,"start_angle":135,"end_angle":405,"stroke_color":"#2196f3","stroke_width":3,"filled":false}
{"id":"spiral5","type":"arc","cx":100,"cy":100,"radius":80,"start_angle":180,"end_angle":450,"stroke_color":"#03a9f4","stroke_width":2,"filled":false}
{"id":"clock_pattern","type":"free_container","width":200,"height":200,"children":["clock_12","clock_3","clock_6","clock_9","clock_center","hour_hand","minute_hand"],"x":550,"y":50}
{"id":"clock_12","type":"quarter_circle","cx":100,"cy":100,"radius":80,"quadrant":1,"stroke_color":"#795548","stroke_width":3,"filled":false}
{"id":"clock_3","type":"quarter_circle","cx":100,"cy":100,"radius":80,"quadrant":4,"stroke_color":"#795548","stroke_width":3,"filled":false}
{"id":"clock_6","type":"quarter_circle","cx":100,"cy":100,"radius":80,"quadrant":3,"stroke_color":"#795548","stroke_width":3,"filled":false}
{"id":"clock_9","type":"quarter_circle","cx":100,"cy":100,"radius":80,"quadrant":2,"stroke_color":"#795548","stroke_width":3,"filled":false}
{"id":"clock_center","type":"ellipse","cx":8,"cy":8,"rx":8,"ry":8,"fill":"#424242","x":92,"y":92}
{"id":"hour_hand","type":"line","start_x":100,"start_y":100,"end_x":100,"end_y":60,"stroke_color":"#212121","stroke_width":4,"x":0,"y":0}
{"id":"minute_hand","type":"line","start_x":100,"start_y":100,"end_x":130,"end_y":100,"stroke_color":"#424242","stroke_width":2,"x":0,"y":0}
{"id":"legend","type":"vstack","children":["flower_desc","spiral_desc","clock_desc"],"h_align":"left","x":50,"y":300}
{"id":"flower_desc","type":"text","content":"🌸 Flower Pattern: 6 filled arc petals (60° each)","font_size":14,"color":"#e91e63"}
{"id":"spiral_desc","type":"text","content":"🌀 Spiral Pattern: Concentric arcs with increasing radius","font_size":14,"color":"#9c27b0"}
{"id":"clock_desc","type":"text","content":"🕒 Clock Pattern: Quarter circles forming clock face","font_size":14,"color":"#795548"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("05_complex_patterns.svg"), "Complex Arc Patterns")
}

// Demo 6: Arc Dashboard with Various Metrics
fn generate_arc_dashboard(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["header","dashboard_content"],"h_align":"center"}
{"id":"header","type":"text","content":"📊 Arc-Based Dashboard","font_size":26,"color":"#2c3e50"}
{"id":"dashboard_content","type":"free_container","width":900,"height":600,"children":["progress_gauges","pie_charts","status_indicators","dashboard_labels"]}
{"id":"progress_gauges","type":"free_container","width":300,"height":200,"children":["gauge1_bg","gauge1_fill","gauge1_text","gauge2_bg","gauge2_fill","gauge2_text","gauge3_bg","gauge3_fill","gauge3_text"],"x":50,"y":50}
{"id":"gauge1_bg","type":"arc","cx":100,"cy":100,"radius":60,"start_angle":180,"end_angle":360,"stroke_color":"#ecf0f1","stroke_width":12,"filled":false}
{"id":"gauge1_fill","type":"arc","cx":100,"cy":100,"radius":60,"start_angle":180,"end_angle":288,"stroke_color":"#e74c3c","stroke_width":12,"filled":false}
{"id":"gauge1_text","type":"text","content":"CPU 60%","font_size":12,"color":"#2c3e50","x":75,"y":120}
{"id":"gauge2_bg","type":"arc","cx":100,"cy":250,"radius":60,"start_angle":180,"end_angle":360,"stroke_color":"#ecf0f1","stroke_width":12,"filled":false}
{"id":"gauge2_fill","type":"arc","cx":100,"cy":250,"radius":60,"start_angle":180,"end_angle":324,"stroke_color":"#f39c12","stroke_width":12,"filled":false}
{"id":"gauge2_text","type":"text","content":"RAM 80%","font_size":12,"color":"#2c3e50","x":75,"y":270}
{"id":"gauge3_bg","type":"arc","cx":250,"cy":175,"radius":60,"start_angle":180,"end_angle":360,"stroke_color":"#ecf0f1","stroke_width":12,"filled":false}
{"id":"gauge3_fill","type":"arc","cx":250,"cy":175,"radius":60,"start_angle":180,"end_angle":252,"stroke_color":"#27ae60","stroke_width":12,"filled":false}
{"id":"gauge3_text","type":"text","content":"Disk 40%","font_size":12,"color":"#2c3e50","x":225,"y":195}
{"id":"pie_charts","type":"free_container","width":250,"height":250,"children":["pie1_s1","pie1_s2","pie1_s3","pie1_s4","pie2_s1","pie2_s2","pie2_s3"],"x":400,"y":50}
{"id":"pie1_s1","type":"arc","cx":80,"cy":80,"radius":50,"start_angle":0,"end_angle":144,"stroke_color":"#3498db","stroke_width":2,"fill_color":"#3498db","filled":true}
{"id":"pie1_s2","type":"arc","cx":80,"cy":80,"radius":50,"start_angle":144,"end_angle":216,"stroke_color":"#e74c3c","stroke_width":2,"fill_color":"#e74c3c","filled":true}
{"id":"pie1_s3","type":"arc","cx":80,"cy":80,"radius":50,"start_angle":216,"end_angle":288,"stroke_color":"#2ecc71","stroke_width":2,"fill_color":"#2ecc71","filled":true}
{"id":"pie1_s4","type":"arc","cx":80,"cy":80,"radius":50,"start_angle":288,"end_angle":360,"stroke_color":"#f39c12","stroke_width":2,"fill_color":"#f39c12","filled":true}
{"id":"pie2_s1","type":"arc","cx":80,"cy":200,"radius":40,"start_angle":0,"end_angle":180,"stroke_color":"#9b59b6","stroke_width":2,"fill_color":"#9b59b6","filled":true}
{"id":"pie2_s2","type":"arc","cx":80,"cy":200,"radius":40,"start_angle":180,"end_angle":270,"stroke_color":"#1abc9c","stroke_width":2,"fill_color":"#1abc9c","filled":true}
{"id":"pie2_s3","type":"arc","cx":80,"cy":200,"radius":40,"start_angle":270,"end_angle":360,"stroke_color":"#e67e22","stroke_width":2,"fill_color":"#e67e22","filled":true}
{"id":"status_indicators","type":"free_container","width":200,"height":300,"children":["status1","status2","status3","status4"],"x":700,"y":50}
{"id":"status1","type":"arc","cx":50,"cy":50,"radius":30,"start_angle":0,"end_angle":90,"stroke_color":"#27ae60","stroke_width":8,"filled":false}
{"id":"status2","type":"arc","cx":150,"cy":50,"radius":30,"start_angle":0,"end_angle":180,"stroke_color":"#f39c12","stroke_width":8,"filled":false}
{"id":"status3","type":"arc","cx":50,"cy":150,"radius":30,"start_angle":0,"end_angle":270,"stroke_color":"#e74c3c","stroke_width":8,"filled":false}
{"id":"status4","type":"arc","cx":150,"cy":150,"radius":30,"start_angle":0,"end_angle":360,"stroke_color":"#8e44ad","stroke_width":8,"filled":false}
{"id":"dashboard_labels","type":"vstack","children":["gauges_label","pie_label","status_label"],"h_align":"left","x":50,"y":400}
{"id":"gauges_label","type":"text","content":"📈 Progress Gauges: System resource usage","font_size":16,"color":"#34495e"}
{"id":"pie_label","type":"text","content":"🍰 Pie Charts: Data distribution visualization","font_size":16,"color":"#34495e"}
{"id":"status_label","type":"text","content":"🔄 Status Indicators: Service availability levels","font_size":16,"color":"#34495e"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("06_arc_dashboard.svg"), "Arc-Based Dashboard")
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