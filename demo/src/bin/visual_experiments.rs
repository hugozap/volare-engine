// src/bin/visual_experiments.rs
// Demo program that generates SVGs for visual experiments using JSONL format

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Generando Experimentos Visuales...\n");

    // Create output directory
    let output_dir = std::env::temp_dir().join("visual_experiments");
    std::fs::create_dir_all(&output_dir)?;

    // 🎯 Experimentos de Animación/Progresión
    println!("=== 🎯 Experimentos de Animación/Progresión ===");
    
    // 1. Construcción de Edificio
    generate_building_construction(&output_dir)?;
    
    // 2. Crecimiento de Árbol
    generate_tree_growth(&output_dir)?;

    // 📊 Visualización de Datos en Tiempo Real
    println!("\n=== 📊 Visualización de Datos ===");
    
    // 3. Dashboard Financiero
    generate_financial_dashboard(&output_dir)?;
    
    // 4. Monitor de Sistema
    generate_system_monitor(&output_dir)?;

    // 🎮 Interfaces Interactivas
    println!("\n=== 🎮 Interfaces Interactivas ===");
    
    // 5. Panel de Control Espacial
    generate_space_control_panel(&output_dir)?;
    
    // 6. Simulador de Tráfico
    generate_traffic_simulator(&output_dir)?;

    // 🧬 Simulaciones Científicas
    println!("\n=== 🧬 Simulaciones Científicas ===");
    
    // 7. Modelo Molecular
    generate_molecular_model(&output_dir)?;
    
    // 8. Sistema Solar
    generate_solar_system(&output_dir)?;

    // 🎨 Arte Generativo
    println!("\n=== 🎨 Arte Generativo ===");
    
    // 9. Patrón Fractal Simple
    generate_fractal_pattern(&output_dir)?;
    
    // 10. Mandala Geométrico
    generate_geometric_mandala(&output_dir)?;

    println!("\n✅ Todos los experimentos generados exitosamente!");
    println!("📁 Archivos guardados en: {}", output_dir.display());

    Ok(())
}

// 1. Construcción de Edificio
fn generate_building_construction(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":400,"height":400,"children":["ground","floor1","floor2","floor3","roof","window1","window2"]}
{"id":"ground","type":"rect","width":400,"height":20,"background":"brown","x":0,"y":380}
{"id":"floor1","type":"rect","width":200,"height":60,"background":"lightgray","x":100,"y":320}
{"id":"floor2","type":"rect","width":200,"height":60,"background":"lightgray","x":100,"y":260}
{"id":"floor3","type":"rect","width":200,"height":60,"background":"lightgray","x":100,"y":200}
{"id":"roof","type":"polyline","points":[[100,200],[150,150],[250,150],[300,200]],"stroke_color":"red","x":0,"y":0}
{"id":"window1","type":"rect","width":20,"height":30,"background":"lightblue","x":120,"y":340}
{"id":"window2","type":"rect","width":20,"height":30,"background":"lightblue","x":160,"y":340}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("01_building_construction.svg"), "Construcción de Edificio")
}

// 2. Crecimiento de Árbol

