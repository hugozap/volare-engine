// src/bin/intricate_city.rs
// Demo program that generates an intricate city using only polylines, rects, and free containers

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;
use custom_components::document::register_document_components;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏙️ Generando sample doc");

    // Create output directory
    let output_dir = std::env::temp_dir().join("document_demo");
    std::fs::create_dir_all(&output_dir)?;

    let input = r##"
{"id":"doc_root","content_id":"main_section","footer_id":"footer","header_id":"header","type":"document"}
{"id":"header","text":"Failure Modes and Effects Analysis (FMEA) example for a centrifugal pump system — sample entries showing Severity (S), Occurrence (O), Detection (D), RPN and recommended actions.","title":"FMEA Matrix — Reliability Example","type":"document.text","variant":"large","width":"full"}
{"id":"main_section","columns":["left_panel","fmea_container"],"meta":"Pump System — Reliability Engineering","title":"FMEA Overview","type":"document.section"}
{"id":"left_panel","background":"#fbfbfb","children":["overview_text","key_points"],"padding":12,"type":"box"}
{"id":"overview_text","text":"This FMEA matrix covers a simplified set of failure modes for a centrifugal pump assembly used in process industries. The table includes severity (S), occurrence (O), detection (D), resulting RPN, and recommended mitigations.","title":"Scope","type":"document.text","variant":"default","width":"md"}
{"id":"key_points","items":["System: Centrifugal Pump Assembly","Focus: Reliability and maintenance prioritization","Metrics: Severity (1-10), Occurrence (1-10), Detection (1-10), RPN = S×O×D","Actions: Design, Maintenance, and Monitoring recommendations"],"meta":"Key Points","type":"document.bullet_list"}
{"id":"f_head_c1","content":"Item","font_size":12,"type":"text"}
{"id":"f_head_c2","content":"Function","font_size":12,"type":"text"}
{"id":"f_head_c3","content":"Failure Mode","font_size":12,"type":"text"}
{"id":"f_head_c4","content":"Effect","font_size":12,"type":"text"}
{"id":"f_head_c5","content":"S","font_size":12,"type":"text"}
{"id":"f_head_c6","content":"Cause","font_size":12,"type":"text"}
{"id":"f_head_c7","content":"O","font_size":12,"type":"text"}
{"id":"f_head_c8","content":"D","font_size":12,"type":"text"}
{"id":"f_head_c9","content":"RPN","font_size":12,"type":"text"}
{"id":"f_head_c10","content":"Recommended Action","font_size":12,"type":"text"}
{"id":"f_r1_c1","content":"1 - Pump Seal","font_size":11,"type":"text"}
{"id":"f_r1_c2","content":"Prevent leakage / contain fluid","font_size":11,"type":"text"}
{"id":"f_r1_c3","content":"Seal wear / degradation","font_size":11,"type":"text"}
{"id":"f_r1_c4","content":"Fluid leakage, potential system downtime","font_size":11,"type":"text"}
{"id":"f_r1_c5","content":"7","font_size":11,"type":"text"}
{"id":"f_r1_c6","content":"Abrasive particles, chemical attack","font_size":11,"type":"text"}
{"id":"f_r1_c7","content":"5","font_size":11,"type":"text"}
{"id":"f_r1_c8","content":"4","font_size":11,"type":"text"}
{"id":"f_r1_c9","content":"140","font_size":11,"type":"text"}
{"id":"f_r1_c10","content":"Specify harder seal material; implement seal inspection & replacement schedule","font_size":11,"type":"text"}
{"id":"f_r2_c1","content":"2 - Motor Bearings","font_size":11,"type":"text"}
{"id":"f_r2_c2","content":"Support shaft rotation, reduce friction","font_size":11,"type":"text"}
{"id":"f_r2_c3","content":"Bearing wear / spalling","font_size":11,"type":"text"}
{"id":"f_r2_c4","content":"Increased vibration, overheating, premature failure","font_size":11,"type":"text"}
{"id":"f_r2_c5","content":"6","font_size":11,"type":"text"}
{"id":"f_r2_c6","content":"Insufficient lubrication, contamination","font_size":11,"type":"text"}
{"id":"f_r2_c7","content":"4","font_size":11,"type":"text"}
{"id":"f_r2_c8","content":"3","font_size":11,"type":"text"}
{"id":"f_r2_c9","content":"72","font_size":11,"type":"text"}
{"id":"f_r2_c10","content":"Establish lubrication schedule; install contamination controls and vibration monitoring","font_size":11,"type":"text"}
{"id":"f_r3_c1","content":"3 - Shaft Coupling","font_size":11,"type":"text"}
{"id":"f_r3_c2","content":"Transmit torque between motor and pump","font_size":11,"type":"text"}
{"id":"f_r3_c3","content":"Misalignment / coupling wear","font_size":11,"type":"text"}
{"id":"f_r3_c4","content":"Excessive vibration, torque loss, potential shaft damage","font_size":11,"type":"text"}
{"id":"f_r3_c5","content":"7","font_size":11,"type":"text"}
{"id":"f_r3_c6","content":"Improper installation, thermal expansion","font_size":11,"type":"text"}
{"id":"f_r3_c7","content":"3","font_size":11,"type":"text"}
{"id":"f_r3_c8","content":"5","font_size":11,"type":"text"}
{"id":"f_r3_c9","content":"105","font_size":11,"type":"text"}
{"id":"f_r3_c10","content":"Perform alignment checks at installation and scheduled intervals; use torque monitoring","font_size":11,"type":"text"}
{"id":"fmea_table","border_color":"#cccccc","border_width":1,"cell_padding":8,"children":["f_head_c1","f_head_c2","f_head_c3","f_head_c4","f_head_c5","f_head_c6","f_head_c7","f_head_c8","f_head_c9","f_head_c10","f_r1_c1","f_r1_c2","f_r1_c3","f_r1_c4","f_r1_c5","f_r1_c6","f_r1_c7","f_r1_c8","f_r1_c9","f_r1_c10","f_r2_c1","f_r2_c2","f_r2_c3","f_r2_c4","f_r2_c5","f_r2_c6","f_r2_c7","f_r2_c8","f_r2_c9","f_r2_c10","f_r3_c1","f_r3_c2","f_r3_c3","f_r3_c4","f_r3_c5","f_r3_c6","f_r3_c7","f_r3_c8","f_r3_c9","f_r3_c10"],"cols":10,"fill_color":"#ffffff","type":"table"}
{"id":"actions_title","content":"Recommended Actions & Owners","font_size":12,"type":"text"}
{"id":"actions_list","items":["Seal material spec review — Engineering","Seal inspection & replacement — Maintenance","Lubrication schedule & contamination controls — Maintenance","Alignment checks & torque monitoring — Installation Team"],"meta":"Top Actions","type":"document.bullet_list"}
{"id":"action_box","background":"#f6f8ff","border_color":"#d6e0ff","border_width":1,"children":["actions_title","actions_list"],"padding":10,"type":"box","width":300}
{"id":"fmea_container","children":["fmea_table","action_box"],"constraints":[{"entities":["fmea_table","action_box"],"type":"align_top"},{"entities":["action_box","fmea_table"],"type":"right_of"},{"entities":["fmea_table","action_box"],"spacing":20.0,"type":"horizontal_spacing"},{"entities":["fmea_table","main_section"],"type":"align_left"}],"type":"constraint_container"}
{"id":"footer","text":"Sample FMEA matrix — values are illustrative. Use this template to expand system coverage and track actions, owners and dates.","type":"document.text","variant":"subtle","width":"full"}
    "##;

 // Parse the JSON Lines
    let mut parser = parser::JsonLinesParser::new();
    let root_id = parser.parse_string(input)?;

    // Create a fresh builder for parsing
    let mut parse_builder = DiagramBuilder::new();
    parse_builder.set_measure_text_fn(measure_text_svg_character_advance);
    register_document_components(&mut parse_builder);

    // Build the diagram
    let diagram = parser.build(&root_id, &mut parse_builder)?;

    // Calculate layout
    layout::layout_tree_node(&mut parse_builder, &diagram);

    // Render to SVG
    let output_path = output_dir.join("demo.svg");
    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&output_path)?;
    svg_renderer.render(&parse_builder, &diagram, &mut svg_file)?;



    println!("\n✅ document demo created");
    println!("📁 Archivo guardado en: {}", output_dir.display());

    Ok(())
}
