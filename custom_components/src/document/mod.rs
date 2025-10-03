// src/components/documents/mod.rs
// Document components with consistent styling through predefined constants

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

/// Document Style Constants
/// These constants define a consistent design system for document components
pub mod style {
    use crate::Float;

    // === COLOR PALETTE ===

    // Primary Colors
    pub const PRIMARY_TEXT: &str = "#212529"; // Dark gray for main text
    pub const SECONDARY_TEXT: &str = "#495057"; // Medium gray for headings
    pub const MUTED_TEXT: &str = "#6c757d"; // Light gray for metadata
    pub const ACCENT_TEXT: &str = "#0d6efd"; // Blue for links/accents

    // Background Colors
    pub const BG_PRIMARY: &str = "white"; // Main content background
    pub const BG_SECONDARY: &str = "#f8f9fa"; // Header/sidebar background
    pub const BG_MUTED: &str = "#f1f3f4"; // Footer/disabled background
    pub const BG_ACCENT: &str = "#e3f2fd"; // Highlight background

    // Border Colors
    pub const BORDER_LIGHT_COLOR: &str = "#dee2e6"; // Light borders
    pub const BORDER_MEDIUM_COLOR: &str = "#adb5bd"; // Medium borders
    pub const BORDER_STRONG_COLOR: &str = "#6c757d"; // Strong borders

    // Status Colors
    pub const SUCCESS: &str = "#198754";
    pub const WARNING: &str = "#ffc107";
    pub const DANGER: &str = "#dc3545";
    pub const INFO: &str = "#0dcaf0";

    // === TYPOGRAPHY ===

    // Font Families
    pub const FONT_SERIF: &str = "Georgia"; // For titles and headers
    pub const FONT_SANS: &str = "Arial"; // For body text
    pub const FONT_MONO: &str = "Consolas"; // For code

    // Font Sizes
    pub const TEXT_XS: Float = 10.0; // Footer text, captions
    pub const TEXT_SM: Float = 12.0; // Small text, metadata
    pub const TEXT_BASE: Float = 14.0; // Body text
    pub const TEXT_LG: Float = 16.0; // Large body text
    pub const TEXT_XL: Float = 18.0; // Section headings
    pub const TEXT_2XL: Float = 24.0; // Page titles
    pub const TEXT_3XL: Float = 32.0; // Main titles

    // Title font weights

    pub const FONT_WEIGHT_NORMAL: u32 = 400;
    pub const FONT_WEIGHT_BOLD_MAX: u32 = 900;
    pub const FONT_WEIGHT_BOLD_MD: u32 = 700;
    pub const FONT_WEIGHT_BOLD_LIGHT: u32 = 600;

    // Line Heights (as spacing multipliers)
    pub const LINE_HEIGHT_TIGHT: Float = 1.2;
    pub const LINE_HEIGHT_NORMAL: Float = 1.5;
    pub const LINE_HEIGHT_RELAXED: Float = 2.0;

    // === SPACING ===

    // Base spacing unit (all other spacing is multiples of this)
    pub const SPACE_UNIT: Float = 8.0;

    // Spacing Scale
    pub const SPACE_XS: Float = SPACE_UNIT * 0.5; // 4px
    pub const SPACE_SM: Float = SPACE_UNIT * 1.0; // 8px
    pub const SPACE_MD: Float = SPACE_UNIT * 2.0; // 16px
    pub const SPACE_LG: Float = SPACE_UNIT * 3.0; // 24px
    pub const SPACE_XL: Float = SPACE_UNIT * 4.0; // 32px
    pub const SPACE_2XL: Float = SPACE_UNIT * 6.0; // 48px
    pub const SPACE_3XL: Float = SPACE_UNIT * 8.0; // 64px

    // Component-specific spacing
    pub const PADDING_TIGHT: Float = SPACE_SM; // 8px
    pub const PADDING_NORMAL: Float = SPACE_MD; // 16px
    pub const PADDING_RELAXED: Float = SPACE_LG; // 24px
    pub const PADDING_LOOSE: Float = SPACE_XL; // 32px

    // === DIMENSIONS ===

