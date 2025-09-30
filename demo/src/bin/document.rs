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
{"id":"doc","type":"document","header_id":"header","content_id":"content","footer_id":"footer"}
{"id":"header","type":"document.title","text":"Partes del Motor de una Motocicleta","variant":"h1","width":"800"}
{"id":"content","type":"table","cols":2,"children":["cilindro_cell"],"border_color":"#333","border_width":1,"cell_padding":15}
{"id":"cilindro_cell","type":"vstack","children":["cilindro_title","cilindro_props"],"h_align":"left"}
{"id":"cilindro_title","type":"document.title","text":"Cilindro","variant":"h3","width":"350"}
{"id":"cilindro_props","type":"document.properties","properties":[["Función","Contiene el pistón y la combustión"],["Material","Aluminio o hierro fundido"],["Tipo","Monocilíndrico o multicilíndrico"],["Refrigeración","Aire o líquido"]]}
{"id":"footer","type":"document.text","text":"Cada componente del motor trabaja en sincronía para convertir la energía química del combustible en movimiento mecánico.","variant":"subtle","width":"800"}
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
