use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::{components::*, DiagramBuilder, diagram_builder::*};

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

// Helper functions for attribute extraction with multiple attribute name support
fn get_string_attr(attrs: &Map<String, Value>, keys: &[&str], default: &str) -> String {
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

fn get_float_attr(attrs: &Map<String, Value>, keys: &[&str], default: f64) -> Float {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(f) = value.as_f64() {
                return f as Float;
            }
        }
    }
    default as Float
}

fn get_int_attr(attrs: &Map<String, Value>, keys: &[&str], default: i64) -> i64 {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(i) = value.as_i64() {
                return i;
            }
        }
    }
    default
}

fn get_bool_attr(attrs: &Map<String, Value>, keys: &[&str], default: bool) -> bool {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(b) = value.as_bool() {
                return b;
            }
        }
    }
    default
}

fn get_array_attr(attrs: &Map<String, Value>, key: &str) -> Option<Vec<String>> {
    attrs.get(key).and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
    })
}

fn get_point_attr(attrs: &Map<String, Value>, x_keys: &[&str], y_keys: &[&str], default: (Float, Float)) -> (Float, Float) {
    let x = get_float_attr(attrs, x_keys, default.0 as f64);
    let y = get_float_attr(attrs, y_keys, default.1 as f64);
    (x, y)
}