    // Standard widths
    pub const WIDTH_SM: Float = 480.0; // Small documents
    pub const WIDTH_MD: Float = 640.0; // Medium documents
    pub const WIDTH_LG: Float = 800.0; // Large documents
    pub const WIDTH_XL: Float = 1024.0; // Extra large documents
    pub const WIDTH_FULL: Float = 1200.0; // Full width documents
    pub const WIDTH_PROPERTY_PANEL: Float = 85.0; // For property field names/values

    // Border radius
    pub const RADIUS_SM: Float = 4.0;
    pub const RADIUS_MD: Float = 8.0;
    pub const RADIUS_LG: Float = 12.0;
    pub const RADIUS_PILL: Float = 9999.0; // For pill-shaped elements

    // Border widths
    pub const BORDER_THIN: Float = 1.0;
    pub const BORDER_MEDIUM: Float = 2.0;
    pub const BORDER_THICK: Float = 4.0;

    // === LINE WIDTHS (for text wrapping) ===
    pub const LINE_WIDTH_NARROW: usize = 300; // Narrow columns
    pub const LINE_WIDTH_NORMAL: usize = 500; // Standard paragraphs
    pub const LINE_WIDTH_WIDE: usize = 700; // Wide content
    pub const LINE_WIDTH_FULL: usize = 900; // Full width text

    // === COMPONENT PRESETS ===

    // Document container presets
    pub const DOCUMENT_WIDTH_DEFAULT: Float = WIDTH_LG;
    pub const DOCUMENT_PADDING_DEFAULT: Float = PADDING_LOOSE;
    pub const DOCUMENT_BORDER_RADIUS: Float = RADIUS_MD;

    // Header presets
    pub const HEADER_PADDING: Float = PADDING_RELAXED;
    pub const HEADER_TITLE_SIZE: Float = TEXT_2XL;
    pub const HEADER_SUBTITLE_SIZE: Float = TEXT_BASE;

    // Content presets
    pub const CONTENT_PADDING: Float = PADDING_LOOSE;
    pub const CONTENT_LINE_WIDTH: usize = LINE_WIDTH_WIDE;

    // Footer presets
    pub const FOOTER_PADDING: Float = PADDING_NORMAL;
    pub const FOOTER_TEXT_SIZE: Float = TEXT_XS;
}

/// Theme Constants
/// Higher-level semantic constants for different component themes
pub mod theme {
    use super::style::*;

    // Document theme
    pub const DOCUMENT_HEADER_BG: &str = BG_SECONDARY;
    pub const DOCUMENT_CONTENT_BG: &str = BG_PRIMARY;
    pub const DOCUMENT_FOOTER_BG: &str = BG_MUTED;
    pub const DOCUMENT_BORDER: &str = BORDER_LIGHT_COLOR;

    // Text themes
    pub const TITLE_COLOR: &str = PRIMARY_TEXT;
    pub const SUBTITLE_COLOR: &str = SECONDARY_TEXT;
    pub const BODY_COLOR: &str = PRIMARY_TEXT;
    pub const META_COLOR: &str = MUTED_TEXT;
    pub const LINK_COLOR: &str = ACCENT_TEXT;

    // Component themes
    pub const CARD_BG: &str = BG_PRIMARY;
    pub const CARD_BORDER: &str = BORDER_LIGHT_COLOR;
    pub const CARD_SHADOW: bool = true;

    pub const ALERT_SUCCESS_BG: &str = "#d1e7dd";
    pub const ALERT_SUCCESS_BORDER: &str = SUCCESS;
    pub const ALERT_WARNING_BG: &str = "#fff3cd";
    pub const ALERT_WARNING_BORDER: &str = WARNING;
    pub const ALERT_DANGER_BG: &str = "#f8d7da";
    pub const ALERT_DANGER_BORDER: &str = DANGER;
}

