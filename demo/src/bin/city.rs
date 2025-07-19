// src/bin/intricate_city.rs
// Demo program that generates an intricate city using only polylines, rects, and free containers

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏙️ Generando Ciudad Intrincada...\n");

    // Create output directory
    let output_dir = std::env::temp_dir().join("intricate_city");
    std::fs::create_dir_all(&output_dir)?;

    // Generate the intricate city
    generate_intricate_city(&output_dir)?;

    println!("\n✅ Ciudad generada exitosamente!");
    println!("📁 Archivo guardado en: {}", output_dir.display());

    Ok(())
}

fn generate_intricate_city(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":1400,"height":900,"background":"#E6F3FF","children":["metro_grid","financial_district","tech_campus","cultural_quarter","harbor","mountain_range","sky_bridges","monorail","shopping_centers"]}

{"id":"metro_grid","type":"free_container","width":1400,"height":900,"children":["metro_line_1","metro_line_2","metro_line_3","metro_line_4","station_markers"],"x":0,"y":0}
{"id":"metro_line_1","type":"polyline","points":[[0,200],[300,205],[600,200],[900,205],[1200,200],[1400,205]],"stroke_color":"#FF6B6B","stroke_width":6,"x":0,"y":0}
{"id":"metro_line_2","type":"polyline","points":[[0,450],[350,455],[700,450],[1050,455],[1400,450]],"stroke_color":"#4ECDC4","stroke_width":6,"x":0,"y":0}
{"id":"metro_line_3","type":"polyline","points":[[200,0],[205,200],[200,400],[205,600],[200,800],[205,900]],"stroke_color":"#45B7D1","stroke_width":6,"x":0,"y":0}
{"id":"metro_line_4","type":"polyline","points":[[800,0],[805,200],[800,400],[805,600],[800,800],[805,900]],"stroke_color":"#96CEB4","stroke_width":6,"x":0,"y":0}
{"id":"station_markers","type":"hstack","children":["station_1","station_2","station_3","station_4","station_5"],"v_align":"center","x":180,"y":180}
{"id":"station_1","type":"rect","width":12,"height":12,"background":"white","border_color":"black"}
{"id":"station_2","type":"rect","width":12,"height":12,"background":"white","border_color":"black"}
{"id":"station_3","type":"rect","width":12,"height":12,"background":"white","border_color":"black"}
{"id":"station_4","type":"rect","width":12,"height":12,"background":"white","border_color":"black"}
{"id":"station_5","type":"rect","width":12,"height":12,"background":"white","border_color":"black"}

{"id":"financial_district","type":"vstack","children":["fd_tier_1","fd_tier_2","fd_tier_3","fd_plaza"],"h_align":"center","x":50,"y":50}
{"id":"fd_tier_1","type":"hstack","children":["tower_mega","tower_alpha","tower_beta"],"v_align":"bottom"}
{"id":"tower_mega","type":"rect","width":45,"height":180,"background":"#2C3E50"}
{"id":"tower_alpha","type":"rect","width":40,"height":160,"background":"#34495E"}
{"id":"tower_beta","type":"rect","width":35,"height":140,"background":"#7F8C8D"}
{"id":"fd_tier_2","type":"hstack","children":["bank_central","office_prime","office_gold"],"v_align":"bottom"}
{"id":"bank_central","type":"rect","width":50,"height":80,"background":"#F39C12"}
{"id":"office_prime","type":"rect","width":30,"height":100,"background":"#E74C3C"}
{"id":"office_gold","type":"rect","width":35,"height":90,"background":"#9B59B6"}
{"id":"fd_tier_3","type":"hstack","children":["retail_1","retail_2","retail_3","retail_4"],"v_align":"center"}
{"id":"retail_1","type":"rect","width":25,"height":25,"background":"#E67E22"}
{"id":"retail_2","type":"rect","width":25,"height":25,"background":"#E74C3C"}
{"id":"retail_3","type":"rect","width":25,"height":25,"background":"#9B59B6"}
{"id":"retail_4","type":"rect","width":25,"height":25,"background":"#3498DB"}
{"id":"fd_plaza","type":"rect","width":200,"height":15,"background":"#BDC3C7"}

{"id":"tech_campus","type":"free_container","width":300,"height":250,"children":["campus_layout","innovation_labs","data_centers"],"x":500,"y":100}
{"id":"campus_layout","type":"vstack","children":["tech_buildings","tech_courtyard","parking_structure"],"h_align":"center","x":0,"y":0}
{"id":"tech_buildings","type":"hstack","children":["lab_a","lab_b","lab_c","lab_d"],"v_align":"bottom"}
{"id":"lab_a","type":"rect","width":40,"height":80,"background":"#1ABC9C"}
{"id":"lab_b","type":"rect","width":35,"height":70,"background":"#16A085"}
{"id":"lab_c","type":"rect","width":45,"height":85,"background":"#3498DB"}
{"id":"lab_d","type":"rect","width":38,"height":75,"background":"#2980B9"}
{"id":"tech_courtyard","type":"rect","width":180,"height":40,"background":"#2ECC71"}
{"id":"parking_structure","type":"hstack","children":["parking_level_1","parking_level_2","parking_level_3"],"v_align":"center"}
{"id":"parking_level_1","type":"rect","width":50,"height":15,"background":"#95A5A6"}
{"id":"parking_level_2","type":"rect","width":50,"height":15,"background":"#7F8C8D"}
{"id":"parking_level_3","type":"rect","width":50,"height":15,"background":"#95A5A6"}
{"id":"innovation_labs","type":"polyline","points":[[0,0],[50,20],[100,10],[150,30],[200,15],[250,25],[300,20]],"stroke_color":"#E74C3C","stroke_width":3,"x":0,"y":50}
{"id":"data_centers","type":"vstack","children":["server_1","server_2"],"h_align":"right","x":220,"y":180}
{"id":"server_1","type":"rect","width":60,"height":30,"background":"#34495E"}
{"id":"server_2","type":"rect","width":60,"height":25,"background":"#2C3E50"}

{"id":"cultural_quarter","type":"free_container","width":280,"height":200,"children":["museums","theaters","galleries"],"x":900,"y":250}
{"id":"museums","type":"vstack","children":["museum_art","museum_history","museum_science"],"h_align":"left","x":0,"y":0}
{"id":"museum_art","type":"hstack","children":["art_wing_1","art_wing_2","art_central"],"v_align":"center"}
{"id":"art_wing_1","type":"rect","width":30,"height":40,"background":"#8E44AD"}
{"id":"art_wing_2","type":"rect","width":30,"height":40,"background":"#9B59B6"}
{"id":"art_central","type":"rect","width":50,"height":60,"background":"#663399"}
{"id":"museum_history","type":"rect","width":120,"height":35,"background":"#D35400"}
{"id":"museum_science","type":"hstack","children":["planetarium","exhibits","imax"],"v_align":"center"}
{"id":"planetarium","type":"rect","width":25,"height":25,"background":"#2C3E50"}
{"id":"exhibits","type":"rect","width":60,"height":30,"background":"#34495E"}
{"id":"imax","type":"rect","width":35,"height":28,"background":"#2C3E50"}
{"id":"theaters","type":"hstack","children":["opera_house","concert_hall"],"v_align":"bottom","x":140,"y":20}
{"id":"opera_house","type":"vstack","children":["opera_dome","opera_base"],"h_align":"center"}
{"id":"opera_dome","type":"rect","width":40,"height":20,"background":"#E67E22"}
{"id":"opera_base","type":"rect","width":60,"height":40,"background":"#D35400"}
{"id":"concert_hall","type":"rect","width":45,"height":70,"background":"#F39C12"}
{"id":"galleries","type":"polyline","points":[[0,150],[40,160],[80,150],[120,160],[160,150],[200,160],[240,150],[280,160]],"stroke_color":"#9B59B6","stroke_width":4,"x":0,"y":0}

{"id":"harbor","type":"free_container","width":400,"height":300,"children":["waterfront","docks","marina","lighthouse"],"x":1000,"y":600}
{"id":"waterfront","type":"rect","width":400,"height":300,"background":"#3498DB","x":0,"y":0}
{"id":"docks","type":"vstack","children":["pier_1","pier_2","pier_3"],"h_align":"left","x":20,"y":50}
{"id":"pier_1","type":"hstack","children":["dock_a","dock_b","dock_c"],"v_align":"center"}
{"id":"dock_a","type":"rect","width":60,"height":8,"background":"#8B4513"}
{"id":"dock_b","type":"rect","width":60,"height":8,"background":"#A0522D"}
{"id":"dock_c","type":"rect","width":60,"height":8,"background":"#8B4513"}
{"id":"pier_2","type":"rect","width":200,"height":10,"background":"#654321"}
{"id":"pier_3","type":"hstack","children":["slip_1","slip_2","slip_3","slip_4"],"v_align":"center"}
{"id":"slip_1","type":"rect","width":40,"height":6,"background":"#8B4513"}
{"id":"slip_2","type":"rect","width":40,"height":6,"background":"#A0522D"}
{"id":"slip_3","type":"rect","width":40,"height":6,"background":"#8B4513"}
{"id":"slip_4","type":"rect","width":40,"height":6,"background":"#A0522D"}
{"id":"marina","type":"polyline","points":[[50,200],[80,220],[120,210],[160,230],[200,220],[240,240],[280,230]],"stroke_color":"white","stroke_width":2,"x":0,"y":0}
{"id":"lighthouse","type":"vstack","children":["lighthouse_base","lighthouse_tower","lighthouse_light"],"h_align":"center","x":350,"y":180}
{"id":"lighthouse_base","type":"rect","width":20,"height":30,"background":"#E74C3C"}
{"id":"lighthouse_tower","type":"rect","width":12,"height":60,"background":"white"}
{"id":"lighthouse_light","type":"rect","width":16,"height":8,"background":"#F1C40F"}

{"id":"mountain_range","type":"polyline","points":[[0,0],[100,50],[200,20],[300,80],[400,30],[500,90],[600,40],[700,100],[800,50],[900,110],[1000,60],[1100,120],[1200,70],[1300,130],[1400,80]],"stroke_color":"#7D6E3E","stroke_width":30,"x":0,"y":0}

{"id":"sky_bridges","type":"free_container","width":1400,"height":900,"children":["bridge_network","aerial_walkways"],"x":0,"y":0}
{"id":"bridge_network","type":"hstack","children":["sky_bridge_1","sky_bridge_2","sky_bridge_3"],"v_align":"center","x":100,"y":150}
{"id":"sky_bridge_1","type":"polyline","points":[[0,0],[150,10],[300,0]],"stroke_color":"#BDC3C7","stroke_width":4,"x":0,"y":0}
{"id":"sky_bridge_2","type":"polyline","points":[[0,0],[200,15],[400,0]],"stroke_color":"#95A5A6","stroke_width":4,"x":0,"y":0}
{"id":"sky_bridge_3","type":"polyline","points":[[0,0],[180,8],[360,0]],"stroke_color":"#BDC3C7","stroke_width":4,"x":0,"y":0}
{"id":"aerial_walkways","type":"vstack","children":["walkway_level_1","walkway_level_2"],"h_align":"center","x":600,"y":100}
{"id":"walkway_level_1","type":"polyline","points":[[0,0],[100,5],[200,0],[300,5],[400,0]],"stroke_color":"#ECF0F1","stroke_width":3,"x":0,"y":0}
{"id":"walkway_level_2","type":"polyline","points":[[50,0],[150,8],[250,0],[350,8],[450,0]],"stroke_color":"#BDC3C7","stroke_width":3,"x":0,"y":0}

{"id":"monorail","type":"polyline","points":[[0,350],[200,355],[400,350],[600,355],[800,350],[1000,355],[1200,350],[1400,355]],"stroke_color":"#2C3E50","stroke_width":8,"x":0,"y":0}

{"id":"shopping_centers","type":"free_container","width":600,"height":400,"children":["mall_complex","outdoor_plaza","market_district"],"x":250,"y":500}
{"id":"mall_complex","type":"vstack","children":["mall_upper","mall_main","mall_food_court"],"h_align":"center","x":0,"y":0}
{"id":"mall_upper","type":"hstack","children":["store_1","store_2","store_3","store_4","store_5"],"v_align":"center"}
{"id":"store_1","type":"rect","width":40,"height":30,"background":"#E91E63"}
{"id":"store_2","type":"rect","width":35,"height":30,"background":"#9C27B0"}
{"id":"store_3","type":"rect","width":45,"height":30,"background":"#673AB7"}
{"id":"store_4","type":"rect","width":38,"height":30,"background":"#3F51B5"}
{"id":"store_5","type":"rect","width":42,"height":30,"background":"#2196F3"}
{"id":"mall_main","type":"rect","width":250,"height":40,"background":"#F5F5F5"}
{"id":"mall_food_court","type":"hstack","children":["restaurant_1","restaurant_2","restaurant_3","seating"],"v_align":"center"}
{"id":"restaurant_1","type":"rect","width":50,"height":25,"background":"#FF5722"}
{"id":"restaurant_2","type":"rect","width":45,"height":25,"background":"#FF9800"}
{"id":"restaurant_3","type":"rect","width":40,"height":25,"background":"#FFC107"}
{"id":"seating","type":"rect","width":80,"height":25,"background":"#FFEB3B"}
{"id":"outdoor_plaza","type":"free_container","width":200,"height":150,"children":["plaza_fountains","plaza_vendors"],"x":300,"y":50}
{"id":"plaza_fountains","type":"vstack","children":["fountain_main","fountain_small_1","fountain_small_2"],"h_align":"center","x":50,"y":20}
{"id":"fountain_main","type":"rect","width":30,"height":30,"background":"#00BCD4"}
{"id":"fountain_small_1","type":"rect","width":15,"height":15,"background":"#26C6DA"}
{"id":"fountain_small_2","type":"rect","width":15,"height":15,"background":"#26C6DA"}
{"id":"plaza_vendors","type":"hstack","children":["vendor_1","vendor_2","vendor_3"],"v_align":"center","x":20,"y":100}
{"id":"vendor_1","type":"rect","width":25,"height":15,"background":"#4CAF50"}
{"id":"vendor_2","type":"rect","width":25,"height":15,"background":"#8BC34A"}
{"id":"vendor_3","type":"rect","width":25,"height":15,"background":"#CDDC39"}
{"id":"market_district","type":"polyline","points":[[400,200],[450,220],[500,200],[550,220],[600,200]],"stroke_color":"#795548","stroke_width":6,"x":0,"y":0}"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("intricate_city.svg"), "Ciudad Intrincada")
}

// Helper function to generate SVG from JSONL
fn generate_svg_from_jsonl(
    jsonl_input: &str,
    output_path: std::path::PathBuf,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📄 Generando: {}", description);

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

    println!("    ✅ Guardado: {}", output_path.file_name().unwrap().to_str().unwrap());

    Ok(())
}