fn get_points_attr(attrs: &Map<String, Value>, key: &str) -> Option<Vec<(Float, Float)>> {
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
fn parse_unified_dimension(attrs: &Map<String, Value>, keys: &[&str]) -> SizeBehavior {
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

/// Parser for JSON Lines diagram format
pub struct JsonLinesParser {
    entities: HashMap<String, JsonEntity>,
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
    pub fn build(
        &self,
        root_id: &str,
        builder: &mut DiagramBuilder,
    ) -> Result<DiagramTreeNode, JsonLinesError> {
        self.build_entity(root_id, builder)
    }

    fn build_entity(
        &self,
        entity_id: &str,
        builder: &mut DiagramBuilder,
    ) -> Result<DiagramTreeNode, JsonLinesError> {
        let entity = self
            .entities
            .get(entity_id)
            .ok_or_else(|| JsonLinesError::EntityNotFound(entity_id.to_string()))?;

            // Clone the entity type to avoid borrow conflicts
            let component_type = entity.entity_type.clone();
            let attributes = entity.attributes.clone();

        // Check for custom components FIRST - they get the raw attributes map
        if builder.has_custom_component(&component_type) {
            return builder.create_custom_component(
                &component_type,
                &attributes,
            ).map_err(|e| JsonLinesError::InvalidStructure(e));
        }

        // Handle built-in components using attribute helpers
        match entity.entity_type.as_str() {
            "text" => {
                let content = get_string_attr(&entity.attributes, &["content", "text"], "");
                if content.is_empty() {
                    return Err(JsonLinesError::MissingAttribute("content or text".to_string()));
                }

                let options = TextOptions {
                    font_size: get_float_attr(&entity.attributes, &["font_size"], 12.0),
                    text_color: get_string_attr(&entity.attributes, &["color", "text_color"], "black"),
                    font_family: get_string_attr(&entity.attributes, &["font_family"], "Arial"),
                    line_width: get_int_attr(&entity.attributes, &["line_width"], 200) as usize,
                    line_spacing: get_float_attr(&entity.attributes, &["line_spacing"], 0.0),
                };

                Ok(builder.new_text(entity_id.to_string(), &content, options))
            }

            "box" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                if children.len() != 1 {
                    return Err(JsonLinesError::InvalidStructure(
                        "Box must have exactly one child".to_string(),
                    ));
                }

                let child = self.build_entity(&children[0], builder)?;

                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let options = BoxOptions {
                    padding: get_float_attr(&entity.attributes, &["padding"], 0.0),
                    fill_color: {
                        let color = get_string_attr(&entity.attributes, &["background", "background_color", "fill"], "white");
                        Fill::Color(color)
                    },
                    stroke_color: get_string_attr(&entity.attributes, &["border_color", "stroke_color"], "black"),
                    stroke_width: get_float_attr(&entity.attributes, &["border_width", "stroke_width"], 1.0),
                    border_radius: get_float_attr(&entity.attributes, &["border_radius"], 0.0),
                    width_behavior,
                    height_behavior,
                };

                Ok(builder.new_box(entity_id.to_string(), child, options))
            }

            "vstack" => {
                let halign = match get_string_attr(&entity.attributes, &["h_align", "horizontal_alignment"], "center").as_str() {
                    "left" => HorizontalAlignment::Left,
                    "center" => HorizontalAlignment::Center,
                    "right" => HorizontalAlignment::Right,
                    _ => HorizontalAlignment::Center,
                };

                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_vstack(entity_id.to_string(),child_nodes?, halign))
            }

            "hstack" => {
                let valign = match get_string_attr(&entity.attributes, &["v_align", "vertical_alignment"], "center").as_str() {
                    "top" => VerticalAlignment::Top,
                    "center" => VerticalAlignment::Center,
                    "bottom" => VerticalAlignment::Bottom,
                    _ => VerticalAlignment::Center,
                };

                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_hstack(entity_id.to_string(),child_nodes?, valign))
            }

            "group" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_group(entity_id.to_string(),child_nodes?))
            }

            "rect" => {
                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let options = RectOptions {
                    width_behavior,
                    height_behavior,
                    fill_color: {
                        let color = get_string_attr(&entity.attributes, &["background", "background_color", "fill"], "white");
                        Fill::Color(color)
                    },
                    stroke_color: get_string_attr(&entity.attributes, &["border_color", "stroke_color"], "black"),
                    stroke_width: get_float_attr(&entity.attributes, &["border_width", "stroke_width"], 1.0),
                    border_radius: get_float_attr(&entity.attributes, &["border_radius"], 0.0),
                };

                Ok(builder.new_rectangle(entity_id.to_string(),options))
            }

            "line" => {
                let start_point = get_point_attr(&entity.attributes, &["start_x", "x1"], &["start_y", "y1"], (0.0, 0.0));
                let end_point = get_point_attr(&entity.attributes, &["end_x", "x2"], &["end_y", "y2"], (0.0, 0.0));

                let options = LineOptions {
                    stroke_color: get_string_attr(&entity.attributes, &["stroke_color", "color"], "black"),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                };

                Ok(builder.new_line(entity_id.to_string(),start_point, end_point, options))
            }

            "ellipse" => {
                let center = get_point_attr(&entity.attributes, &["cx", "center_x"], &["cy", "center_y"], (50.0, 50.0));
                let radius = get_point_attr(&entity.attributes, &["rx", "radius_x"], &["ry", "radius_y"], (25.0, 25.0));

                let options = EllipseOptions {
                    fill_color: get_string_attr(&entity.attributes, &["fill", "fill_color", "background"], "white"),
                    stroke_color: get_string_attr(&entity.attributes, &["stroke", "stroke_color", "border_color"], "black"),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width", "border_width"], 1.0),
                };

                Ok(builder.new_elipse(entity_id.to_string(),center, radius, options))
            }

            "image" => {
                let width_behavior = parse_unified_dimension(&entity.attributes, &["width"]);
                let height_behavior = parse_unified_dimension(&entity.attributes, &["height"]);

                let src = get_string_attr(&entity.attributes, &["src"], "");
                let file_path = get_string_attr(&entity.attributes, &["file_path"], "");

                if !src.is_empty() {
                    Ok(builder.new_image(entity_id.to_string(),&src, (width_behavior, height_behavior)))
                } else if !file_path.is_empty() {
                    Ok(builder.new_image_from_file(entity_id.to_string(),&file_path, (width_behavior, height_behavior)))
                } else {
                    Err(JsonLinesError::MissingAttribute("src or file_path".to_string()))
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
                    header_fill_color: get_string_attr(&entity.attributes, &["header_fill_color", "header_background"], "lightgray"),
                    fill_color: get_string_attr(&entity.attributes, &["fill_color", "background"], "white"),
                    border_color: get_string_attr(&entity.attributes, &["border_color"], "black"),
                    border_width: get_int_attr(&entity.attributes, &["border_width"], 1) as usize,
                    cell_padding: get_int_attr(&entity.attributes, &["cell_padding", "padding"], 20) as usize,
                };

                Ok(builder.new_table(entity_id.to_string(),child_nodes?, cols, options))
            }

            "polyline" => {
                let points = get_points_attr(&entity.attributes, "points")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("points".to_string()))?;

                let options = LineOptions {
                    stroke_color: get_string_attr(&entity.attributes, &["stroke_color", "color"], "black"),
                    stroke_width: get_float_attr(&entity.attributes, &["stroke_width"], 1.0),
                };

                Ok(builder.new_polyline(entity_id.to_string(),points, options))
            }

            "free_container" => {
                let children = get_array_attr(&entity.attributes, "children")
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let mut positioned_children = Vec::new();
                for child_id in children {
                    let child_entity = self
                        .entities
                        .get(&child_id)
                        .ok_or_else(|| JsonLinesError::EntityNotFound(child_id.clone()))?;

                    let child_node = self.build_entity(&child_id, builder)?;
                    let position = get_point_attr(&child_entity.attributes, &["x"], &["y"], (0.0, 0.0));
                    positioned_children.push((child_node, position));
                }

                Ok(builder.new_free_container(entity_id.to_string(),positioned_children))
            }

            _ => Err(JsonLinesError::UnknownEntityType(
                entity.entity_type.clone(),
            )),
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
    id_counter: usize,
}