pub fn create_document_container(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let mut children = Vec::new();
    let header_id = get_string_attr(attrs, &["header_id"], "");
    let content_id = get_string_attr(attrs, &["content_id"], "");
    let footer_id = get_string_attr(attrs, &["footer_id"], "");
    println!("create_document_container id {}", id);
    eprintln!("Building header: {}", header_id);
    // let newparser = JsonLinesParser::new();
    if let Ok(header_child) = parser.build(&header_id, builder) {
        let mut header_options = BoxOptions::new();
        header_options.padding = PADDING_NORMAL;
        header_options.fill_color = Fill::Color(BG_PRIMARY.to_string());
        header_options.stroke_width = 0.0;
        header_options.stroke_color = BG_PRIMARY.to_string();

        let header_container = builder.new_box(
            format!("{}_header_container", &id),
            header_child,
            header_options,
        );

        children.push(header_container);
    }

    if let Ok(content_child) = parser.build(&content_id, builder) {
        let mut content_options = BoxOptions::new();
        content_options.padding = PADDING_NORMAL;
        content_options.fill_color = Fill::Color(BG_PRIMARY.to_string());
        content_options.stroke_width = 0.0;
        content_options.stroke_color = BG_PRIMARY.to_string();

        let content_container = builder.new_box(
            format!("{}_content_container", &id),
            content_child,
            content_options,
        );
        children.push(content_container);
    }

    if let Ok(footer_child) = parser.build(&footer_id, builder) {
        let mut footer_options = BoxOptions::new();
        footer_options.padding = PADDING_NORMAL;
        footer_options.fill_color = Fill::Color(BG_PRIMARY.to_string());
        footer_options.stroke_width = 0.0;
        footer_options.stroke_color = BG_PRIMARY.to_string();

        let footer_container = builder.new_box(
            format!("{}_footer_container", &id),
            footer_child,
            footer_options,
        );
        children.push(footer_container);
    }

    let vstack = builder.new_vstack(id.to_string(), children, HorizontalAlignment::Left);

    return Ok(vstack);
}

/**
 * Element for presenting text with different variants
 *
 * variants:
 * default: Standard content blocks
 * large: Hero sections or main headings
 * small: Compact spaces or secondary information
 * subtle: Labels or supporting content
 * emphasized: Important callouts or quotes
 *
 * width attribute: |sm|md|lg|xl|full|<number>
 */
pub fn create_document_text(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    attrs.clone().iter().for_each(|(k, v)| {
        println!("k,v: {},{}", k, v);
    });
    let variant = get_string_attr(attrs, &["variant"], "default");
    let content = get_string_attr(attrs, &["text", "content"], "");
    let max_width = get_width(attrs, &["width"], WIDTH_SM);

    document_text(id, builder, variant, content, max_width)
}

pub fn document_text(
    id: &str,
    builder: &mut DiagramBuilder,
    variant: String,
    content: String,
    max_width: f32,
) -> Result<DiagramTreeNode, String> {
    let mut toptions = match variant.as_str() {
        "xlarge" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_XL,
            text_color: PRIMARY_TEXT.to_string(),
            ..TextOptions::default()
        },
        "large" | "emphasized" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_LG,
            text_color: PRIMARY_TEXT.to_string(),
            ..TextOptions::default()
        },

        "small" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_XS,
            text_color: PRIMARY_TEXT.to_string(),
            ..TextOptions::default()
        },

        "subtle" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_XS,
            text_color: SECONDARY_TEXT.to_string(),
            ..TextOptions::default()
        },

        _ => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_BASE,
            text_color: PRIMARY_TEXT.to_string(),
            ..TextOptions::default()
        },
    };

    // we need to calculate the line width for the max width

    toptions.line_width = calculate_optimal_line_width(&builder, &content, &toptions, max_width);

    toptions.line_spacing = toptions.font_size * 0.4;

    let text = builder.new_text(format!("{}_text", id.clone()), content.as_str(), toptions);

    let coptions = BoxOptions {
        fill_color: Fill::Color("transparent".to_string()),
        stroke_color: "transparent".to_string(),
        stroke_width: 0.0,
        padding: 0.0,
        border_radius: 0.0,
        width_behavior: SizeBehavior::Content,
        height_behavior: SizeBehavior::Content,
        horizontal_alignment: HorizontalAlignment::Left,
    };
    let container = builder.new_box(format!("{}_text_container", id), text, coptions);

    // if variant is emphasized add left line
    if variant == "emphasized" {
        let left_line = builder.new_rectangle(
            format!("{}_left_line", id),
            RectOptions {
                stroke_width: 1.0,
                width_behavior: SizeBehavior::Fixed(1.0),
                stroke_color: PRIMARY_TEXT.to_owned(),
                fill_color: Fill::Color(PRIMARY_TEXT.to_owned()),
                ..Default::default()
            },
        );
        let constraints = vec![
            SimpleConstraint::SameHeight(vec![
                container.entity_id.clone(),
                left_line.entity_id.clone(),
            ]),
            SimpleConstraint::LeftOf(left_line.entity_id.clone(), container.entity_id.clone()),
            SimpleConstraint::HorizontalSpacing(
                left_line.entity_id.clone(),
                container.entity_id.clone(),
                SPACE_SM,
            ),
        ];
        let e_container = builder.new_constraint_layout_container(
            format!("{}_emp_container", id),
            vec![(container, None), (left_line, None)],
            constraints,
        );
        return Ok(e_container);
    }

    Ok(container)
}

