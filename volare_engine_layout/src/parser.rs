use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::{components::*, DiagramBuilder, diagram_builder::*};

/// A JSON Lines entity representing a single diagram element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,

    // Common attributes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,

    // Text-specific attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    // Box/Rectangle attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Value>,

    // Position attributes (for free containers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<Float>,

    // Image attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    // Table attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_fill_color: Option<String>,

    // Line/Polyline attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_point: Option<(Float, Float)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_point: Option<(Float, Float)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<(Float, Float)>>,

    // Ellipse attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<(Float, Float)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<(Float, Float)>,

    // Catch-all for any other attributes
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}


// 2. Helper function to parse unified width/height values

/// Parse a unified width/height value that can be either a number (fixed) or string (behavior)
fn parse_unified_dimension(value: Option<&Value>) -> SizeBehavior {
    match value {
        Some(Value::Number(num)) => {
            // Numeric value = fixed size
            if let Some(float_val) = num.as_f64() {
                SizeBehavior::Fixed(float_val as Float)
            } else {
                SizeBehavior::Content // Fallback for invalid numbers
            }
        }
        Some(Value::String(behavior)) => {
            // String value = behavior specification
            match behavior.to_lowercase().as_str() {
                "content" => SizeBehavior::Content,
                "grow" => SizeBehavior::Grow,
                "auto" => SizeBehavior::Content, // Alias for content
                _ => {
                    // Try to parse as number string (e.g., "300")
                    if let Ok(parsed) = behavior.parse::<Float>() {
                        SizeBehavior::Fixed(parsed)
                    } else {
                        SizeBehavior::Content // Fallback for unknown strings
                    }
                }
            }
        }
        _ => {
            // No value or unsupported type = content behavior (default)
            SizeBehavior::Content
        }
    }
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

        match entity.entity_type.as_str() {
            "text" => {
                let content = entity
                    .content
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("content".to_string()))?;

                let options = TextOptions {
                    font_size: entity.font_size.unwrap_or(12.0),
                    text_color: entity.color.clone().unwrap_or_else(|| "black".to_string()),
                    font_family: entity
                        .font_family
                        .clone()
                        .unwrap_or_else(|| "Arial".to_string()),
                    ..Default::default()
                };

                Ok(builder.new_text(content, options))
            }

            "box" => {
                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                if children.len() != 1 {
                    return Err(JsonLinesError::InvalidStructure(
                        "Box must have exactly one child".to_string(),
                    ));
                }

                let child = self.build_entity(&children[0], builder)?;

                let width_behavior = parse_unified_dimension(entity.width.as_ref());
                let height_behavior = parse_unified_dimension(entity.height.as_ref());

                
                let options = BoxOptions {
                    padding: entity.padding.unwrap_or(0.0),
                    fill_color: entity
                        .background
                        .as_ref()
                        .map(|bg| Fill::Color(bg.clone()))
                        .unwrap_or(Fill::Color("white".to_string())),
                    stroke_color: entity
                        .border_color
                        .clone()
                        .unwrap_or_else(|| "black".to_string()),
                    stroke_width: entity.border_width.unwrap_or(1.0),
                    border_radius: entity.border_radius.unwrap_or(0.0),
                    width_behavior,
                    height_behavior,
                    ..BoxOptions::default()
                };

                Ok(builder.new_box(child, options))
            }

            "vstack" => {
                
                let halign = entity
                    .extra
                    .get("h-align")
                    .and_then(|v| v.as_str())
                    .and_then(|s| match s {
                        "left" => Some(HorizontalAlignment::Left),
                        "center" => Some(HorizontalAlignment::Center),
                        "right" => Some(HorizontalAlignment::Right),
                        _ => None,
                    })
                    .unwrap_or(HorizontalAlignment::Center);

                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_vstack(child_nodes?, halign))
            }

            "hstack" => {
                let valign = entity
                    .extra
                    .get("v-align")
                    .and_then(|v| v.as_str())
                    .and_then(|s| match s {
                        "top" => Some(VerticalAlignment::Top),
                        "center" => Some(VerticalAlignment::Center),
                        "bottom" => Some(VerticalAlignment::Bottom),
                        _ => None,
                    })
                    .unwrap_or(VerticalAlignment::Center);
                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_hstack(child_nodes?,valign ))
            }

            "group" => {
                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                Ok(builder.new_group(child_nodes?))
            }

            "rect" => {
                let width_behavior = parse_unified_dimension(entity.width.as_ref());
                let height_behavior = parse_unified_dimension(entity.height.as_ref());

                let options = RectOptions {
                    width_behavior,
                    height_behavior,
                    fill_color: entity
                        .background
                        .as_ref()
                        .map(|bg| Fill::Color(bg.clone()))
                        .unwrap_or(Fill::Color("white".to_string())),
                    stroke_color: entity
                        .border_color
                        .clone()
                        .unwrap_or_else(|| "black".to_string()),
                    stroke_width: entity.border_width.unwrap_or(1.0),
                    ..Default::default()
                };

                Ok(builder.new_rectangle(options))
            }

            "line" => {
                let options = LineOptions {
                    stroke_color: entity
                        .stroke_color
                        .clone()
                        .unwrap_or_else(|| "black".to_string()),
                    stroke_width: entity.stroke_width.unwrap_or(1.0),
                    ..Default::default()
                };

                Ok(builder.new_line(entity.start_point.unwrap_or((0.0, 0.0)), entity.end_point.unwrap_or((0.0, 0.0)), options))
            }

            "ellipse" => {
                let center = entity.center.unwrap_or((50.0, 50.0));
                let radius = entity.radius.unwrap_or((25.0, 25.0));
                let options = EllipseOptions {
                    fill_color: entity
                        .background.clone()
                        .unwrap_or_else(|| "white".to_string()),
                    stroke_color: entity
                        .border_color
                        .clone()
                        .unwrap_or_else(|| "black".to_string()),
                    stroke_width: entity.border_width.unwrap_or(1.0),
                    ..Default::default()
                };

                Ok(builder.new_elipse(center, radius, options))
            }

            "image" => {

                let width_behavior = parse_unified_dimension(entity.width.as_ref());
                let height_behavior = parse_unified_dimension(entity.height.as_ref());

                if let Some(src) = &entity.src {
                    Ok(builder.new_image(src, (width_behavior, height_behavior)))
                } else if let Some(file_path) = &entity.file_path {
                    Ok(builder.new_image_from_file(file_path, (width_behavior, height_behavior)))
                } else {
                    Err(JsonLinesError::MissingAttribute(
                        "src or file_path".to_string(),
                    ))
                }
            }

            "table" => {
                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;
                let cols = entity
                    .cols
                    .ok_or_else(|| JsonLinesError::MissingAttribute("cols".to_string()))?;

                let child_nodes: Result<Vec<_>, _> = children
                    .iter()
                    .map(|child_id| self.build_entity(child_id, builder))
                    .collect();

                let options = TableOptions {
                    header_fill_color: entity
                        .header_fill_color
                        .clone()
                        .unwrap_or_else(|| "lightgray".to_string()),
                    ..Default::default()
                };

                Ok(builder.new_table(child_nodes?, cols, options))
            }

            "polyline" => {
                let points = entity
                    .points
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("points".to_string()))?;

                let options = LineOptions {
                    stroke_color: entity
                        .stroke_color
                        .clone()
                        .unwrap_or_else(|| "black".to_string()),
                    stroke_width: entity.stroke_width.unwrap_or(1.0),
                    ..Default::default()
                };

                Ok(builder.new_polyline(points.clone(), options))
            }

            "free_container" => {
                let children = entity
                    .children
                    .as_ref()
                    .ok_or_else(|| JsonLinesError::MissingAttribute("children".to_string()))?;

                let mut positioned_children = Vec::new();
                for child_id in children {
                    let child_entity = self
                        .entities
                        .get(child_id)
                        .ok_or_else(|| JsonLinesError::EntityNotFound(child_id.clone()))?;

                    let child_node = self.build_entity(child_id, builder)?;
                    let position = (child_entity.x.unwrap_or(0.0), child_entity.y.unwrap_or(0.0));
                    positioned_children.push((child_node, position));
                }

                Ok(builder.new_free_container_with_children(positioned_children))
            }

            _ => Err(JsonLinesError::UnknownEntityType(
                entity.entity_type.clone(),
            )),
        }
    }

    /// Validate that all child references exist
    pub fn validate(&self) -> Result<(), JsonLinesError> {
        for (id, entity) in &self.entities {
            if let Some(children) = &entity.children {
                for child_id in children {
                    if !self.entities.contains_key(child_id) {
                        return Err(JsonLinesError::MissingChild {
                            parent: id.clone(),
                            child: child_id.clone(),
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

impl Default for JsonEntity {
    fn default() -> Self {
        Self {
            id: String::new(),
            entity_type: String::new(),
            children: None,
            content: None,
            font_size: None,
            color: None,
            font_family: None,
            padding: None,
            background: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            width: None,
            height: None,
            x: None,
            y: None,
            src: None,
            file_path: None,
            cols: None,
            header_fill_color: None,
            stroke_color: None,
            stroke_width: None,
            start_point: None,
            end_point: None,
            points: None,
            center: None,
            radius: None,
            extra: Map::new(),
        }
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

    #[test]
    fn test_json_lines_parsing() {
        let input = r#"
{"id":"root","type":"box","padding":10,"background":"white","children":["text1"]}
{"id":"text1","type":"text","content":"Hello World","font_size":16,"color":"blue"}
"#;

        let mut parser = JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();
        assert_eq!(root_id, "root");

        parser.validate().unwrap();

        let mut builder = DiagramBuilder::new();
        let _diagram = parser.build(&root_id, &mut builder).unwrap();
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
}

// Example of what an LLM might generate
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
