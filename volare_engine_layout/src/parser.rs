use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::transform::Transform;
use crate::{components::*, diagram_builder::*, DiagramBuilder, SimpleConstraint};
use anyhow::{bail, Context, Error, Result};
use thiserror::Error;

/// Simplified JSON Lines entity with only essential fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,

    // All attributes go into this single map - much cleaner!
    #[serde(flatten)]
    pub attributes: Map<String, Value>,
}

impl Default for JsonEntity {
    fn default() -> Self {
        Self {
            id: String::new(),
            entity_type: String::new(),
            attributes: Map::new(),
        }
    }
}

// Add this enum after JsonEntity struct
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ConstraintDeclaration {
    #[serde(rename = "align_left")]
    AlignLeft { entities: Vec<String> },
    #[serde(rename = "align_right")]
    AlignRight { entities: Vec<String> },
    #[serde(rename = "align_top")]
    AlignTop { entities: Vec<String> },
    #[serde(rename = "align_bottom")]
    AlignBottom { entities: Vec<String> },
    #[serde(rename = "align_center_horizontal")]
    AlignCenterHorizontal { entities: Vec<String> },
    #[serde(rename = "align_center_vertical")]
    AlignCenterVertical { entities: Vec<String> },
    #[serde(rename = "right_of")]
    RightOf { entities: Vec<String> },
    #[serde(rename = "left_of")]
    LeftOf { entities: Vec<String> },
    #[serde(rename = "above")]
    Above { entities: Vec<String> },
    #[serde(rename = "below")]
    Below { entities: Vec<String> },
    #[serde(rename = "horizontal_spacing")]
    HorizontalSpacing {
        entities: Vec<String>,
        spacing: Float,
    },
    #[serde(rename = "vertical_spacing")]
    VerticalSpacing {
        entities: Vec<String>,
        spacing: Float,
    },
    #[serde(rename = "stack_horizontal")]
    StackHorizontal {
        entities: Vec<String>,
        spacing: Float,
    },
    #[serde(rename = "stack_vertical")]
    StackVertical {
        entities: Vec<String>,
        spacing: Float,
    },
    #[serde(rename = "fixed_distance")]
    FixedDistance {
        entities: Vec<String>,
        distance: Float,
    },
    #[serde(rename = "same_width")]
    SameWidth { entities: Vec<String> },
    #[serde(rename = "same_height")]
    SameHeight { entities: Vec<String> },
    #[serde(rename = "same_size")]
    SameSize { entities: Vec<String> },
    #[serde(rename = "proportional_width")]
    ProportionalWidth { entities: Vec<String>, ratio: Float },
    #[serde(rename = "proportional_height")]
    ProportionalHeight { entities: Vec<String>, ratio: Float },
    #[serde(rename = "aspect_ratio")]
    AspectRatio { entity: String, ratio: Float },
}

fn convert_constraint_declaration(decl: &ConstraintDeclaration) -> Result<SimpleConstraint> {
    match decl {
        ConstraintDeclaration::AlignLeft { entities } => {
            Ok(SimpleConstraint::AlignLeft(entities.to_vec()))
        }
        ConstraintDeclaration::AlignRight { entities } => {
            Ok(SimpleConstraint::AlignRight(entities.to_vec()))
        }
        ConstraintDeclaration::AlignTop { entities } => {
            Ok(SimpleConstraint::AlignTop(entities.to_vec()))
        }
        ConstraintDeclaration::AlignBottom { entities } => {
            Ok(SimpleConstraint::AlignBottom(entities.to_vec()))
        }
        ConstraintDeclaration::AlignCenterHorizontal { entities } => {
            Ok(SimpleConstraint::AlignCenterHorizontal(entities.to_vec()))
        }
        ConstraintDeclaration::AlignCenterVertical { entities } => {
            Ok(SimpleConstraint::AlignCenterVertical(entities.to_vec()))
        }
        ConstraintDeclaration::RightOf { entities } => {
            if entities.len() != 2 {
                bail!("right_of requires exactly 2 entities");
            }
            Ok(SimpleConstraint::RightOf(
                entities[0].clone(),
                entities[1].clone(),
            ))
        }
        ConstraintDeclaration::LeftOf { entities } => {
            if entities.len() != 2 {
                bail!("left_of requires exactly 2 entities");
            }
            Ok(SimpleConstraint::LeftOf(
                entities[0].clone(),
                entities[1].clone(),
            ))
        }
        ConstraintDeclaration::Above { entities } => {
            if entities.len() != 2 {
                bail!("above requires exactly 2 entities")
            }
            Ok(SimpleConstraint::Above(
                entities[0].clone(),
                entities[1].clone(),
            ))
        }
        ConstraintDeclaration::Below { entities } => {
            if entities.len() != 2 {
                bail!("below requires exactly 2 entities");
            }
            Ok(SimpleConstraint::Below(
                entities[0].clone(),
                entities[1].clone(),
            ))
        }
        ConstraintDeclaration::HorizontalSpacing { entities, spacing } => {
            if entities.len() != 2 {
                bail!("horizontal_spacing requires exactly 2 entities");
            }
            Ok(SimpleConstraint::HorizontalSpacing(
                entities[0].clone(),
                entities[1].clone(),
                *spacing,
            ))
        }
        ConstraintDeclaration::VerticalSpacing { entities, spacing } => {
            if entities.len() != 2 {
                bail!("vertical_spacing requires exactly 2 entities");
            }
            Ok(SimpleConstraint::VerticalSpacing(
                entities[0].clone(),
                entities[1].clone(),
                *spacing,
            ))
        }
        ConstraintDeclaration::StackHorizontal { entities, spacing } => {
            if entities.len() < 2 {
                bail!("stack_horizontal requires at least 2 entities");
            }
            Ok(SimpleConstraint::StackHorizontal(
                entities.clone(),
                Some(*spacing),
            ))
        }
        ConstraintDeclaration::StackVertical { entities, spacing } => {
            if entities.len() < 2 {
                bail!("stack_vertical requires at least 2 entities");
            }
            Ok(SimpleConstraint::StackVertical(
                entities.clone(),
                Some(*spacing),
            ))
        }
        ConstraintDeclaration::FixedDistance { entities, distance } => {
            if entities.len() != 2 {
                bail!("fixed_distance requires exactly 2 entities");
            }
            Ok(SimpleConstraint::FixedDistance(
                entities[0].clone(),
                entities[1].clone(),
                *distance,
            ))
        }
        ConstraintDeclaration::SameWidth { entities } => {
            Ok(SimpleConstraint::SameWidth(entities.to_vec()))
        }
        ConstraintDeclaration::SameHeight { entities } => {
            Ok(SimpleConstraint::SameHeight(entities.to_vec()))
        }
        ConstraintDeclaration::SameSize { entities } => {
            Ok(SimpleConstraint::SameSize(entities.to_vec()))
        }
        ConstraintDeclaration::ProportionalWidth { entities, ratio } => {
            if entities.len() != 2 {
                bail!("proportional_width requires exactly 2 entities");
            }
            Ok(SimpleConstraint::ProportionalWidth(
                entities[0].clone(),
                entities[1].clone(),
                *ratio,
            ))
        }
        ConstraintDeclaration::ProportionalHeight { entities, ratio } => {
            if entities.len() != 2 {
                bail!("proportional_height requires exactly 2 entities");
            }
            Ok(SimpleConstraint::ProportionalHeight(
                entities[0].clone(),
                entities[1].clone(),
                *ratio,
            ))
        }
        ConstraintDeclaration::AspectRatio { entity, ratio } => {
            Ok(SimpleConstraint::AspectRatio(entity.clone(), *ratio))
        }
    }
}