/**
 * Custom component for creating Titles
 */
pub fn create_document_title(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let variant = get_string_attr(attrs, &["variant"], "default");
    let content = get_string_attr(attrs, &["text", "content"], "");
    let container_width = get_width(attrs, &["width"], WIDTH_MD);
    document_title(id, builder, variant, content, container_width)
}

fn document_title(
    id: &str,
    builder: &mut DiagramBuilder,
    variant: String,
    content: String,
    container_width: f32,
) -> Result<DiagramTreeNode, String> {
    let mut toptions = match variant.as_str() {
        "h1" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_3XL,
            text_color: PRIMARY_TEXT.to_string(),
            line_spacing: TEXT_3XL * 0.1,
            font_weight: FONT_WEIGHT_BOLD_MAX,
            ..TextOptions::default()
        },
        "h2" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_2XL,
            text_color: PRIMARY_TEXT.to_string(),
            line_spacing: TEXT_2XL * 0.4,
            font_weight: FONT_WEIGHT_BOLD_MAX,
            ..TextOptions::default()
        },

        "h3" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_XL,
            text_color: PRIMARY_TEXT.to_string(),
            line_spacing: TEXT_XL * 0.5,
            font_weight: FONT_WEIGHT_BOLD_MD,
            ..TextOptions::default()
        },

        "h4" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_LG,
            text_color: SECONDARY_TEXT.to_string(),
            line_spacing: TEXT_LG * 0.5,
            font_weight: FONT_WEIGHT_BOLD_LIGHT,
            ..TextOptions::default()
        },

        "h5" => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_BASE,
            text_color: SECONDARY_TEXT.to_string(),
            line_spacing: TEXT_BASE * 0.1,
            font_weight: FONT_WEIGHT_BOLD_LIGHT,
            ..TextOptions::default()
        },

        _ => TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_2XL,
            text_color: PRIMARY_TEXT.to_string(),
            line_spacing: TEXT_2XL * 0.5,
            font_weight: FONT_WEIGHT_BOLD_LIGHT,
            ..TextOptions::default()
        },
    };

    toptions.line_width =
        calculate_optimal_line_width(&builder, &content, &toptions, container_width);

    let bottom_margin = toptions.line_spacing;
    let text = builder.new_text(format!("{}_text", id), content.as_str(), toptions);
    let w_size = SizeBehavior::Content;
    let coptions = BoxOptions {
        fill_color: Fill::Color("transparent".to_string()),
        stroke_color: "transparent".to_string(),
        stroke_width: 0.0,
        padding: 0.0,
        border_radius: 0.0,
        width_behavior: w_size,
        height_behavior: SizeBehavior::Content,
        horizontal_alignment: HorizontalAlignment::Left,
    };
    let spacer = builder.new_spacer(
        format!("{}_spacer", id),
        SpacerOptions {
            width: 0.0,
            height: bottom_margin,
            direction: SpacerDirection::Vertical,
        },
    );
    let t_container = builder.new_box(format!("{}_text_container", id), text.clone(), coptions);
    let container = builder.new_vstack(
        format!("{}_container", id),
        vec![t_container, spacer],
        HorizontalAlignment::Left,
    );
    Ok(container)
}

/**
 * Creates a vstack with horizontal alignment set to left
 * and spacers between elements for better layout.
 */