fn generate_tree_growth(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
   // 2. Crecimiento de Árbol
 let jsonl_input = r##"
{"id":"root","type":"free_container","width":400,"height":300,"children":["trunk","branch1","branch2","leaf1","leaf2","apple1"]}
{"id":"trunk","type":"rect","width":20,"height":100,"background":"brown","x":190,"y":150}
{"id":"branch1","type":"line","start_x":200,"start_y":150,"end_x":170,"end_y":120,"stroke_color":"brown","stroke_width":3,"x":0,"y":0}
{"id":"branch2","type":"line","start_x":200,"start_y":150,"end_x":230,"end_y":120,"stroke_color":"brown","stroke_width":3,"x":0,"y":0}
{"id":"leaf1","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"green","x":155,"y":105}
{"id":"leaf2","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"green","x":215,"y":105}
{"id":"apple1","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"red","x":160,"y":125}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("02_tree_growth.svg"), "Crecimiento de Árbol")
}

// 3. Dashboard Financiero
fn generate_financial_dashboard(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["header","metrics","chart_area"],"h_align":"center"}
{"id":"header","type":"text","content":"📈 Financial Dashboard","font_size":24,"color":"darkblue"}
{"id":"metrics","type":"hstack","children":["revenue","profit","users"],"v_align":"center"}
{"id":"revenue","type":"vstack","children":["rev_icon","rev_value","rev_change"],"h_align":"center"}
{"id":"rev_icon","type":"text","content":"💰","font_size":20}
{"id":"rev_value","type":"text","content":"$125,430","font_size":18,"color":"#333"}
{"id":"rev_change","type":"text","content":"↗ +12.5%","font_size":12,"color":"green"}
{"id":"profit","type":"vstack","children":["prof_icon","prof_value","prof_change"],"h_align":"center"}
{"id":"prof_icon","type":"text","content":"📊","font_size":20}
{"id":"prof_value","type":"text","content":"$45,200","font_size":18,"color":"#333"}
{"id":"prof_change","type":"text","content":"↗ +8.2%","font_size":12,"color":"green"}
{"id":"users","type":"vstack","children":["user_icon","user_value","user_change"],"h_align":"center"}
{"id":"user_icon","type":"text","content":"👥","font_size":20}
{"id":"user_value","type":"text","content":"8,945","font_size":18,"color":"#333"}
{"id":"user_change","type":"text","content":"↘ -2.1%","font_size":12,"color":"red"}
{"id":"chart_area","type":"free_container","width":250,"height":120,"children":["bar1","bar2","bar3"]}
{"id":"bar1","type":"rect","width":30,"height":50,"background":"blue","x":50,"y":50}
{"id":"bar2","type":"rect","width":30,"height":75,"background":"green","x":100,"y":25}
{"id":"bar3","type":"rect","width":30,"height":40,"background":"red","x":150,"y":60}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("03_financial_dashboard.svg"), "Dashboard Financiero")
}

// 4. Monitor de Sistema
fn generate_system_monitor(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","cpu_section","memory_section","network_section"],"h_align":"left"}
{"id":"title","type":"text","content":"🖥️ System Monitor","font_size":20,"color":"#333"}
{"id":"cpu_section","type":"hstack","children":["cpu_label","cpu_container"],"v_align":"center"}
{"id":"cpu_label","type":"text","content":"CPU:","font_size":14}
{"id":"cpu_container","type":"free_container","width":210,"height":25,"children":["cpu_bg","cpu_fill","cpu_text"]}
{"id":"cpu_bg","type":"rect","width":200,"height":20,"background":"lightgray","x":5,"y":2}
{"id":"cpu_fill","type":"rect","width":150,"height":20,"background":"orange","x":5,"y":2}
{"id":"cpu_text","type":"text","content":"75%","font_size":12,"x":105,"y":12}
{"id":"memory_section","type":"hstack","children":["mem_label","mem_container"],"v_align":"center"}
{"id":"mem_label","type":"text","content":"Memory:","font_size":14}
{"id":"mem_container","type":"free_container","width":210,"height":25,"children":["mem_bg","mem_fill","mem_text"]}
{"id":"mem_bg","type":"rect","width":200,"height":20,"background":"lightgray","x":5,"y":2}
{"id":"mem_fill","type":"rect","width":90,"height":20,"background":"blue","x":5,"y":2}
{"id":"mem_text","type":"text","content":"45%","font_size":12,"x":105,"y":12}
{"id":"network_section","type":"hstack","children":["net_label","net_indicator"],"v_align":"center"}
{"id":"net_label","type":"text","content":"Network:","font_size":14}
{"id":"net_indicator","type":"ellipse","cx":8,"cy":8,"rx":8,"ry":8,"fill":"green"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("04_system_monitor.svg"), "Monitor de Sistema")
}

// 5. Panel de Control Espacial
fn generate_space_control_panel(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":400,"height":250,"background":"black","border_color":"cyan","border_width":2,"children":["radar","blip1","blip2","status_panel"]}
{"id":"radar","type":"ellipse","cx":80,"cy":80,"rx":80,"ry":80,"fill":"darkgreen","stroke":"green","x":50,"y":50}
{"id":"blip1","type":"ellipse","cx":3,"cy":3,"rx":3,"ry":3,"fill":"red","x":120,"y":140}
{"id":"blip2","type":"ellipse","cx":3,"cy":3,"rx":3,"ry":3,"fill":"yellow","x":170,"y":160}
{"id":"status_panel","type":"vstack","children":["shields","energy","weapons"],"h_align":"left","x":250,"y":50}
{"id":"shields","type":"text","content":"🛡️ Shields: 85%","color":"cyan","font_size":12}
{"id":"energy","type":"text","content":"⚡ Energy: 92%","color":"yellow","font_size":12}
{"id":"weapons","type":"text","content":"🔫 Weapons: Online","color":"green","font_size":12}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("05_space_control_panel.svg"), "Panel de Control Espacial")
}

