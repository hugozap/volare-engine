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
{"id":"root","type":"document","header_id":"header","content_id":"content","footer_id":"footer"}
{"id":"header","type":"document.text","text":"Cause & Effect (Ishikawa) Diagram — Example","variant":"large","width":"full"}
{"id":"content","type":"document.section","title":"Fishbone Diagram — Constraint Layout Example","meta":"Example data illustrating primary causes and sub-causes","columns":["diagram_container"]}
{"id":"footer","type":"document.text","text":"Generated example fishbone diagram — problem, primary causes, and sub-causes.","variant":"small","width":"full"}
{"id":"spine","type":"rect","width":700,"height":6,"fill":"#333","border_radius":3,"x":100,"y":297}
{"id":"problem_box","type":"rect","width":200,"height":80,"fill":"#ffdddd","border_color":"#cc6666","border_width":2,"border_radius":8,"x":820,"y":257}
{"id":"problem_text","type":"text","content":"Late Deliveries","font_size":16,"color":"#800000","line_width":180,"x":920,"y":297,"text_anchor":"middle"}
{"id":"bone_u1_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":-35,"x":250,"y":210,"transform_origin":"right center"}
{"id":"bone_u1_label","type":"text","content":"People","font_size":13,"color":"#333","font_weight":"bold","x":120,"y":165}
{"id":"bone_u1_c1","type":"text","content":"Staff shortages","font_size":10,"color":"#444","x":70,"y":185}
{"id":"bone_u1_c2","type":"text","content":"Insufficient training","font_size":10,"color":"#444","x":70,"y":198}
{"id":"bone_u2_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":-23,"x":320,"y":245,"transform_origin":"right center"}
{"id":"bone_u2_label","type":"text","content":"Process","font_size":13,"color":"#333","font_weight":"bold","x":180,"y":215}
{"id":"bone_u2_c1","type":"text","content":"Complex approvals","font_size":10,"color":"#444","x":120,"y":232}
{"id":"bone_u2_c2","type":"text","content":"Unclear workflows","font_size":10,"color":"#444","x":120,"y":245}
{"id":"bone_u3_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":-15,"x":400,"y":270,"transform_origin":"right center"}
{"id":"bone_u3_label","type":"text","content":"Equipment","font_size":13,"color":"#333","font_weight":"bold","x":260,"y":250}
{"id":"bone_u3_c1","type":"text","content":"Tool breakdowns","font_size":10,"color":"#444","x":200,"y":265}
{"id":"bone_u3_c2","type":"text","content":"Maintenance delays","font_size":10,"color":"#444","x":200,"y":278}
{"id":"bone_u4_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":-8,"x":540,"y":288,"transform_origin":"right center"}
{"id":"bone_u4_label","type":"text","content":"Materials","font_size":13,"color":"#333","font_weight":"bold","x":400,"y":275}
{"id":"bone_u4_c1","type":"text","content":"Supplier delays","font_size":10,"color":"#444","x":350,"y":288}
{"id":"bone_u4_c2","type":"text","content":"Quality issues","font_size":10,"color":"#444","x":350,"y":301}
{"id":"bone_l1_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":35,"x":250,"y":384,"transform_origin":"right center"}
{"id":"bone_l1_label","type":"text","content":"Measurements","font_size":13,"color":"#333","font_weight":"bold","x":100,"y":415}
{"id":"bone_l1_c1","type":"text","content":"Poor KPIs","font_size":10,"color":"#444","x":70,"y":395}
{"id":"bone_l1_c2","type":"text","content":"Infrequent monitoring","font_size":10,"color":"#444","x":70,"y":408}
{"id":"bone_l2_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":23,"x":320,"y":349,"transform_origin":"right center"}
{"id":"bone_l2_label","type":"text","content":"Environment","font_size":13,"color":"#333","font_weight":"bold","x":160,"y":365}
{"id":"bone_l2_c1","type":"text","content":"Site constraints","font_size":10,"color":"#444","x":120,"y":348}
{"id":"bone_l2_c2","type":"text","content":"Logistics bottlenecks","font_size":10,"color":"#444","x":120,"y":361}
{"id":"bone_l3_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":15,"x":400,"y":324,"transform_origin":"right center"}
{"id":"bone_l3_label","type":"text","content":"Methods","font_size":13,"color":"#333","font_weight":"bold","x":260,"y":335}
{"id":"bone_l3_c1","type":"text","content":"Manual handoffs","font_size":10,"color":"#444","x":200,"y":318}
{"id":"bone_l3_c2","type":"text","content":"Lack of standard work","font_size":10,"color":"#444","x":200,"y":331}
{"id":"bone_l4_line","type":"rect","width":160,"height":5,"fill":"#666","border_radius":2,"rotation":8,"x":540,"y":306,"transform_origin":"right center"}
{"id":"bone_l4_label","type":"text","content":"Management","font_size":13,"color":"#333","font_weight":"bold","x":380,"y":315}
{"id":"bone_l4_c1","type":"text","content":"Unrealistic deadlines","font_size":10,"color":"#444","x":350,"y":295}
{"id":"bone_l4_c2","type":"text","content":"Poor communication","font_size":10,"color":"#444","x":350,"y":308}
{"id":"diagram_container","type":"vstack","children":["spine","problem_box","problem_text","bone_u1_line","bone_u1_label","bone_u1_c1","bone_u1_c2","bone_u2_line","bone_u2_label","bone_u2_c1","bone_u2_c2","bone_u3_line","bone_u3_label","bone_u3_c1","bone_u3_c2","bone_u4_line","bone_u4_label","bone_u4_c1","bone_u4_c2","bone_l1_line","bone_l1_label","bone_l1_c1","bone_l1_c2","bone_l2_line","bone_l2_label","bone_l2_c1","bone_l2_c2","bone_l3_line","bone_l3_label","bone_l3_c1","bone_l3_c2","bone_l4_line","bone_l4_label","bone_l4_c1","bone_l4_c2"]} "##;

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