pub fn create_vstack(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let children_ids = get_array_attr(attrs, "children").or(Some([].to_vec()));

    let mut children = Vec::<DiagramTreeNode>::new();
    for (_, elem) in children_ids.unwrap().iter().enumerate() {
        let child_elem = parser.build(elem.as_str(), builder).ok();
        if let Some(elem) = child_elem {
            children.push(elem);
        }
    }

    vstack(id, builder, parser, children)
}

/**
 * Creates a vstack and adds spacers between all elements
 */
fn vstack(
    id: &str,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
    children: Vec<DiagramTreeNode>,
) -> Result<DiagramTreeNode, String> {
    let mut final_children = Vec::<DiagramTreeNode>::new();

    for (ix, elem) in children.iter().enumerate() {
        final_children.push(elem.clone());
        //add spacer
        let spacer_id = format!("{}_spacer_{}", id, ix);
        let spacer = builder.new_spacer(
            spacer_id,
            SpacerOptions {
                width: 0.0,
                height: SPACE_SM,
                direction: SpacerDirection::Vertical,
            },
        );
        final_children.push(spacer);
    }

    Ok(builder.new_vstack(id.to_string(), final_children, HorizontalAlignment::Left))
}


/**
 * Creates a vstack with horizontal alignment set to left
 * and spacers between elements for better layout.
 */
pub fn create_hstack(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let children_ids = get_array_attr(attrs, "children").or(Some([].to_vec()));

    let mut children = Vec::<DiagramTreeNode>::new();
    for (_, elem) in children_ids.unwrap().iter().enumerate() {
        let child_elem = parser.build(elem.as_str(), builder).ok();
        if let Some(elem) = child_elem {
            children.push(elem);
        }
    }

    hstack(id, builder, parser, children)
}

fn hstack(
    id: &str,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
    children: Vec<DiagramTreeNode>,
) -> Result<DiagramTreeNode, String> {
    let mut final_children = Vec::<DiagramTreeNode>::new();

    for (ix, elem) in children.iter().enumerate() {
        final_children.push(elem.clone());
        //add spacer
        let spacer_id = format!("{}_spacer_{}", id, ix);
        let spacer = builder.new_spacer(
            spacer_id,
            SpacerOptions {
                width: SPACE_SM,
                height: 0.0,
                direction: SpacerDirection::Horizontal,
            },
        );
        final_children.push(spacer);
    }

    Ok(builder.new_hstack(id.to_string(), final_children, VerticalAlignment::Top))
}

/**
 * Component useful for presenting a list of name/value property list
 */
pub fn create_properties(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let properties = get_properties(attrs, &["properties", "items"]);
    let title = get_string_attr(attrs, &["title","meta"], "");

    let table_opts = TableOptions {
        cell_padding: SPACE_SM,
        with_header: false,
        border_color: BORDER_LIGHT_COLOR.to_string(),
        border_width: 1,
        fill_color: BG_SECONDARY.to_string(),
        ..Default::default()
    };

    // Crear las celdas a partir de las propiedades Vec<(String,String)>
    let cell_values: Vec<String> = properties.into_iter().flat_map(|(a, b)| [a, b]).collect();

    let cell_texts: Vec<DiagramTreeNode> = cell_values
        .into_iter()
        .map(|value| {
            // TODO: variant debe ser enum
            // TODO: en vez de uuid derivar el id del nombre de la propiedad si es posible
            // TODO: usar width diferente para nombre y value
            document_text(
                format!("{}_prop_{}", id, uuid::Uuid::new_v4()).as_str(),
                builder,
                "small".into(),
                value,
                WIDTH_PROPERTY_PANEL,
            )
        })
        .filter_map(|v| v.ok())
        .collect();


    let table_id = format!("{}_table", id);
    let table = builder.new_table(table_id.clone(), cell_texts, 2, table_opts);
    if title.len() == 0 {
        return Ok(table);
    } else {
        let title_id = format!("{}_title", id);
        // Return a document.vstack with the title and the table
        let title = document_text(
            title_id.as_str(),
            builder,
            // TODO: usar enum
            "meta".to_string(),
            title,
            WIDTH_PROPERTY_PANEL,
        );
        //TODO: revisar uso de unwrap aqui, 
        let stack_children = vec![title,Ok(table)].iter().filter_map(|e| e.clone().ok()).collect();
        return vstack(id, builder, parser, stack_children);
    }
}