// Helper functions for attribute extraction with multiple attribute name support
pub fn get_string_attr(attrs: &Map<String, Value>, keys: &[&str], default: &str) -> String {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(s) = value.as_str() {
                return if s.is_empty() && !default.is_empty() {
                    default.to_string()
                } else {
                    s.to_string()
                };
            }
        }
    }
    default.to_string()
}

pub fn get_float_attr(attrs: &Map<String, Value>, keys: &[&str], default: f64) -> Float {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(f) = value.as_f64() {
                return f as Float;
            }
        }
    }
    default as Float
}

pub fn get_int_attr(attrs: &Map<String, Value>, keys: &[&str], default: i64) -> i64 {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(i) = value.as_i64() {
                return i;
            }
        }
    }
    default
}

pub fn get_bool_attr(attrs: &Map<String, Value>, keys: &[&str], default: bool) -> bool {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(b) = value.as_bool() {
                return b;
            }
        }
    }
    default
}

pub fn get_array_attr(attrs: &Map<String, Value>, key: &str) -> Option<Vec<String>> {
    attrs.get(key).and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
    })
}

pub fn get_point_attr(
    attrs: &Map<String, Value>,
    x_keys: &[&str],
    y_keys: &[&str],
    default: (Float, Float),
) -> (Float, Float) {
    let x = get_float_attr(attrs, x_keys, default.0 as f64);
    let y = get_float_attr(attrs, y_keys, default.1 as f64);
    (x, y)
}

