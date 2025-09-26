// src/components/documents/mod.rs
// Document components with consistent styling through predefined constants

use serde_json::{Map, Value};
use crate::parser::{get_array_attr, get_bool_attr, get_float_attr, get_int_attr, get_string_attr, JsonLinesParser};
use crate::*;
use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use uuid::fmt::Simple;


/// Document Style Constants
/// These constants define a consistent design system for document components
pub mod style {
    use crate::Float;

    // === COLOR PALETTE ===
    
    // Primary Colors
    pub const PRIMARY_TEXT: &str = "#212529";      // Dark gray for main text
    pub const SECONDARY_TEXT: &str = "#495057";     // Medium gray for headings
    pub const MUTED_TEXT: &str = "#6c757d";        // Light gray for metadata
    pub const ACCENT_TEXT: &str = "#0d6efd";       // Blue for links/accents

    // Background Colors
    pub const BG_PRIMARY: &str = "white";          // Main content background
    pub const BG_SECONDARY: &str = "#f8f9fa";      // Header/sidebar background
    pub const BG_MUTED: &str = "#f1f3f4";         // Footer/disabled background
    pub const BG_ACCENT: &str = "#e3f2fd";        // Highlight background

    // Border Colors
    pub const BORDER_LIGHT_COLOR: &str = "#dee2e6";      // Light borders
    pub const BORDER_MEDIUM_COLOR: &str = "#adb5bd";     // Medium borders
    pub const BORDER_STRONG_COLOR: &str = "#6c757d";     // Strong borders

    // Status Colors
    pub const SUCCESS: &str = "#198754";
    pub const WARNING: &str = "#ffc107";
    pub const DANGER: &str = "#dc3545";
    pub const INFO: &str = "#0dcaf0";

    // === TYPOGRAPHY ===
    
    // Font Families
    pub const FONT_SERIF: &str = "Georgia";        // For titles and headers
    pub const FONT_SANS: &str = "Arial";           // For body text
    pub const FONT_MONO: &str = "Consolas";        // For code

    // Font Sizes
    pub const TEXT_XS: Float = 10.0;    // Footer text, captions
    pub const TEXT_SM: Float = 12.0;    // Small text, metadata
    pub const TEXT_BASE: Float = 14.0;  // Body text
    pub const TEXT_LG: Float = 16.0;    // Large body text
    pub const TEXT_XL: Float = 18.0;    // Section headings
    pub const TEXT_2XL: Float = 24.0;   // Page titles
    pub const TEXT_3XL: Float = 32.0;   // Main titles

    // Line Heights (as spacing multipliers)
    pub const LINE_HEIGHT_TIGHT: Float = 1.2;
    pub const LINE_HEIGHT_NORMAL: Float = 1.5;
    pub const LINE_HEIGHT_RELAXED: Float = 1.8;

    // === SPACING ===
    
    // Base spacing unit (all other spacing is multiples of this)
    pub const SPACE_UNIT: Float = 8.0;

    // Spacing Scale
    pub const SPACE_XS: Float = SPACE_UNIT * 0.5;  // 4px
    pub const SPACE_SM: Float = SPACE_UNIT * 1.0;  // 8px
    pub const SPACE_MD: Float = SPACE_UNIT * 2.0;  // 16px
    pub const SPACE_LG: Float = SPACE_UNIT * 3.0;  // 24px
    pub const SPACE_XL: Float = SPACE_UNIT * 4.0;  // 32px
    pub const SPACE_2XL: Float = SPACE_UNIT * 6.0; // 48px
    pub const SPACE_3XL: Float = SPACE_UNIT * 8.0; // 64px

    // Component-specific spacing
    pub const PADDING_TIGHT: Float = SPACE_SM;     // 8px
    pub const PADDING_NORMAL: Float = SPACE_MD;    // 16px
    pub const PADDING_RELAXED: Float = SPACE_LG;   // 24px
    pub const PADDING_LOOSE: Float = SPACE_XL;     // 32px

    // === DIMENSIONS ===
    
    // Standard widths
    pub const WIDTH_SM: Float = 480.0;     // Small documents
    pub const WIDTH_MD: Float = 640.0;     // Medium documents  
    pub const WIDTH_LG: Float = 800.0;     // Large documents
    pub const WIDTH_XL: Float = 1024.0;    // Extra large documents
    pub const WIDTH_FULL: Float = 1200.0;  // Full width documents

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
    pub const LINE_WIDTH_NARROW: usize = 300;   // Narrow columns
    pub const LINE_WIDTH_NORMAL: usize = 500;   // Standard paragraphs
    pub const LINE_WIDTH_WIDE: usize = 700;     // Wide content
    pub const LINE_WIDTH_FULL: usize = 900;     // Full width text

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
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser : &JsonLinesParser,
) -> Result<DiagramTreeNode, String> {

    let mut children = Vec::new();
    let header_id = get_string_attr(attrs, &["header_id"], "");
    let content_id = get_string_attr(attrs, &["content_id"], "");
    let footer_id = get_string_attr(attrs, &["footer_id"], "");

    let children_ids = vec![header_id, content_id, footer_id];
    children_ids.iter().for_each(|elem| {
        //TODO: if child is not present, do not fail
        if elem.len() > 0 {
            let child = parser.build(&elem, builder);
            if let Ok(c) = child {
                children.push(c);
            }
        }
    });


    let vstack = builder.new_vstack(format!("{}_document_container",
     get_string_attr(attrs, &["id"], "")), 
     children, HorizontalAlignment::Left);

     return Ok(vstack);

    
}

/// Register document components with a DiagramBuilder
pub fn register_document_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("document", create_document_container);
    println!("📄 Document component registered: 'document'");
}