/**
 * Used for the property panels, each item is expected to have two values, name and value
 * e.g items=[["name","value"], ["name", "value"]...]
 */
pub fn get_properties(attrs: &Map<String, Value>, keys: &[&str]) -> Vec<(String, String)> {
    let mut items = Vec::<(String, String)>::new();
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(arr) = value.as_array() {
                for item_arr_val in arr {
                    if let Some(item_arr) = item_arr_val.as_array() {
                        if item_arr.len() == 2 {
                            let name_elem = &item_arr[0];
                            let value_elem = &item_arr[1];
                            let name = name_elem.as_str().unwrap();
                            let value = value_elem.as_str().unwrap();
                            items.push((name.to_owned(), value.to_owned()));
                        }
                    }
                }
            }
        }
    }
    items
}

// Helper function for extracting a text width from names or pixel value
// |sm|md|lg|xl|full|<number>
pub fn get_width(attrs: &Map<String, Value>, keys: &[&str], default: Float) -> Float {
    for key in keys {
        if let Some(value) = attrs.get(*key) {
            if let Some(s) = value.as_str() {
                return if s.is_empty() {
                    default
                } else {
                    match s {
                        "sm" => WIDTH_SM,
                        "md" => WIDTH_MD,
                        "lg" => WIDTH_LG,
                        "xl" => WIDTH_XL,
                        "full" => WIDTH_FULL,
                        _ => {
                            if let Ok(val) = s.parse::<f32>() {
                                val
                            } else {
                                WIDTH_MD
                            }
                        }
                    }
                };
            }
        }
    }
    default
}
/**
 * Document Section Component
 *
 * Creates a section with optional title, meta information, and multi-column layout.
 *
 * Attributes:
 * - title (string, optional): The section title
 * - meta (string, optional): Metadata/category information
 * - columns (array of ids): Column content element IDs
 *
 * Example JSONL:
 * {"id":"section-example", "type":"document.section", "title":"Section Title", "meta":"Design Theory - 2024", "columns":["col1","col2","col3"]}
 */
pub fn create_document_section(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let mut section_children = Vec::new();

    // Extract attributes
    let title = get_string_attr(attrs, &["title"], "");
    let meta = get_string_attr(attrs, &["meta"], "");
    let columns = get_array_attr(attrs, "columns");
    let witdth = get_width(attrs, &["width"], WIDTH_MD);

    // Create header if title or meta is present
    if !title.is_empty() || !meta.is_empty() {
        let header_node = create_section_header(id, &title, &meta, builder, parser, witdth)?;
        section_children.push(header_node);
        let header_spacer_opts = SpacerOptions {
            width: 0.0,
            height: SPACE_MD,
            direction: SpacerDirection::Vertical,
        };
        let header_spacer = builder.new_spacer(format!("{}_header_spacer", id), header_spacer_opts);
        section_children.push(header_spacer);
    }

    // Create columns layout if columns are provided
    if let Some(column_ids) = columns {
        if !column_ids.is_empty() {
            let columns_node = create_columns_layout(id, &column_ids, builder, parser)?;
            section_children.push(columns_node);
        }
    }

    // Wrap everything in a vertical stack
    let section_vstack = builder.new_vstack(
        format!("{}_section", id),
        section_children,
        HorizontalAlignment::Left,
    );

    // Add spacing after entire section
    let section_spacer = builder.new_spacer(
        format!("{}_bottom_spacer", id),
        SpacerOptions {
            width: 0.0,
            height: SPACE_XS,
            direction: SpacerDirection::Vertical,
        },
    );

    let section_with_spacing = builder.new_vstack(
        id.to_string(),
        vec![section_vstack, section_spacer],
        HorizontalAlignment::Left,
    );

    Ok(section_with_spacing)
}

/**
 * Creates the header for a section with title and optional meta information
 * Uses the document.title component for consistent styling
 */