pub fn get_points_attr(attrs: &Map<String, Value>, key: &str) -> Option<Vec<(Float, Float)>> {
    attrs.get(key).and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    item.as_array().and_then(|point_arr| {
                        if point_arr.len() >= 2 {
                            let x = point_arr[0].as_f64().unwrap_or(0.0) as Float;
                            let y = point_arr[1].as_f64().unwrap_or(0.0) as Float;
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
    })
}

/// Parse a unified width/height value that can be either a number (fixed) or string (behavior)
pub fn parse_unified_dimension(attrs: &Map<String, Value>, keys: &[&str]) -> SizeBehavior {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            match value {
                Value::Number(num) => {
                    if let Some(float_val) = num.as_f64() {
                        return SizeBehavior::Fixed(float_val as Float);
                    }
                }
                Value::String(behavior) => {
                    return match behavior.to_lowercase().as_str() {
                        "content" | "auto" => SizeBehavior::Content,
                        "grow" => SizeBehavior::Grow,
                        _ => {
                            // Try to parse as number string
                            if let Ok(parsed) = behavior.parse::<Float>() {
                                SizeBehavior::Fixed(parsed)
                            } else {
                                SizeBehavior::Content
                            }
                        }
                    };
                }
                _ => {}
            }
        }
    }
    SizeBehavior::Content // Default
}
fn parse_transform_attributes(
    obj: &Map<String, Value>,
    session: &mut DiagramBuilder,
    entity_id: EntityID,
) {
    println!("🔍 Parsing transforms for entity: {}", entity_id);
    println!(
        "🔍 Available attributes: {:?}",
        obj.keys().collect::<Vec<_>>()
    );

    let mut transform = Transform::identity();

    // Parse individual transform properties
    // Store container-relative position separately
    if let Some(x) = obj.get("x").and_then(|v| v.as_f64()) {
        if let Some(y) = obj.get("y").and_then(|v| v.as_f64()) {
            session.set_container_relative_position(entity_id.clone(), x as Float, y as Float);
        }
    }

    if let Some(rotation) = obj
        .get("rotation")
        .or_else(|| obj.get("rotate"))
        .and_then(|v| v.as_f64())
    {
        println!(
            "🔄 Found rotation: {} degrees for entity {}",
            rotation, entity_id
        );
        transform = transform.combine(&Transform::rotation(rotation as Float));
    } else {
        println!("❌ No rotation found for entity {}", entity_id);
    }

    if let Some(scale) = obj.get("scale") {
        match scale {
            Value::Number(s) => {
                let s = s.as_f64().unwrap_or(1.0) as Float;
                println!("📏 Found uniform scale: {}", s);
                transform = transform.combine(&Transform::scale(s, s));
            }
            Value::Array(arr) if arr.len() >= 2 => {
                let sx = arr[0].as_f64().unwrap_or(1.0) as Float;
                let sy = arr[1].as_f64().unwrap_or(1.0) as Float;
                println!("📏 Found scale: [{}, {}]", sx, sy);
                transform = transform.combine(&Transform::scale(sx, sy));
            }
            _ => {}
        }
    }

    // Parse CSS-style transform string
    if let Some(transform_str) = obj.get("transform").and_then(|v| v.as_str()) {
        if let Ok(parsed_transform) = parse_css_transform(transform_str) {
            transform = transform.combine(&parsed_transform);
        }
    }

    println!("📐 Final transform for {}: {:?}", entity_id, transform);
    session.set_transform(entity_id, transform);
}

// Parse CSS-style transform strings like "rotate(45deg) scale(1.5) translate(10px, 20px)"
fn parse_css_transform(transform_str: &str) -> Result<Transform, String> {
    // Implementation would parse CSS transform functions
    // For now, simplified version:
    let result = Transform::identity();

    // This is a simplified parser - full implementation would be more robust
    if transform_str.contains("rotate(") {
        // Extract rotation value...
    }

    Ok(result)
}

/// Parser for JSON Lines diagram format
pub struct JsonLinesParser {
    pub entities: HashMap<String, JsonEntity>,
}

impl JsonLinesParser {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    /// Parse from a string containing JSON Lines
    pub fn parse_string(&mut self, input: &str) -> Result<String, JsonLinesError> {
        let mut root_id = None;

        for (line_num, line) in input.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonEntity>(line) {
                Ok(entity) => {
                    if root_id.is_none() {
                        root_id = Some(entity.id.clone());
                    }
                    self.entities.insert(entity.id.clone(), entity);
                }
                Err(e) => {
                    return Err(JsonLinesError::ParseError {
                        line: line_num + 1,
                        message: e.to_string(),
                    });
                }
            }
        }

