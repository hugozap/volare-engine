
use std::fmt::format;

use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use crate::document::style::{
    BG_ACCENT, BG_PRIMARY, BG_SECONDARY, BORDER_LIGHT_COLOR, DOCUMENT_WIDTH_DEFAULT, FONT_SANS,
    FONT_WEIGHT_BOLD_LIGHT, FONT_WEIGHT_BOLD_MAX, FONT_WEIGHT_BOLD_MD, FONT_WEIGHT_NORMAL,
    LINE_HEIGHT_NORMAL, LINE_HEIGHT_RELAXED, LINE_HEIGHT_TIGHT, MUTED_TEXT, PADDING_NORMAL,
    PRIMARY_TEXT, SECONDARY_TEXT, SPACE_3XL, SPACE_MD, SPACE_SM, SPACE_XS, TEXT_2XL, TEXT_3XL,
    TEXT_BASE, TEXT_LG, TEXT_XL, TEXT_XS, WIDTH_FULL, WIDTH_LG, WIDTH_MD, WIDTH_PROPERTY_PANEL,
    WIDTH_SM, WIDTH_XL,
};
use crate::document::theme::BODY_COLOR;
use crate::parser::{
    get_array_attr, get_bool_attr, get_float_attr, get_int_attr, get_string_attr, JsonLinesParser,
};
use crate::*;
use serde_json::{from_value, Map, Value};
use uuid::fmt::Simple;
use uuid::uuid;
use anyhow::{bail, Result};

pub fn create_ishikawa(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode> {

    let test = builder.new_text(id.to_string(), "ishikawa placeholder", TextOptions::new());
    return Ok(test);
}

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    println!("📄 Diagram components registered");
}