fn create_section_header(
    id: &str,
    title: &str,
    meta: &str,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
    width: Float,
) -> Result<DiagramTreeNode, String> {
    let mut header_children = Vec::new();

    // Create title if present using document.title component
    if !title.is_empty() {
        // Create attributes map for the title
        //TODO : remplazar esto, usar funcion directa que no requiera mapa de atributos
        let mut title_attrs = serde_json::Map::new();
        title_attrs.insert(
            "text".to_string(),
            serde_json::Value::String(title.to_string()),
        );
        title_attrs.insert(
            "variant".to_string(),
            serde_json::Value::String("h2".to_string()),
        );
        title_attrs.insert(
            "width".to_string(),
            serde_json::Value::String(width.to_string()),
        );

        // Use the create_title function
        let title_node =
            create_document_title(&format!("{}_title", id), &title_attrs, builder, parser)?;

        header_children.push(title_node);
    }

    // Create meta text if present
    if !meta.is_empty() {
        // Add small spacing between title and meta
        if !title.is_empty() {
            let meta_spacer = builder.new_spacer(
                format!("{}_meta_spacer", id),
                SpacerOptions {
                    width: 0.0,
                    height: SPACE_XS,
                    direction: SpacerDirection::Vertical,
                },
            );
            header_children.push(meta_spacer);
        }

        let meta_options = TextOptions {
            font_family: FONT_SANS.to_string(),
            font_size: TEXT_XS,
            text_color: MUTED_TEXT.to_string(),
            line_width: 100,
            line_spacing: LINE_HEIGHT_NORMAL,
            font_weight: 400,
        };

        let meta_node = builder.new_text(format!("{}_meta", id), meta, meta_options);

        header_children.push(meta_node);
    }

    // Return vertical stack with header elements
    let header_vstack = builder.new_vstack(
        format!("{}_header", id),
        header_children,
        HorizontalAlignment::Left,
    );

    Ok(header_vstack)
}

/**
 * Creates a horizontal layout for columns
 */