        root_id.ok_or(JsonLinesError::NoEntities)
    }

    /// Parse from a file
    pub fn parse_file(&mut self, file_path: &str) -> Result<String, JsonLinesError> {
        let file = File::open(file_path).map_err(|e| JsonLinesError::IoError(e.to_string()))?;
        let reader = BufReader::new(file);
        let mut root_id = None;

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| JsonLinesError::IoError(e.to_string()))?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonEntity>(&line) {
                Ok(entity) => {
                    if root_id.is_none() {
                        root_id = Some(entity.id.clone());
                    }
                    self.entities.insert(entity.id.clone(), entity);
                }
                Err(e) => {
                    return Err(JsonLinesError::ParseError {
                        line: line_num + 1,
                        message: e.to_string(),
                    });
                }
            }
        }

        root_id.ok_or(JsonLinesError::NoEntities)
    }

    /// Parse from an iterator of lines (useful for streaming)
    pub fn parse_lines<I>(&mut self, lines: I) -> Result<String, JsonLinesError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut root_id = None;

        for (line_num, line) in lines.into_iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonEntity>(line) {
                Ok(entity) => {
                    if root_id.is_none() {
                        root_id = Some(entity.id.clone());
                    }
                    self.entities.insert(entity.id.clone(), entity);
                }
                Err(e) => {
                    return Err(JsonLinesError::ParseError {
                        line: line_num + 1,
                        message: e.to_string(),
                    });
                }
            }
        }

        root_id.ok_or(JsonLinesError::NoEntities)
    }

    /// Build the diagram tree from parsed entities
    pub fn build(&self, root_id: &str, builder: &mut DiagramBuilder) -> Result<DiagramTreeNode> {
        self.build_entity(root_id, builder)
    }

    fn build_entity(
        &self,
        entity_id: &str,
        builder: &mut DiagramBuilder,
    ) -> Result<DiagramTreeNode> {
        println!("*** building entity {} ***", entity_id);
        let entity = self
            .entities
            .get(entity_id)
            .ok_or_else(|| JsonLinesError::EntityNotFound(entity_id.to_string()))?;

        // Clone the entity type to avoid borrow conflicts
        let component_type = entity.entity_type.clone();
        let attributes = entity.attributes.clone();
        println!("Attributes length {}", attributes.len());

        // Check for custom components FIRST - they get the raw attributes map
        if builder.has_custom_component(&component_type) {
            return builder.create_custom_component(&entity_id, &component_type, &attributes, &self);
        }

        // Handle built-in components using attribute helpers
        match entity.entity_type.as_str() {
            "spacer" => {
                let width = get_float_attr(&entity.attributes, &["width"], 1.0);
                let height = get_float_attr(&entity.attributes, &["height"], 20.0);
                let direction = get_string_attr(&entity.attributes, &["direction"], "vertical");

                // Determine spacer direction and final dimensions
                let spacer_direction = match direction.as_str() {
                    "horizontal" => SpacerDirection::Horizontal,
                    "both" => SpacerDirection::Both,
                    _ => SpacerDirection::Vertical, // default
                };

                let spacer_options = SpacerOptions {
                    width,
                    height,
                    direction: spacer_direction,
                };

                Ok(builder.new_spacer(entity_id.to_string(), spacer_options))
            }

            "text" => {
                let content = get_string_attr(&entity.attributes, &["content", "text"], "");
                if content.is_empty() {
                    bail!("Missing attribute content or text");
                }

                // Only support "bold", otherwise return the value or "400" by default
                let str_font_weight = &get_string_attr(&entity.attributes, &["font_weight"], "400");
                let f_weight: u32 = if let Ok(val) = str::parse::<u32>(&str_font_weight) {
                    val
                } else if str_font_weight == "bold " {
                    900
                } else {
                    400
                };

                let options = TextOptions {
                    font_size: get_float_attr(&entity.attributes, &["font_size"], 12.0),
                    text_color: get_string_attr(
                        &entity.attributes,
                        &["color", "text_color"],
                        "black",
                    ),
                    font_weight: f_weight,
                    font_family: get_string_attr(&entity.attributes, &["font_family"], "Arial"),
                    line_width: get_int_attr(&entity.attributes, &["line_width"], 200) as usize,
                    line_spacing: get_float_attr(&entity.attributes, &["line_spacing"], 0.0),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_text(entity_id.to_string(), &content, options))
            }

            "box" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                if children.len() != 1 {
                    bail!("Box must have exactly one child");
                }

                let child = self.build_entity(&children[0], builder)?;

                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let options = BoxOptions {
                    padding: get_float_attr(&entity.attributes, &["padding"], 0.0),
                    fill_color: {
                        let color = get_string_attr(
                            &entity.attributes,
                            &["background", "background_color", "fill"],
                            "white",
                        );
                        Fill::Color(color)
                    },
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["border_color", "stroke_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(
                        &entity.attributes,
                        &["border_width", "stroke_width"],
                        1.0,
                    ),
                    border_radius: get_float_attr(&entity.attributes, &["border_radius"], 0.0),
                    width_behavior,
                    height_behavior,
                    // TODO: leer de atributo
                    horizontal_alignment: HorizontalAlignment::Center,
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_box(entity_id.to_string(), child, options))
            }

            "vstack" => {
                let halign = match get_string_attr(
                    &entity.attributes,
                    &["h_align", "horizontal_alignment"],
                    "center",
                )
                .as_str()
                {
                    "left" => HorizontalAlignment::Left,
                    "center" => HorizontalAlignment::Center,
                    "right" => HorizontalAlignment::Right,
                    _ => HorizontalAlignment::Center,
                };

                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Vec<_> = children
                    .iter()
                    .filter_map(|child_id| {
                        match self.build_entity(child_id, builder) {
                            Ok(node) => Some(node),
                            Err(e) => {
                                eprintln!("Warning: Failed to build child '{}': {}", child_id, e);
                                None // Skip this child, continue with others
                            }
                        }
                    })
                    .collect();

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_vstack(entity_id.to_string(), child_nodes, halign))
            }

            "hstack" => {
                let valign = match get_string_attr(
                    &entity.attributes,
                    &["v_align", "vertical_alignment"],
                    "center",
                )
                .as_str()
                {
                    "top" => VerticalAlignment::Top,
                    "center" => VerticalAlignment::Center,
                    "bottom" => VerticalAlignment::Bottom,
                    _ => VerticalAlignment::Center,
                };

                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Vec<_> = children
                    .iter()
                    .filter_map(|child_id| {
                        match self.build_entity(child_id, builder) {
                            Ok(node) => Some(node),
                            Err(e) => {
                                eprintln!("Warning: Failed to build child '{}': {}", child_id, e);
                                None // Skip this child, continue with others
                            }
                        }
                    })
                    .collect();

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_hstack(entity_id.to_string(), child_nodes, valign))
            }

            "group" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Vec<_> = children
                    .iter()
                    .filter_map(|child_id| {
                        match self.build_entity(child_id, builder) {
                            Ok(node) => Some(node),
                            Err(e) => {
                                eprintln!("Warning: Failed to build child '{}': {}", child_id, e);
                                None // Skip this child, continue with others
                            }
                        }
                    })
                    .collect();

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_group(entity_id.to_string(), child_nodes))
            }

            "rect" => {
                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let options = RectOptions {
                    width_behavior,
                    height_behavior,
                    fill_color: {
                        let color = get_string_attr(
                            &entity.attributes,
                            &["background", "background_color", "fill"],
                            "white",
                        );
                        Fill::Color(color)
                    },
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["border_color", "stroke_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(
                        &entity.attributes,
                        &["border_width", "stroke_width"],
                        1.0,
                    ),
                    border_radius: get_float_attr(&entity.attributes, &["border_radius"], 0.0),
                };
                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_rectangle(entity_id.to_string(), options))
            }

            "line" => {
                let start_point = get_point_attr(
                    &entity.attributes,
                    &["start_x", "x1"],
                    &["start_y", "y1"],
                    (0.0, 0.0),
                );
                let end_point = get_point_attr(
                    &entity.attributes,
                    &["end_x", "x2"],
                    &["end_y", "y2"],
                    (0.0, 0.0),
                );

                let options = LineOptions {
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke_color", "color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_line(
                    entity_id.to_string(),
                    LinePointReference::Value(start_point.0, start_point.1),
                    LinePointReference::Value(end_point.0, end_point.1),
                    options,
                ))
            }

            "ellipse" => {
                let radius = get_point_attr(
                    &entity.attributes,
                    &["rx", "radius_x"],
                    &["ry", "radius_y"],
                    (25.0, 25.0),
                );

                let options = EllipseOptions {
                    fill_color: get_string_attr(
                        &entity.attributes,
                        &["fill", "fill_color", "background"],
                        "white",
                    ),
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke", "stroke_color", "border_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(
                        &entity.attributes,
                        &["stroke_width", "border_width"],
                        1.0,
                    ),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_elipse(entity_id.to_string(), radius, options))
            }

            "arc" => {
                let radius = get_float_attr(&entity.attributes, &["radius", "r"], 50.0);
                let start_angle =
                    get_float_attr(&entity.attributes, &["start_angle", "start"], 0.0);
                let end_angle = get_float_attr(&entity.attributes, &["end_angle", "end"], 90.0);

                let options = ArcOptions {
                    fill_color: get_string_attr(
                        &entity.attributes,
                        &["fill", "fill_color"],
                        "none",
                    ),
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke", "stroke_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                    filled: get_bool_attr(&entity.attributes, &["filled"], false),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_arc(
                    entity_id.to_string(),
                    radius,
                    start_angle,
                    end_angle,
                    options,
                ))
            }

            // Complete fixed semicircle section for parser.rs
            "semicircle" => {
                let radius = get_float_attr(&entity.attributes, &["radius", "r"], 50.0);
                let facing_up = get_bool_attr(&entity.attributes, &["facing_up", "up"], true);

                let (start, end) = if facing_up {
                    (180.0, 360.0) // FIXED: Top semicircle should be 180° to 360°
                } else {
                    (0.0, 180.0) // FIXED: Bottom semicircle should be 0° to 180°
                };

                let options = ArcOptions {
                    fill_color: get_string_attr(
                        &entity.attributes,
                        &["fill", "fill_color"],
                        "none",
                    ),
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke", "stroke_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                    filled: get_bool_attr(&entity.attributes, &["filled"], false),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_arc(entity_id.to_string(), radius, start, end, options))
            }

            "quarter_circle" => {
                let radius = get_float_attr(&entity.attributes, &["radius", "r"], 50.0);
                let quadrant = get_int_attr(&entity.attributes, &["quadrant"], 1) as u8;

                let options = ArcOptions {
                    fill_color: get_string_attr(
                        &entity.attributes,
                        &["fill", "fill_color"],
                        "none",
                    ),
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke", "stroke_color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                    filled: get_bool_attr(&entity.attributes, &["filled"], false),
                };

                // Parse and apply transforms
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_quarter_circle(entity_id.to_string(), radius, quadrant, options))
            }

            "image" => {
                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let src = get_string_attr(&entity.attributes, &["src"], "");
                let file_path = get_string_attr(&entity.attributes, &["file_path"], "");

                if !src.is_empty() {
                    parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                    Ok(builder.new_image(
                        entity_id.to_string(),
                        &src,
                        (width_behavior, height_behavior),
                    ))
                } else if !file_path.is_empty() {
                    parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());
                    Ok(builder.new_image_from_file(
                        entity_id.to_string(),
                        &file_path,
                        (width_behavior, height_behavior),
                    ))
                } else {
                    Err(JsonLinesError::MissingAttribute("src or file_path".to_string()).into())
                }
            }

            "table" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;
                let cols = get_int_attr(&entity.attributes, &["cols", "columns"], 1) as usize;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                let options = TableOptions {
                    header_fill_color: get_string_attr(
                        &entity.attributes,
                        &["header_fill_color", "header_background"],
                        "lightgray",
                    ),
                    fill_color: get_string_attr(
                        &entity.attributes,
                        &["fill_color", "background"],
                        "white",
                    ),
                    border_color: get_string_attr(&entity.attributes, &["border_color"], "black"),
                    border_width: get_int_attr(&entity.attributes, &["border_width"], 1) as usize,
                    cell_padding: get_int_attr(&entity.attributes, &["cell_padding", "padding"], 20)
                        as Float,
                    with_header: true,
                };

                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_table(entity_id.to_string(), child_nodes?, cols, options))
            }

            "polyline" => {
                let points = get_points_attr(&entity.attributes, "points")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("points".to_string()))?;

                let options = LineOptions {
                    stroke_color: get_string_attr(
                        &entity.attributes,
                        &["stroke_color", "color"],
                        "black",
                    ),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                };
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_polyline(entity_id.to_string(), points, options))
            }

            "free_container" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let mut positioned_children = Vec::new();
                for child_id in children {
                    let _child_entity = self
                        .entities
                        .get(&child_id)
                        .ok_or_else(|| JsonLinesError::EntityNotFound(child_id.clone()))?;

                    let child_node = self.build_entity(&child_id, builder)?;
                    let pos = builder.get_container_relative_position(&child_id);
                    println!(
                        "📍 Free container adding child: {} at position: ({}, {})",
                        child_id, pos.x, pos.y
                    );

                    positioned_children.push((child_node, (pos.x, pos.y)));
                }
                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_free_container(entity_id.to_string(), positioned_children))
            }

            "constraint_container" => {
                let children = get_array_attr(&entity.attributes, "children").unwrap_or_default();

                // Parse inline constraints if present
                let mut constraints = Vec::new();
                if let Some(constraints_attr) = entity.attributes.get("constraints") {
                    if let Some(constraints_array) = constraints_attr.as_array() {
                        for constraint_val in constraints_array {
                            if let Ok(constraint_decl) =
                                serde_json::from_value::<ConstraintDeclaration>(
                                    constraint_val.clone(),
                                )
                            {
                                let simple_constraint =
                                    convert_constraint_declaration(&constraint_decl)?;
                                constraints.push(simple_constraint);
                            }
                        }
                    }
                }

                let mut children_with_pos = Vec::new();

                for child_id in children {
                    let _child_entity = self
                        .entities
                        .get(&child_id)
                        .ok_or_else(|| JsonLinesError::EntityNotFound(child_id.clone()))?;

                    if let Ok(child_node) = self.build_entity(&child_id, builder) {
                        let pos = builder.get_container_relative_position(&child_id);

                        // For constraint containers, position is optional (constraints determine positioning)
                        let suggest_pos = if pos.x != 0.0 || pos.y != 0.0 {
                            Some(pos)
                        } else {
                            None
                        };

                        children_with_pos.push((child_node, suggest_pos));
                    }
                }

                parse_transform_attributes(&entity.attributes, builder, entity_id.to_string());

                Ok(builder.new_constraint_layout_container(
                    entity_id.to_string(),
                    children_with_pos,
                    constraints,
                ))
            }
            _ => Err(JsonLinesError::UnknownEntityType(entity.entity_type.clone()).into()),
        }
    }

    /// Validate that all child references exist
    pub fn validate(&self) -> Result<(), JsonLinesError> {
        for (id, entity) in &self.entities {
            if let Some(children) = get_array_attr(&entity.attributes, "children") {
                for child_id in children {
                    if !self.entities.contains_key(&child_id) {
                        return Err(JsonLinesError::MissingChild {
                            parent: id.clone(),
                            child: child_id,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Get all entity IDs
    pub fn get_entity_ids(&self) -> Vec<&String> {
        self.entities.keys().collect()
    }
}

/// Builder for creating JSON Lines diagrams
pub struct JsonLinesBuilder {
    entities: Vec<JsonEntity>,
}

impl JsonLinesBuilder {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Create a text entity
    pub fn text(&mut self, id: String, content: &str) -> String {
        let mut attrs = Map::new();
        attrs.insert("content".to_string(), Value::String(content.to_string()));

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "text".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a styled text entity
    pub fn text_styled(
        &mut self,
        id: String,
        content: &str,
        font_size: f64,
        color: &str,
    ) -> String {
        let mut attrs = Map::new();
        attrs.insert("content".to_string(), Value::String(content.to_string()));
        attrs.insert(
            "font_size".to_string(),
            Value::Number(serde_json::Number::from_f64(font_size).unwrap()),
        );
        attrs.insert("color".to_string(), Value::String(color.to_string()));

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "text".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a box entity
    pub fn box_with(
        &mut self,
        id: String,
        child: String,
        padding: f64,
        background: &str,
    ) -> String {
        let mut attrs = Map::new();
        attrs.insert(
            "children".to_string(),
            Value::Array(vec![Value::String(child)]),
        );
        attrs.insert(
            "padding".to_string(),
            Value::Number(serde_json::Number::from_f64(padding).unwrap()),
        );
        attrs.insert(
            "background".to_string(),
            Value::String(background.to_string()),
        );

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "box".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a vertical stack
    pub fn vstack(&mut self, id: String, children: Vec<String>) -> String {
        let mut attrs = Map::new();
        attrs.insert(
            "children".to_string(),
            Value::Array(children.into_iter().map(Value::String).collect()),
        );

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "vstack".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a horizontal stack
    pub fn hstack(&mut self, id: String, children: Vec<String>) -> String {
        let mut attrs = Map::new();
        attrs.insert(
            "children".to_string(),
            Value::Array(children.into_iter().map(Value::String).collect()),
        );

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "hstack".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a rectangle
    pub fn rect(&mut self, id: String, width: f64, height: f64, color: &str) -> String {
        let mut attrs = Map::new();
        attrs.insert(
            "width".to_string(),
            Value::Number(serde_json::Number::from_f64(width).unwrap()),
        );
        attrs.insert(
            "height".to_string(),
            Value::Number(serde_json::Number::from_f64(height).unwrap()),
        );
        attrs.insert("background".to_string(), Value::String(color.to_string()));

        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "rect".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a custom component entity
    pub fn custom_component(
        &mut self,
        id: String,
        component_type: &str,
        attributes: Map<String, Value>,
    ) -> String {
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: component_type.to_string(),
            attributes,
        });
        id
    }

    /// Build and return the JSON Lines string
    pub fn build(&self) -> Result<String, serde_json::Error> {
        let mut lines = Vec::new();
        for entity in &self.entities {
            lines.push(serde_json::to_string(entity)?);
        }
        Ok(lines.join("\n"))
    }

    /// Write to a file
    pub fn write_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(file_path)?;
        for entity in &self.entities {
            writeln!(file, "{}", serde_json::to_string(entity)?)?;
        }
        Ok(())
    }

    /// Get the root entity ID (first entity)
    pub fn root_id(&self) -> Option<String> {
        self.entities.first().map(|e| e.id.clone())
    }
}

#[derive(Debug)]
pub enum JsonLinesError {
    ParseError { line: usize, message: String },
    EntityNotFound(String),
    MissingAttribute(String),
    InvalidStructure(String),
    UnknownEntityType(String),
    MissingChild { parent: String, child: String },
    NoEntities,
    IoError(String),
    ConstraintError(String),
}

impl std::fmt::Display for JsonLinesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonLinesError::ParseError { line, message } => {
                write!(f, "Parse error on line {}: {}", line, message)
            }
            JsonLinesError::EntityNotFound(id) => write!(f, "Entity not found: {}", id),
            JsonLinesError::MissingAttribute(attr) => {
                write!(f, "Missing required attribute: {}", attr)
            }
            JsonLinesError::InvalidStructure(msg) => write!(f, "Invalid structure: {}", msg),
            JsonLinesError::UnknownEntityType(t) => write!(f, "Unknown entity type: {}", t),
            JsonLinesError::MissingChild { parent, child } => {
                write!(f, "Parent {} references missing child {}", parent, child)
            }
            JsonLinesError::NoEntities => write!(f, "No entities found"),
            JsonLinesError::IoError(msg) => write!(f, "IO error: {}", msg),
            JsonLinesError::ConstraintError(msg) => write!(f, "Constraint error: {}", msg),
        }
    }
}

impl std::error::Error for JsonLinesError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiagramBuilder;
    use serde_json::json;

    #[test]
    fn test_simplified_json_lines_parsing() {
        let input = r#"
{"id":"root","type":"box","padding":10,"background":"white","children":["text1"]}
{"id":"text1","type":"text","content":"Hello World","font_size":16,"color":"blue"}
"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();
        assert_eq!(root_id, "root");

        parser.validate().unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        let _diagram = parser.build(&root_id, &mut builder).unwrap();
    }

    #[test]
    fn test_attribute_aliases() {
        // Test that multiple attribute names work for the same concept
        let input = r#"
{"id":"box1","type":"box","padding":5,"background_color":"red","children":["text1"]}
{"id":"text1","type":"text","text":"Using text instead of content","text_color":"white"}
"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        let diagram = parser.build(&root_id, &mut builder);

        assert!(diagram.is_ok());
    }

    #[test]
    fn test_custom_component_access() {
        // Test that custom components get all attributes
        fn test_component(
            attrs: &Map<String, Value>,
            _builder: &mut DiagramBuilder,
        ) -> Result<DiagramTreeNode> {
            // Should be able to access any attribute
            assert!(attrs.contains_key("custom_prop"));
            assert!(attrs.contains_key("width"));
            assert!(attrs.contains_key("background"));

            // Return a dummy node for testing
            Ok(DiagramTreeNode::new(
                EntityType::TextShape,
                "test_component".to_string(),
            ))
        }

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        builder.register_custom_component("test_comp", test_component);

        let input = r#"{"id":"test1","type":"test_comp","width":200,"background":"blue","custom_prop":"value"}"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();
        let diagram = parser.build(&root_id, &mut builder);

        assert!(diagram.is_ok());
    }

    #[test]
    fn test_streaming_parse() {
        let lines = vec![
            r#"{"id":"e1","type":"text","content":"Hello"}"#.to_string(),
            r#"{"id":"e2","type":"text","content":"World"}"#.to_string(),
            r#"{"id":"e3","type":"hstack","children":["e1","e2"]}"#.to_string(),
        ];

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_lines(lines).unwrap();
        assert_eq!(root_id, "e1");
        parser.validate().unwrap();
    }

    #[test]
    fn test_builder_api() {
        let mut builder = JsonLinesBuilder::new();

        let title = builder.text_styled("title".to_string(), "Document Title", 18.0, "blue");
        let left_text = builder.text("left_text".to_string(), "Left Panel");
        let right_text = builder.text("right_text".to_string(), "Right Panel");

        let left_box = builder.box_with("left_box".to_string(), left_text, 10.0, "lightblue");
        let right_box = builder.box_with("right_box".to_string(), right_text, 10.0, "lightgreen");

        let content = builder.hstack("content".to_string(), vec![left_box, right_box]);
        let footer = builder.text_styled("footer".to_string(), "Footer", 12.0, "gray");

        let _root = builder.vstack("root".to_string(), vec![title, content, footer]);

        let jsonl = builder.build().unwrap();
        println!("Generated JSON Lines:\n{}", jsonl);

        // Parse it back to verify
        let mut parser = JsonLinesParser::new();
        parser.parse_string(&jsonl).unwrap();
        parser.validate().unwrap();
    }

    #[test]
    fn test_complex_attributes() {
        // Test points for polyline
        let input = r#"{"id":"poly1","type":"polyline","points":[[0,0],[10,10],[20,0]],"stroke_color":"red"}"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        let diagram = parser.build(&root_id, &mut builder);

        assert!(diagram.is_ok());
    }

    #[test]
    fn test_size_behaviors() {
        // Test different size behavior specifications
        let input = r#"
{"id":"box1","type":"box","width":"content","height":100,"children":["text1"]}
{"id":"text1","type":"text","content":"Auto-sized"}
"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        let diagram = parser.build(&root_id, &mut builder);

        assert!(diagram.is_ok());
    }
}

// Example of what an LLM might generate with the new simplified format
pub fn example_llm_generated_jsonl() -> &'static str {
    r#"{"id":"document","type":"box","padding":20,"background":"white","border_color":"gray","children":["layout"]}
{"id":"layout","type":"vstack","children":["header","body","footer"]}
{"id":"header","type":"text","content":"My Document","font_size":24,"color":"darkblue"}
{"id":"body","type":"hstack","children":["sidebar","main"]}
{"id":"sidebar","type":"box","padding":15,"background":"lightgray","children":["nav"]}
{"id":"nav","type":"vstack","children":["link1","link2","link3"]}
{"id":"link1","type":"text","content":"Home","color":"blue"}
{"id":"link2","type":"text","content":"About","color":"blue"}
{"id":"link3","type":"text","content":"Contact","color":"blue"}
{"id":"main","type":"box","padding":15,"background":"white","children":["content"]}
{"id":"content","type":"vstack","children":["article_title","article_body"]}
{"id":"article_title","type":"text","content":"Article Title","font_size":18}
{"id":"article_body","type":"text","content":"This is the main content of the article..."}
{"id":"footer","type":"text","content":"Copyright 2024","font_size":10,"color":"gray"}"#
}

#[cfg(test)]
mod transform_debug_tests {
    use super::*;
    use crate::DiagramBuilder;

    #[test]
    fn test_transform_parsing_debug() {
        println!("🧪 Testing transform parsing...");

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as f32 * 8.0, 16.0));

        // Create a simple JSON object with rotation
        let mut attributes = serde_json::Map::new();
        attributes.insert(
            "rotation".to_string(),
            serde_json::Value::Number(serde_json::Number::from(45)),
        );
        attributes.insert(
            "width".to_string(),
            serde_json::Value::Number(serde_json::Number::from(60)),
        );
        attributes.insert(
            "height".to_string(),
            serde_json::Value::Number(serde_json::Number::from(40)),
        );

        println!("🔍 Attributes: {:?}", attributes);

        // Test the function directly
        parse_transform_attributes(&attributes, &mut builder, "test_entity".to_string());

        // Check if the transform was applied
        let transform = builder.get_transform("test_entity".to_string());
        println!("📐 Result transform: {:?}", transform);

        // Check if it's not just identity
        let is_identity = transform.matrix[0] == 1.0
            && transform.matrix[1] == 0.0
            && transform.matrix[2] == 0.0
            && transform.matrix[3] == 1.0
            && transform.matrix[4] == 0.0
            && transform.matrix[5] == 0.0;

        println!("❓ Is identity transform: {}", is_identity);
        assert!(
            !is_identity,
            "Transform should not be identity - rotation should be applied!"
        );
    }

    #[test]
    fn test_full_parser_with_rotation() {
        println!("🧪 Testing full parser with rotation...");

        let input = r#"{"id":"test_rect","type":"rect","width":60,"height":40,"background":"red","rotation":45}"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as f32 * 8.0, 16.0));

        let diagram = parser.build(&root_id, &mut builder).unwrap();

        // Check the transform
        let transform = builder.get_transform("test_rect".to_string());
        println!("📐 Full parser result transform: {:?}", transform);

        let is_identity = transform.matrix[0] == 1.0
            && transform.matrix[1] == 0.0
            && transform.matrix[2] == 0.0
            && transform.matrix[3] == 1.0
            && transform.matrix[4] == 0.0
            && transform.matrix[5] == 0.0;

        println!("❓ Full parser - Is identity transform: {}", is_identity);
        assert!(!is_identity, "Full parser should apply rotation!");
    }
}

// Also add this debug function to see what's happening during build_entity calls:
impl JsonLinesParser {
    // Add this method to your existing JsonLinesParser impl
    pub fn debug_build_entity(
        &mut self,
        entity_id: &str,
        builder: &mut DiagramBuilder,
    ) -> Result<DiagramTreeNode> {
        println!("🏗️ Building entity: {}", entity_id);

        let entity = self
            .entities
            .get(entity_id)
            .ok_or_else(|| JsonLinesError::EntityNotFound(entity_id.to_string()))?;

        println!("🏗️ Entity type: {}", entity.entity_type);
        println!(
            "🏗️ Entity attributes: {:?}",
            entity.attributes.keys().collect::<Vec<_>>()
        );

        // Check if rotation attribute exists
        if let Some(rotation_value) = entity.attributes.get("rotation") {
            println!("🔄 Found rotation attribute: {:?}", rotation_value);
        } else {
            println!("❌ No rotation attribute found");
        }

        // Call the original build_entity
        self.build_entity(entity_id, builder)
    }
}
