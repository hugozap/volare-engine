// src/bin/dense_city.rs
// Demo program that generates a super dense city using loops

use demo::measure_text::measure_text_svg_character_advance;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏙️ Generando Ciudad Súper Densa con Loops...\n");

    // Create output directory
    let output_dir = std::env::temp_dir().join("dense_city");
    std::fs::create_dir_all(&output_dir)?;

    // Generate the dense city
    generate_dense_city(&output_dir)?;

    println!("\n✅ Ciudad densa generada exitosamente!");
    println!("📁 Archivo guardado en: {}", output_dir.display());

    Ok(())
}

fn generate_dense_city(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️ Construyendo ciudad con loops...");
    
    let mut jsonl_lines = Vec::new();
    
    // Configuración de la ciudad
    let city_width = 2000;
    let city_height = 1500;
    let block_size = 100;
    let street_width = 8;
    
    // Colores para diferentes tipos de edificios
    let residential_colors = ["#DEB887", "#F4A460", "#CD853F", "#D2691E", "#8B4513", "#A0522D"];
    let commercial_colors = ["#4169E1", "#32CD32", "#FF6347", "#FFD700", "#9370DB", "#20B2AA"];
    let office_colors = ["#708090", "#2F4F4F", "#696969", "#778899", "#B0C4DE", "#87CEEB"];
    let industrial_colors = ["#808080", "#696969", "#A9A9A9", "#778899", "#708090"];
    
    // Root container
    jsonl_lines.push(format!(
        r##"{{"id":"root","type":"free_container","width":{},"height":{},"background":"#87CEEB","children":["street_grid","districts","labels","transportation","landmarks"]}}"##,
        city_width, city_height
    ));
    
    // Generar grid de calles
    let mut street_children = Vec::new();
    let mut street_id = 0;
    
    // Calles horizontales
    for i in 0..(city_height / block_size + 1) {
        let y = i * block_size;
        let id = format!("h_street_{}", street_id);
        street_children.push(id.clone());
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"#696969","x":0,"y":{}}}"##,
            id, city_width, street_width, y
        ));
        street_id += 1;
    }
    
    // Calles verticales
    for i in 0..(city_width / block_size + 1) {
        let x = i * block_size;
        let id = format!("v_street_{}", street_id);
        street_children.push(id.clone());
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"#696969","x":{},"y":0}}"##,
            id, street_width, city_height, x
        ));
        street_id += 1;
    }
    
    // Container para calles
    jsonl_lines.push(format!(
        r##"{{"id":"street_grid","type":"free_container","width":{},"height":{},"children":[{}],"x":0,"y":0}}"##,
        city_width, city_height,
        street_children.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    
    // Generar distritos con loops
    let mut district_children = Vec::new();
    
    // 1. Distrito Residencial (noroeste)
    let mut residential_buildings = Vec::new();
    for block_y in 0..6 {
        for block_x in 0..8 {
            for house_y in 0..3 {
                for house_x in 0..4 {
                    let id = format!("res_{}_{}_{}_{}", block_x, block_y, house_x, house_y);
                    let x = block_x * block_size + house_x * 22 + 12;
                    let y = block_y * block_size + house_y * 28 + 12;
                    let width = 18 + (house_x * 2);
                    let height = 20 + (house_y * 3);
                    let color = residential_colors[(house_x + house_y) % residential_colors.len()];
                    
                    residential_buildings.push(id.clone());
                    jsonl_lines.push(format!(
                        r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"{}","x":{},"y":{}}}"##,
                        id, width, height, color, x, y
                    ));
                }
            }
        }
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"residential_district","type":"free_container","width":800,"height":600,"children":[{}],"x":0,"y":0}}"##,
        residential_buildings.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    district_children.push("residential_district".to_string());
    
    // 2. Distrito Comercial (centro)
    let mut commercial_buildings = Vec::new();
    for block_y in 0..8 {
        for block_x in 8..14 {
            for bldg_y in 0..2 {
                for bldg_x in 0..3 {
                    let id = format!("com_{}_{}_{}_{}", block_x, block_y, bldg_x, bldg_y);
                    let x = block_x * block_size + bldg_x * 30 + 10;
                    let y = block_y * block_size + bldg_y * 45 + 10;
                    let width = 25 + (bldg_x * 5);
                    let height = 35 + (bldg_y * 10) + (block_y * 2);
                    let color = commercial_colors[(bldg_x + bldg_y + block_y) % commercial_colors.len()];
                    
                    commercial_buildings.push(id.clone());
                    jsonl_lines.push(format!(
                        r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"{}","x":{},"y":{}}}"##,
                        id, width, height, color, x, y
                    ));
                }
            }
        }
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"commercial_district","type":"free_container","width":600,"height":800,"children":[{}],"x":800,"y":0}}"##,
        commercial_buildings.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    district_children.push("commercial_district".to_string());
    
    // 3. Distrito de Oficinas (suroeste)
    let mut office_buildings = Vec::new();
    for block_y in 6..12 {
        for block_x in 0..10 {
            let id = format!("office_{}_{}", block_x, block_y);
            let x = block_x * block_size + 15;
            let y = block_y * block_size + 15;
            let width = 70;
            let height = 60 + (block_x * 8) + (block_y * 3);
            let color = office_colors[(block_x + block_y) % office_colors.len()];
            
            office_buildings.push(id.clone());
            jsonl_lines.push(format!(
                r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"{}","x":{},"y":{}}}"##,
                id, width, height, color, x, y
            ));
        }
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"office_district","type":"free_container","width":1000,"height":600,"children":[{}],"x":0,"y":600}}"##,
        office_buildings.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    district_children.push("office_district".to_string());
    
    // 4. Zona Industrial (sureste)
    let mut industrial_buildings = Vec::new();
    for block_y in 8..15 {
        for block_x in 10..20 {
            let id = format!("ind_{}_{}", block_x, block_y);
            let x = block_x * block_size + 20;
            let y = block_y * block_size + 20;
            let width = 60 + (block_x % 3) * 15;
            let height = 40 + (block_y % 4) * 10;
            let color = industrial_colors[(block_x + block_y) % industrial_colors.len()];
            
            industrial_buildings.push(id.clone());
            jsonl_lines.push(format!(
                r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"{}","x":{},"y":{}}}"##,
                id, width, height, color, x, y
            ));
        }
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"industrial_district","type":"free_container","width":1000,"height":700,"children":[{}],"x":1000,"y":800}}"##,
        industrial_buildings.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    district_children.push("industrial_district".to_string());
    
    // 5. Rascacielos en el centro
    let mut skyscraper_buildings = Vec::new();
    for i in 0..15 {
        for j in 0..8 {
            let id = format!("sky_{}_{}", i, j);
            let x = 1400 + i * 35;
            let y = 200 + j * 70;
            let width = 25 + (i % 3) * 5;
            let height = 100 + i * 12 + j * 8;
            let color_idx = (i + j) % office_colors.len();
            let color = office_colors[color_idx];
            
            skyscraper_buildings.push(id.clone());
            jsonl_lines.push(format!(
                r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"{}","x":{},"y":{}}}"##,
                id, width, height, color, x, y
            ));
        }
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"skyscraper_district","type":"free_container","width":600,"height":800,"children":[{}],"x":1400,"y":0}}"##,
        skyscraper_buildings.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    district_children.push("skyscraper_district".to_string());
    
    // Container para todos los distritos
    jsonl_lines.push(format!(
        r##"{{"id":"districts","type":"free_container","width":{},"height":{},"children":[{}],"x":0,"y":0}}"##,
        city_width, city_height,
        district_children.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    
    // Generar etiquetas para distritos
    let mut label_children = Vec::new();
    let labels = [
        ("Residential Quarter", 400, 50),
        ("Commercial District", 1100, 50),
        ("Business Center", 500, 650),
        ("Industrial Zone", 1500, 850),
        ("Downtown Core", 1700, 50),
    ];
    
    for (i, (text, x, y)) in labels.iter().enumerate() {
        let id = format!("label_{}", i);
        label_children.push(id.clone());
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"text","content":"{}","font_size":16,"color":"#000080","x":{},"y":{}}}"##,
            id, text, x, y
        ));
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"labels","type":"free_container","width":{},"height":{},"children":[{}],"x":0,"y":0}}"##,
        city_width, city_height,
        label_children.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    
    // Generar sistema de transporte con loops
    let mut transport_children = Vec::new();
    
    // Metro lines
    let metro_lines = [
        (("#FF0000", 6), vec![(0, 300), (500, 305), (1000, 300), (1500, 305), (2000, 300)]),
        (("#00FF00", 6), vec![(0, 600), (600, 605), (1200, 600), (1800, 605), (2000, 600)]),
        (("#0000FF", 6), vec![(300, 0), (305, 400), (300, 800), (305, 1200), (300, 1500)]),
        (("#FFFF00", 6), vec![(900, 0), (905, 350), (900, 700), (905, 1050), (900, 1500)]),
    ];
    
    for (line_idx, ((color, width), points)) in metro_lines.iter().enumerate() {
        let id = format!("metro_line_{}", line_idx);
        transport_children.push(id.clone());
        let points_str = points.iter()
            .map(|(x, y)| format!("[{},{}]", x, y))
            .collect::<Vec<_>>()
            .join(",");
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"polyline","points":[{}],"stroke_color":"{}","stroke_width":{},"x":0,"y":0}}"##,
            id, points_str, color, width
        ));
    }
    
    // Estaciones de metro con validación
    for i in 0..20 {
        for j in 0..15 {
            if (i * 150) % 300 == 0 && (j * 100) % 300 == 0 {
                let id = format!("station_{}_{}", i, j);
                transport_children.push(id.clone());
                jsonl_lines.push(format!(
                    r##"{{"id":"{}","type":"rect","width":8,"height":8,"background":"white","stroke_color":"black","x":{},"y":{}}}"##,
                    id, i * 100 + 296, j * 100 + 296
                ));
            }
        }
    }
    
    // Autopistas con loops
    let highway_segments = [
        (0, 900, 2000, 900, 12),
        (1500, 0, 1500, 1500, 10),
    ];
    
    for (seg_idx, (x1, y1, x2, y2, width)) in highway_segments.iter().enumerate() {
        let id = format!("highway_{}", seg_idx);
        transport_children.push(id.clone());
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"polyline","points":[[{},{}],[{},{}]],"stroke_color":"#2F2F2F","stroke_width":{},"x":0,"y":0}}"##,
            id, x1, y1, x2, y2, width
        ));
    }
    
    jsonl_lines.push(format!(
        r##"{{"id":"transportation","type":"free_container","width":{},"height":{},"children":[{}],"x":0,"y":0}}"##,
        city_width, city_height,
        transport_children.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    
    // Generar landmarks importantes
    let mut landmark_children = Vec::new();
    
    // Parques generados con loops
    let parks = [(400, 400, 150, 100), (1200, 200, 120, 80), (800, 1000, 180, 120)];
    for (park_idx, (x, y, w, h)) in parks.iter().enumerate() {
        let park_id = format!("park_{}", park_idx);
        landmark_children.push(park_id.clone());
        jsonl_lines.push(format!(
            r##"{{"id":"{}","type":"rect","width":{},"height":{},"background":"#32CD32","x":{},"y":{}}}"##,
            park_id, w, h, x, y
        ));
        
        // Árboles en el parque
        for tree_x in 0..(w / 20) {
            for tree_y in 0..(h / 20) {
                let tree_id = format!("tree_{}_{}_{}", park_idx, tree_x, tree_y);
                landmark_children.push(tree_id.clone());
                jsonl_lines.push(format!(
                    r##"{{"id":"{}","type":"rect","width":4,"height":4,"background":"#228B22","x":{},"y":{}}}"##,
                    tree_id, x + tree_x * 20 + 8, y + tree_y * 20 + 8
                ));
            }
        }
    }
    
    // Río serpenteante
    let mut river_points = Vec::new();
    for i in 0..40 {
        let x = i * 50;
        let y = 1200.0 + (i as f32 * 0.5).sin() * 100.0;
        river_points.push(format!("[{},{}]", x, y as i32));
    }
    landmark_children.push("river".to_string());
    jsonl_lines.push(format!(
        r##"{{"id":"river","type":"polyline","points":[{}],"stroke_color":"#4169E1","stroke_width":20,"x":0,"y":0}}"##,
        river_points.join(",")
    ));
    
    jsonl_lines.push(format!(
        r##"{{"id":"landmarks","type":"free_container","width":{},"height":{},"children":[{}],"x":0,"y":0}}"##,
        city_width, city_height,
        landmark_children.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")
    ));
    
    // Escribir el JSONL generado con validación
    let jsonl_content = jsonl_lines.join("\n");
    
    // Validar JSON antes de continuar
    println!("🔍 Validando JSONL generado...");
    let mut parser_test = parser::JsonLinesParser::new();
    match parser_test.parse_string(&jsonl_content) {
        Ok(_) => println!("✅ JSONL válido!"),
        Err(e) => {
            println!("❌ Error en JSONL: {:?}", e);
            // Escribir a archivo para debug
            let debug_path = output_dir.join("debug_jsonl.txt");
            std::fs::write(&debug_path, &jsonl_content)?;
            println!("🐛 JSONL escrito a {} para debug", debug_path.display());
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSONL inválido: {:?}", e)
            )));
        }
    }
    
    println!("📊 Estadísticas de la ciudad:");
    println!("   🏠 Edificios residenciales: {}", residential_buildings.len());
    println!("   🏢 Edificios comerciales: {}", commercial_buildings.len());
    println!("   🏭 Edificios de oficinas: {}", office_buildings.len());
    println!("   🏗️ Edificios industriales: {}", industrial_buildings.len());
    println!("   🏙️ Rascacielos: {}", skyscraper_buildings.len());
    println!("   📏 Total de líneas JSONL: {}", jsonl_lines.len());
    
    generate_svg_from_jsonl(&jsonl_content, output_dir.join("dense_city.svg"), "Ciudad Súper Densa")
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