fn create_columns_layout(
    id: &str,
    column_ids: &[String],
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    let mut column_nodes = Vec::new();

    for (idx, column_id) in column_ids.iter().enumerate() {
        // Build the column content
        match parser.build(column_id, builder) {
            Ok(column_node) => {
                // Wrap each column in a box for consistent spacing and layout
                let column_box_options = BoxOptions {
                    fill_color: Fill::Color("transparent".to_string()),
                    stroke_color: "transparent".to_string(),
                    stroke_width: 0.0,
                    padding: 0.0,
                    border_radius: 0.0,
                    width_behavior: SizeBehavior::Content, // Size based on column content
                    height_behavior: SizeBehavior::Content,
                    horizontal_alignment: HorizontalAlignment::Left,
                };

                let column_box = builder.new_box(
                    format!("{}_col_{}", id, idx),
                    column_node,
                    column_box_options,
                );

                column_nodes.push(column_box);
                if idx < column_ids.len() - 1 {
                    // Add some spacing between columns
                    let col_spacer_opts = SpacerOptions {
                        width: SPACE_MD,
                        height: 0.0,
                        direction: SpacerDirection::Horizontal,
                    };
                    let col_spacer =
                        builder.new_spacer(format!("{}_{}_col_spacer", id, idx), col_spacer_opts);
                    column_nodes.push(col_spacer);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to build column '{}': {}", column_id, e);
                // Continue with other columns even if one fails
            }
        }
    }

    if column_nodes.is_empty() {
        return Err("No valid columns found in section".to_string());
    }

    // Create horizontal stack for columns
    let columns_hstack = builder.new_hstack(
        format!("{}_columns", id),
        column_nodes,
        VerticalAlignment::Top, // Align columns to top
    );

    Ok(columns_hstack)
}

/**
 * Bullet List Component
 *
 * Creates a bulleted list with good typography and spacing
 *
 * Attributes:
 * - items (array of strings): List items to display
 * - width (optional): Width constraint (sm|md|lg|xl|full|number)
 *
 * Example JSONL:
 * {"id":"my-list", "type":"bullet-list", "items":["First item","Second item","Third item"]}
 */
pub fn create_bullet_list(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {
    // Extract items array
    let items = get_array_attr(attrs, "items");

    if items.is_none() {
        return Err("bullet-list requires 'items' attribute with at least one item".to_string());
    }
    let items = items.unwrap();

    let container_width = get_width(attrs, &["width"], WIDTH_MD);

    // Create list items
    let mut list_children = Vec::new();

    for (idx, item_text) in items.iter().enumerate() {
        let item_node = create_list_item(
            &format!("{}_item_{}", id, idx),
            item_text,
            "•", // Bullet character
            builder,
            container_width,
        )?;

        list_children.push(item_node);

        // Add spacing between items (except after last item)
        if idx < items.len() - 1 {
            let item_spacer = builder.new_spacer(
                format!("{}_item_spacer_{}", id, idx),
                SpacerOptions {
                    width: 0.0,
                    height: SPACE_SM,
                    direction: SpacerDirection::Vertical,
                },
            );
            list_children.push(item_spacer);
        }
    }

    // Wrap all items in a vertical stack
    let list_vstack = builder.new_vstack(
        format!("{}_list", id),
        list_children,
        HorizontalAlignment::Left,
    );

    // Add spacing after the entire list
    let bottom_spacer = builder.new_spacer(
        format!("{}_bottom_spacer", id),
        SpacerOptions {
            width: 0.0,
            height: SPACE_MD,
            direction: SpacerDirection::Vertical,
        },
    );

    let list_with_spacing = builder.new_vstack(
        id.to_string(),
        vec![list_vstack, bottom_spacer],
        HorizontalAlignment::Left,
    );

    Ok(list_with_spacing)
}

/**
 * Creates a single list item with bullet/number and text
 */
fn create_list_item(
    id: &str,
    text: &str,
    marker: &str,
    builder: &mut DiagramBuilder,
    container_width: Float,
) -> Result<DiagramTreeNode, String> {
    // Create bullet/marker
    let marker_options = TextOptions {
        font_family: FONT_SANS.to_string(),
        font_size: TEXT_BASE,
        text_color: PRIMARY_TEXT.to_string(),
        line_width: 20,
        line_spacing: TEXT_BASE * 0.4,
        font_weight: FONT_WEIGHT_NORMAL,
    };

    let marker_node = builder.new_text(format!("{}_marker", id), marker, marker_options);

    // Create text content
    let mut text_options = TextOptions {
        font_family: FONT_SANS.to_string(),
        font_size: TEXT_BASE,
        text_color: PRIMARY_TEXT.to_string(),
        line_spacing: TEXT_BASE * 0.4,
        font_weight: FONT_WEIGHT_NORMAL,
        ..Default::default()
    };

    text_options.line_width =
        calculate_optimal_line_width(&builder, text, &text_options, container_width);

    let text_node = builder.new_text(format!("{}_text", id), text, text_options);

    // Wrap text in a box to control width
    let text_box_options = BoxOptions {
        fill_color: Fill::Color("transparent".to_string()),
        stroke_color: "transparent".to_string(),
        stroke_width: 0.0,
        padding: 0.0,
        border_radius: 0.0,
        width_behavior: SizeBehavior::Content,
        height_behavior: SizeBehavior::Content,
        horizontal_alignment: HorizontalAlignment::Left,
    };

    let text_box = builder.new_box(format!("{}_text_box", id), text_node, text_box_options);

    let spacer = builder.new_spacer(
        format!("{}_spacer", id),
        SpacerOptions {
            width: SPACE_XS,
            height: 0.0,
            direction: SpacerDirection::Horizontal,
        },
    );

    // Create horizontal stack with marker and text
    let item_hstack = builder.new_hstack(
        format!("{}_hstack", id),
        vec![marker_node, spacer, text_box],
        VerticalAlignment::Top,
    );

    Ok(item_hstack)
}

pub fn register_document_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("document", create_document_container);
    builder.register_custom_component("document.hstack", create_hstack);
    builder.register_custom_component("document.vstack", create_vstack);
    builder.register_custom_component("document.text", create_document_text);
    builder.register_custom_component("document.title", create_document_title);
    builder.register_custom_component("document.properties", create_properties);
    builder.register_custom_component("document.section", create_document_section);
    builder.register_custom_component("document.bullet_list", create_bullet_list);
    println!("📄 Document component registered: 'document'");
}