impl JsonLinesBuilder {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn next_id(&mut self) -> String {
        self.id_counter += 1;
        format!("e{}", self.id_counter)
    }

    /// Create a text entity
    pub fn text(&mut self, content: &str) -> String {
        let id = self.next_id();
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
    pub fn text_styled(&mut self, content: &str, font_size: f64, color: &str) -> String {
        let id = self.next_id();
        let mut attrs = Map::new();
        attrs.insert("content".to_string(), Value::String(content.to_string()));
        attrs.insert("font_size".to_string(), Value::Number(serde_json::Number::from_f64(font_size).unwrap()));
        attrs.insert("color".to_string(), Value::String(color.to_string()));
        
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "text".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a box entity
    pub fn box_with(&mut self, child: String, padding: f64, background: &str) -> String {
        let id = self.next_id();
        let mut attrs = Map::new();
        attrs.insert("children".to_string(), Value::Array(vec![Value::String(child)]));
        attrs.insert("padding".to_string(), Value::Number(serde_json::Number::from_f64(padding).unwrap()));
        attrs.insert("background".to_string(), Value::String(background.to_string()));
        
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "box".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a vertical stack
    pub fn vstack(&mut self, children: Vec<String>) -> String {
        let id = self.next_id();
        let mut attrs = Map::new();
        attrs.insert("children".to_string(), Value::Array(children.into_iter().map(Value::String).collect()));
        
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "vstack".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a horizontal stack
    pub fn hstack(&mut self, children: Vec<String>) -> String {
        let id = self.next_id();
        let mut attrs = Map::new();
        attrs.insert("children".to_string(), Value::Array(children.into_iter().map(Value::String).collect()));
        
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "hstack".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a rectangle
    pub fn rect(&mut self, width: f64, height: f64, color: &str) -> String {
        let id = self.next_id();
        let mut attrs = Map::new();
        attrs.insert("width".to_string(), Value::Number(serde_json::Number::from_f64(width).unwrap()));
        attrs.insert("height".to_string(), Value::Number(serde_json::Number::from_f64(height).unwrap()));
        attrs.insert("background".to_string(), Value::String(color.to_string()));
        
        self.entities.push(JsonEntity {
            id: id.clone(),
            entity_type: "rect".to_string(),
            attributes: attrs,
        });
        id
    }

    /// Create a custom component entity
    pub fn custom_component(&mut self, component_type: &str, attributes: Map<String, Value>) -> String {
        let id = self.next_id();
        
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
        }
    }
}

impl std::error::Error for JsonLinesError {}

#[cfg(test)]
mod tests {
    use crate::DiagramBuilder;
    use super::*;
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
        ) -> Result<DiagramTreeNode, String> {
            // Should be able to access any attribute
            assert!(attrs.contains_key("custom_prop"));
            assert!(attrs.contains_key("width"));
            assert!(attrs.contains_key("background"));
            
            // Return a dummy node for testing
            Ok(DiagramTreeNode::new(EntityType::TextShape, 1))
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

        let title = builder.text_styled("Document Title", 18.0, "blue");
        let left_text = builder.text("Left Panel");
        let right_text = builder.text("Right Panel");

        let left_box = builder.box_with(left_text, 10.0, "lightblue");
        let right_box = builder.box_with(right_text, 10.0, "lightgreen");

        let content = builder.hstack(vec![left_box, right_box]);
        let footer = builder.text_styled("Footer", 12.0, "gray");

        let _root = builder.vstack(vec![title, content, footer]);

        let jsonl = builder.build().unwrap();
        println!("Generated JSON Lines:\n{}", jsonl);

        // Parse it back to verify
        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(&jsonl).unwrap();
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