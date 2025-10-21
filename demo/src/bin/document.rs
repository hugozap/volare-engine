// src/bin/intricate_city.rs
// Demo program that generates an intricate city using only polylines, rects, and free containers

use custom_components::register_all_components;
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

//     let input = r##"
// {"id":"test","type":"ishikawa","problem":"problem goes here"}"##;

//     let input = r##"{"id":"root","type":"hstack","children":["box1","box2","box3","conn1","conn2"],"spacing":50}
// {"id":"box1","type":"rect","width":80,"height":60,"fill":"lightblue","stroke":"blue","stroke_width":2}
// {"id":"box2","type":"rect","width":80,"height":60,"fill":"lightgreen","stroke":"green","stroke_width":2}
// {"id":"box3","type":"rect","width":80,"height":60,"fill":"lightyellow","stroke":"orange","stroke_width":2}
// {"id":"conn1","type":"connector","source":"box1","source_port":"right","target":"box2","target_port":"left","stroke_color":"red","stroke_width":2}
// {"id":"conn2","type":"connector","source":"box2","source_port":"right","target":"box3","target_port":"left","stroke_color":"green","stroke_width":2}
// "##;

let input = r##"{"id":"root","type":"activity_diagram","swimlanes":[{"name":"Lane 1","activities":[{"id":"start","label":"Start","type":"start"},{"id":"decision","label":"Check","type":"decision"}]},{"name":"Lane 2","activities":[{"id":"actA","label":"Option A","type":"normal"},{"id":"merge","label":"Merge","type":"merge"}]},{"name":"Lane 3","activities":[{"id":"actB","label":"Option B","type":"normal"}]}],"flows":[{"from":"start","to":"decision"},{"from":"decision","to":"actA","condition":"Yes"},{"from":"decision","to":"actB","condition":"No"},{"from":"actA","to":"merge"},{"from":"actB","to":"merge"}]}"##;


 // Parse the JSON Lines
    let mut parser = parser::JsonLinesParser::new();
    let root_id = parser.parse_string(input)?;

    // Create a fresh builder for parsing
    let mut parse_builder = DiagramBuilder::new();
    parse_builder.set_measure_text_fn(measure_text_svg_character_advance);
    register_all_components(&mut parse_builder);

    // Build the diagram
    let diagram = parser.build(&root_id, &mut parse_builder)?;

    // Calculate layout
    layout::layout_diagram(&mut parse_builder, &diagram);

    // Render to SVG
    let output_path = output_dir.join("demo.svg");
    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&output_path)?;
    svg_renderer.render(&parse_builder, &diagram, &mut svg_file)?;



    println!("\n✅ document demo created");
    println!("📁 Archivo guardado en: {}", output_dir.display());

    Ok(())
}