// 6. Simulador de Tráfico
fn generate_traffic_simulator(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":400,"height":300,"background":"lightgray","children":["road_h","road_v","car1","car2","car3","traffic_light"]}
{"id":"road_h","type":"rect","width":400,"height":40,"background":"darkgray","x":0,"y":130}
{"id":"road_v","type":"rect","width":40,"height":300,"background":"darkgray","x":180,"y":0}
{"id":"car1","type":"rect","width":30,"height":15,"background":"red","x":50,"y":140}
{"id":"car2","type":"rect","width":30,"height":15,"background":"blue","x":320,"y":140}
{"id":"car3","type":"rect","width":15,"height":30,"background":"green","x":185,"y":50}
{"id":"traffic_light","type":"vstack","children":["red_light","yellow_light","green_light"],"h_align":"center","x":175,"y":110}
{"id":"red_light","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"red"}
{"id":"yellow_light","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"gray"}
{"id":"green_light","type":"ellipse","cx":5,"cy":5,"rx":5,"ry":5,"fill":"gray"}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("06_traffic_simulator.svg"), "Simulador de Tráfico")
}

// 7. Modelo Molecular
fn generate_molecular_model(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":300,"height":300,"children":["carbon1","hydrogen1","hydrogen2","oxygen1","bond1","bond2","bond3"]}
{"id":"carbon1","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"gray","x":100,"y":100}
{"id":"hydrogen1","type":"ellipse","cx":8,"cy":8,"rx":8,"ry":8,"fill":"white","stroke":"black","x":150,"y":90}
{"id":"hydrogen2","type":"ellipse","cx":8,"cy":8,"rx":8,"ry":8,"fill":"white","stroke":"black","x":150,"y":110}
{"id":"oxygen1","type":"ellipse","cx":12,"cy":12,"rx":12,"ry":12,"fill":"red","x":200,"y":100}
{"id":"bond1","type":"line","start_x":115,"start_y":110,"end_x":150,"end_y":100,"stroke_color":"black","stroke_width":2,"x":0,"y":0}
{"id":"bond2","type":"line","start_x":115,"start_y":115,"end_x":150,"end_y":118,"stroke_color":"black","stroke_width":2,"x":0,"y":0}
{"id":"bond3","type":"line","start_x":130,"start_y":115,"end_x":200,"end_y":112,"stroke_color":"black","stroke_width":2,"x":0,"y":0}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("07_molecular_model.svg"), "Modelo Molecular")
}

// 8. Sistema Solar
fn generate_solar_system(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":500,"height":500,"background":"black","children":["sun","mercury_orbit","venus_orbit","earth_orbit","mercury","venus","earth","moon"]}
{"id":"sun","type":"ellipse","cx":25,"cy":25,"rx":25,"ry":25,"fill":"yellow","x":225,"y":225}
{"id":"mercury_orbit","type":"ellipse","cx":250,"cy":250,"rx":80,"ry":80,"fill":"none","stroke":"gray","x":0,"y":0}
{"id":"venus_orbit","type":"ellipse","cx":250,"cy":250,"rx":120,"ry":120,"fill":"none","stroke":"gray","x":0,"y":0}
{"id":"earth_orbit","type":"ellipse","cx":250,"cy":250,"rx":160,"ry":160,"fill":"none","stroke":"gray","x":0,"y":0}
{"id":"mercury","type":"ellipse","cx":4,"cy":4,"rx":4,"ry":4,"fill":"orange","x":320,"y":246}
{"id":"venus","type":"ellipse","cx":6,"cy":6,"rx":6,"ry":6,"fill":"yellow","x":360,"y":244}
{"id":"earth","type":"ellipse","cx":8,"cy":8,"rx":8,"ry":8,"fill":"blue","x":400,"y":242}
{"id":"moon","type":"ellipse","cx":2,"cy":2,"rx":2,"ry":2,"fill":"gray","x":415,"y":240}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("08_solar_system.svg"), "Sistema Solar")
}

// 9. Patrón Fractal Simple
fn generate_fractal_pattern(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":400,"height":400,"children":["center","nw","ne","sw","se","nw_nw","nw_ne","nw_sw","nw_se"]}
{"id":"center","type":"rect","width":100,"height":100,"background":"blue","x":150,"y":150}
{"id":"nw","type":"rect","width":50,"height":50,"background":"lightblue","x":100,"y":100}
{"id":"ne","type":"rect","width":50,"height":50,"background":"lightblue","x":250,"y":100}
{"id":"sw","type":"rect","width":50,"height":50,"background":"lightblue","x":100,"y":250}
{"id":"se","type":"rect","width":50,"height":50,"background":"lightblue","x":250,"y":250}
{"id":"nw_nw","type":"rect","width":25,"height":25,"background":"cyan","x":75,"y":75}
{"id":"nw_ne","type":"rect","width":25,"height":25,"background":"cyan","x":125,"y":75}
{"id":"nw_sw","type":"rect","width":25,"height":25,"background":"cyan","x":75,"y":125}
{"id":"nw_se","type":"rect","width":25,"height":25,"background":"cyan","x":125,"y":125}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("09_fractal_pattern.svg"), "Patrón Fractal Simple")
}

// 10. Mandala Geométrico
fn generate_geometric_mandala(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let jsonl_input = r##"
{"id":"root","type":"free_container","width":300,"height":300,"children":["center_circle","ring1_1","ring1_2","ring1_3","ring1_4","ring1_5","ring1_6"]}
{"id":"center_circle","type":"ellipse","cx":20,"cy":20,"rx":20,"ry":20,"fill":"purple","x":130,"y":130}
{"id":"ring1_1","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":135,"y":80}
{"id":"ring1_2","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":185,"y":105}
{"id":"ring1_3","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":185,"y":155}
{"id":"ring1_4","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":135,"y":180}
{"id":"ring1_5","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":85,"y":155}
{"id":"ring1_6","type":"ellipse","cx":15,"cy":15,"rx":15,"ry":15,"fill":"blue","x":85,"y":105}
"##;

    generate_svg_from_jsonl(jsonl_input, output_dir.join("10_geometric_mandala.svg"), "Mandala Geométrico")
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