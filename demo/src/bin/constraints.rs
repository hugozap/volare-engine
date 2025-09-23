// src/bin/intricate_city.rs
// Demo program that generates an intricate city using only polylines, rects, and free containers

use demo::measure_text::measure_text_svg_character_advance;
use resvg::usvg::roxmltree::Children;
use uuid::fmt::Simple;
use volare_engine_layout::diagram_builder::DiagramTreeNode;
use std::fs::File;
use std::path::Path;
use volare_engine_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cassowary constraints demo...\n");
    let mut builder = DiagramBuilder::new();
    let output_dir = std::env::temp_dir().join("constraints_demo");
    std::fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("constraints1.svg");
    builder.set_measure_text_fn(measure_text_svg_character_advance);
    let children = vec![builder.new_rectangle(
        "r1".to_string(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(50.0),
            height_behavior: SizeBehavior::Fixed(20.0),
            fill_color: Fill::Color("red".to_string()),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    ),
    builder.new_rectangle(
        "r2".to_string(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(50.0),
            height_behavior: SizeBehavior::Fixed(20.0),
            fill_color: Fill::Color("blue".to_string()),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    ),
    builder.new_rectangle(
        "r3".to_string(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(50.0),
            height_behavior: SizeBehavior::Fixed(20.0),
            fill_color: Fill::Color("green".to_string()),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    ),
      builder.new_rectangle(
        "r4".to_string(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(50.0),
            height_behavior: SizeBehavior::Fixed(20.0),
            fill_color: Fill::Color("purple".to_string()),
            stroke_color: "black".to_string(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    )
    ];

    let mut children_with_pos: Vec<(DiagramTreeNode, Option<Point>)>= children.iter().map(|c| {
        (c.clone(), None)
    }).collect();

     if let Some(elem) = children_with_pos.get_mut(0) {
        elem.1  = Some(Point::new(0.0, 0.0))
     }

    builder.set_position("r1".to_string(), 50.0, 50.0);

    let mut constraints = Vec::<SimpleConstraint>::new();
    // R1 should be set to the right of r2
    constraints.push(SimpleConstraint::RightOf("r1".into(), "r2".into()));
    constraints.push(SimpleConstraint::Below("r3".into(), "r2".into()));
    constraints.push(SimpleConstraint::Above("r4".into(), "r1".into()));
    constraints.push(SimpleConstraint::RightOf("r1".into(), "r4".into()));
    let root = builder.new_constraint_layout_container("container".to_string(), children_with_pos, constraints);
    layout_tree_node(&mut builder, &root);
    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&output_path)?;
    svg_renderer.render(&builder, &root, &mut svg_file)?;

    println!("    ✅ Guardado: {}",output_path.to_str().unwrap());
  

    Ok(())
}
