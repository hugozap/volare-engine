// components/infographics/mod.rs
// Complete implementation of infographic-specific components for Volare Engine

use serde_json::{Map, Value};
use volare_engine_layout::*;

// Helper functions for attribute extraction
fn get_string_attr(attrs: &Map<String, Value>, key: &str, default: &str) -> String {
    attrs
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

fn get_float_attr(attrs: &Map<String, Value>, key: &str, default: f64) -> Float {
    attrs.get(key).and_then(|v| v.as_f64()).unwrap_or(default) as Float
}

fn get_bool_attr(attrs: &Map<String, Value>, key: &str, default: bool) -> bool {
    attrs.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

fn get_int_attr(attrs: &Map<String, Value>, key: &str, default: i64) -> i64 {
    attrs.get(key).and_then(|v| v.as_i64()).unwrap_or(default)
}

fn get_array_attr(attrs: &Map<String, Value>, key: &str) -> Vec<Value> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

fn generate_unique_id(base: &str) -> String {
    format!(
        "{}_{}",
        base,
        uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string()
    )
}

// ============================================================================
// DATA VISUALIZATION COMPONENTS
// ============================================================================

/// Stat Card Component - Displays a key metric with optional trend indicator
/// Example: {"type":"stat_card","title":"Revenue","value":"$2.4M","change":"+12%","trend":"up","icon":"💰"}
pub fn create_stat_card_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("stat_card"));
    let title = get_string_attr(attrs, "title", "Metric");
    let value = get_string_attr(attrs, "value", "0");
    let change = get_string_attr(attrs, "change", "");
    let trend = get_string_attr(attrs, "trend", "neutral");
    let color = get_string_attr(attrs, "color", "#007bff");
    let icon = get_string_attr(attrs, "icon", "📊");
    let width = get_float_attr(attrs, "width", 200.0);

    // Create icon
    let icon_text = builder.new_text(
        format!("{}_icon", id),
        &icon,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 24.0,
            text_color: color.clone(),
            line_width: 50,
            line_spacing: 0.0,
        },
    );

    // Create value (large number)
    let value_text = builder.new_text(
        format!("{}_value", id),
        &value,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 32.0,
            text_color: "#333333".to_string(),
            line_width: (width - 40.0) as usize,
            line_spacing: 0.0,
        },
    );

    // Create title
    let title_text = builder.new_text(
        format!("{}_title", id),
        &title,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 14.0,
            text_color: "#666666".to_string(),
            line_width: (width - 40.0) as usize,
            line_spacing: 0.0,
        },
    );

    // Create layout
    let mut children = vec![icon_text, value_text, title_text];

    // Add change indicator if provided
    if !change.is_empty() {
        let trend_color = match trend.as_str() {
            "up" => "#28a745",
            "down" => "#dc3545",
            _ => "#6c757d",
        };

        let trend_symbol = match trend.as_str() {
            "up" => "↗",
            "down" => "↘",
            _ => "→",
        };

        let change_text = builder.new_text(
            format!("{}_change", id),
            &format!("{} {}", trend_symbol, change),
            TextOptions {
                font_family: "Arial".to_string(),
                font_size: 12.0,
                text_color: trend_color.to_string(),
                line_width: (width - 40.0) as usize,
                line_spacing: 0.0,
            },
        );
        children.push(change_text);
    }

    // Layout vertically
    let content = builder.new_vstack(
        format!("{}_content", id),
        children,
        HorizontalAlignment::Center,
    );

    // Wrap in styled box
    Ok(builder.new_box(
        id,
        content,
        BoxOptions {
            fill_color: Fill::Color("#ffffff".to_string()),
            stroke_color: "#e0e0e0".to_string(),
            stroke_width: 1.0,
            padding: 20.0,
            border_radius: 8.0,
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Content,
        },
    ))
}

/// Progress Bar Component - Shows progress towards a goal
/// Example: {"type":"progress_bar","label":"Goal Progress","value":75,"max":100,"color":"blue"}
pub fn create_progress_bar_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("progress_bar"));
    let label = get_string_attr(attrs, "label", "Progress");
    let value = get_float_attr(attrs, "value", 50.0);
    let max_value = get_float_attr(attrs, "max", 100.0);
    let color = get_string_attr(attrs, "color", "#007bff");
    let width = get_float_attr(attrs, "width", 300.0);
    let height = get_float_attr(attrs, "height", 20.0);
    let show_percentage = get_bool_attr(attrs, "show_percentage", true);

    // Calculate percentage
    let percentage = (value / max_value * 100.0).min(100.0).max(0.0);
    let fill_width = width * (percentage / 100.0);

    // Create label
    let label_text = builder.new_text(
        format!("{}_label", id),
        &label,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 14.0,
            text_color: "#333333".to_string(),
            line_width: width as usize,
            line_spacing: 0.0,
        },
    );

    // Create background bar
    let bg_bar = builder.new_rectangle(
        format!("{}_bg", id),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Fixed(height),
            fill_color: Fill::Color("#e9ecef".to_string()),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            border_radius: height / 2.0,
        },
    );

    // Create progress fill
    let progress_fill = builder.new_rectangle(
        format!("{}_fill", id),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(fill_width),
            height_behavior: SizeBehavior::Fixed(height),
            fill_color: Fill::Color(color),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            border_radius: height / 2.0,
        },
    );

    // Create elements for free container
    let mut progress_elements = vec![(bg_bar, (0.0, 0.0)), (progress_fill, (0.0, 0.0))];

    // Add percentage text if requested
    if show_percentage {
        let percentage_text = builder.new_text(
            format!("{}_percentage", id),
            &format!("{}%", percentage as i32),
            TextOptions {
                font_family: "Arial".to_string(),
                font_size: 12.0,
                text_color: "#333333".to_string(),
                line_width: 50,
                line_spacing: 0.0,
            },
        );
        // Position text in the center of the progress bar
        progress_elements.push((percentage_text, ((width - 30.0) / 2.0, height / 2.0 - 6.0)));
    }

    // Create progress bar container
    let progress_container =
        builder.new_free_container(format!("{}_progress", id), progress_elements);

    let label_spacer = builder.new_rectangle(
        format!("{}_label_spacer", id),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(1.0),
            height_behavior: SizeBehavior::Fixed(8.0), // 8px gap
            fill_color: Fill::Color("transparent".to_string()),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            border_radius: 0.0,
        },
    );

    // Combine label, spacer, and progress bar
    let content = builder.new_vstack(
        format!("{}_content", id),
        vec![label_text, label_spacer, progress_container], // Added spacer here
        HorizontalAlignment::Left,
    );

    Ok(content)
}

/// Icon Stat Component - Icon with a large number and label
/// Example: {"type":"icon_stat","icon":"👥","value":"10,000","label":"Active Users","size":"large"}
pub fn create_icon_stat_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("icon_stat"));
    let icon = get_string_attr(attrs, "icon", "📊");
    let value = get_string_attr(attrs, "value", "0");
    let label = get_string_attr(attrs, "label", "Metric");
    let size = get_string_attr(attrs, "size", "medium");
    let color = get_string_attr(attrs, "color", "#333333");

    // Size configurations
    let (icon_size, value_size, label_size) = match size.as_str() {
        "small" => (20.0, 24.0, 12.0),
        "large" => (36.0, 48.0, 16.0),
        _ => (28.0, 36.0, 14.0), // medium
    };

    // Create icon
    let icon_element = builder.new_text(
        format!("{}_icon", id),
        &icon,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: icon_size,
            text_color: color.clone(),
            line_width: 100,
            line_spacing: 0.0,
        },
    );

    // Create value
    let value_element = builder.new_text(
        format!("{}_value", id),
        &value,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: value_size,
            text_color: color.clone(),
            line_width: 200,
            line_spacing: 0.0,
        },
    );

    // Create label
    let label_element = builder.new_text(
        format!("{}_label", id),
        &label,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: label_size,
            text_color: "#666666".to_string(),
            line_width: 200,
            line_spacing: 0.0,
        },
    );

    // Layout elements
    Ok(builder.new_vstack(
        id,
        vec![icon_element, value_element, label_element],
        HorizontalAlignment::Center,
    ))
}

// ============================================================================
// LAYOUT & STRUCTURE COMPONENTS
// ============================================================================

/// Hero Section Component - Large title with optional subtitle and background
/// Example: {"type":"hero_section","title":"2024 Report","subtitle":"Growth Beyond Expectations","background":"#f8f9fa"}
pub fn create_hero_section_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("hero_section"));
    let title = get_string_attr(attrs, "title", "Title");
    let subtitle = get_string_attr(attrs, "subtitle", "");
    let background = get_string_attr(attrs, "background", "#ffffff");
    let width = get_float_attr(attrs, "width", 800.0);
    let title_color = get_string_attr(attrs, "title_color", "#333333");
    let subtitle_color = get_string_attr(attrs, "subtitle_color", "#666666");

    // Create title
    let title_text = builder.new_text(
        format!("{}_title", id),
        &title,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 42.0,
            text_color: title_color,
            line_width: (width - 80.0) as usize,
            line_spacing: 4.0,
        },
    );

    let mut children = vec![title_text];

    // Add subtitle if provided
    if !subtitle.is_empty() {
        let subtitle_text = builder.new_text(
            format!("{}_subtitle", id),
            &subtitle,
            TextOptions {
                font_family: "Arial".to_string(),
                font_size: 18.0,
                text_color: subtitle_color,
                line_width: (width - 80.0) as usize,
                line_spacing: 2.0,
            },
        );
        children.push(subtitle_text);
    }

    // Layout content
    let content = builder.new_vstack(
        format!("{}_content", id),
        children,
        HorizontalAlignment::Center,
    );

    // Wrap in styled container
    Ok(builder.new_box(
        id,
        content,
        BoxOptions {
            fill_color: Fill::Color(background),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            padding: 40.0,
            border_radius: 0.0,
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Content,
        },
    ))
}

/// Timeline Component - Shows events in chronological order
/// Example: {"type":"timeline","items":[{"date":"Q1","event":"Launch"},{"date":"Q2","event":"Growth"}],"orientation":"horizontal"}
pub fn create_timeline_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("timeline"));
    let title = get_string_attr(attrs, "title", "");
    let orientation = get_string_attr(attrs, "orientation", "horizontal");
    let items = get_array_attr(attrs, "items");
    let color = get_string_attr(attrs, "color", "#007bff");

    let mut timeline_children = Vec::new();

    // Add title if provided
    if !title.is_empty() {
        let title_text = builder.new_text(
            format!("{}_title", id),
            &title,
            TextOptions {
                font_family: "Arial".to_string(),
                font_size: 20.0,
                text_color: "#333333".to_string(),
                line_width: 600,
                line_spacing: 0.0,
            },
        );
        timeline_children.push(title_text);
    }

    // Create timeline items
    let mut item_nodes = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if let Some(item_obj) = item.as_object() {
            let date = get_string_attr(item_obj, "date", &format!("Item {}", i + 1));
            let event = get_string_attr(item_obj, "event", "Event");

            // Create date
            let date_text = builder.new_text(
                format!("{}_date_{}", id, i),
                &date,
                TextOptions {
                    font_family: "Arial".to_string(),
                    font_size: 14.0,
                    text_color: color.clone(),
                    line_width: 150,
                    line_spacing: 0.0,
                },
            );

            // Create event
            let event_text = builder.new_text(
                format!("{}_event_{}", id, i),
                &event,
                TextOptions {
                    font_family: "Arial".to_string(),
                    font_size: 12.0,
                    text_color: "#666666".to_string(),
                    line_width: 150,
                    line_spacing: 2.0,
                },
            );

            // Create timeline item
            let item_content = builder.new_vstack(
                format!("{}_item_{}", id, i),
                vec![date_text, event_text],
                HorizontalAlignment::Center,
            );

            // Wrap in box for visual separation
            let item_box = builder.new_box(
                format!("{}_item_box_{}", id, i),
                item_content,
                BoxOptions {
                    fill_color: Fill::Color("#f8f9fa".to_string()),
                    stroke_color: color.clone(),
                    stroke_width: 1.0,
                    padding: 15.0,
                    border_radius: 8.0,
                    width_behavior: SizeBehavior::Fixed(180.0),
                    height_behavior: SizeBehavior::Content,
                },
            );

            item_nodes.push(item_box);
        }
    }

    // Layout timeline items based on orientation
    let timeline_items = match orientation.as_str() {
        "vertical" => builder.new_vstack(
            format!("{}_items", id),
            item_nodes,
            HorizontalAlignment::Center,
        ),
        _ => builder.new_hstack(format!("{}_items", id), item_nodes, VerticalAlignment::Top),
    };

    timeline_children.push(timeline_items);

    // Create final timeline layout
    Ok(builder.new_vstack(id, timeline_children, HorizontalAlignment::Center))
}

/// Callout Box Component - Highlighted text box for important information
/// Example: {"type":"callout_box","text":"Key Insight","style":"highlight","background":"#e8f5e8"}
pub fn create_callout_box_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("callout_box"));
    let text = get_string_attr(attrs, "text", "Important Information");
    let style = get_string_attr(attrs, "style", "default");
    let width = get_float_attr(attrs, "width", 400.0);
    let background = get_string_attr(attrs, "background", "");

    // Style configurations
    let (bg_color, border_color, text_color, icon) = match style.as_str() {
        "highlight" => ("#fff3cd", "#ffc107", "#856404", "⚠️"),
        "success" => ("#d4edda", "#28a745", "#155724", "✅"),
        "info" => ("#d1ecf1", "#17a2b8", "#0c5460", "ℹ️"),
        "warning" => ("#f8d7da", "#dc3545", "#721c24", "⚠️"),
        _ => ("#f8f9fa", "#dee2e6", "#333333", "💡"),
    };

    let final_bg = if background.is_empty() {
        bg_color
    } else {
        &background
    };

    // Create icon
    let icon_text = builder.new_text(
        format!("{}_icon", id),
        icon,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 18.0,
            text_color: border_color.to_string(),
            line_width: 30,
            line_spacing: 0.0,
        },
    );

    // Create text
    let content_text = builder.new_text(
        format!("{}_text", id),
        &text,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size: 14.0,
            text_color: text_color.to_string(),
            line_width: (width - 80.0) as usize,
            line_spacing: 2.0,
        },
    );

    // Layout content
    let content = builder.new_hstack(
        format!("{}_content", id),
        vec![icon_text, content_text],
        VerticalAlignment::Top,
    );

    // Wrap in styled box
    Ok(builder.new_box(
        id,
        content,
        BoxOptions {
            fill_color: Fill::Color(final_bg.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: 1.0,
            padding: 16.0,
            border_radius: 8.0,
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Content,
        },
    ))
}

// ============================================================================
// VISUAL ENHANCEMENT COMPONENTS
// ============================================================================
/// Badge Component - Small label/tag
/// Example: {"type":"badge","text":"NEW","color":"red","style":"pill"}
pub fn create_badge_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let id = get_string_attr(attrs, "id", &generate_unique_id("badge"));
    let text = get_string_attr(attrs, "text", "Badge");
    let color = get_string_attr(attrs, "color", "blue");
    let style = get_string_attr(attrs, "style", "default");
    let size = get_string_attr(attrs, "size", "medium");

    // Color configurations
    let (bg_color, text_color) = match color.as_str() {
        "red" => ("#dc3545", "#ffffff"),
        "green" => ("#28a745", "#ffffff"),
        "blue" => ("#007bff", "#ffffff"),
        "yellow" => ("#ffc107", "#212529"),
        "gray" => ("#6c757d", "#ffffff"),
        _ => ("#007bff", "#ffffff"),
    };

    // Size configurations
    let (font_size, padding) = match size.as_str() {
        "small" => (10.0, 4.0),
        "large" => (14.0, 12.0),
        _ => (12.0, 8.0), // medium
    };

    // Style configurations
    let border_radius = match style.as_str() {
        "pill" => font_size,
        "rounded" => 4.0,
        _ => 2.0,
    };

    // Create text
    let badge_text = builder.new_text(
        format!("{}_text", id),
        &text,
        TextOptions {
            font_family: "Arial".to_string(),
            font_size,
            text_color: text_color.to_string(),
            line_width: 100,
            line_spacing: 0.0,
        },
    );

    // Wrap in styled box
    Ok(builder.new_box(
        id,
        badge_text,
        BoxOptions {
            fill_color: Fill::Color(bg_color.to_string()),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            padding,
            border_radius,
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Content,
        },
    ))
}

/// Quote Block Component - Stylized quote with author attribution
/// Example: {"type":"quote_block","text":"This changed everything","author":"CEO Jane Smith","style":"large"}
/// Quote Block Component - Stylized quote with author attribution
/// Example: {"type":"quote_block","text":"This changed everything","author":"CEO Jane Smith","style":"large"}
fn create_quote_block_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    println!("💬 Creating quote block component with attrs: {:?}", attrs);

    // Extract attributes
    let quote_text = get_string_attr(attrs, "text", "Quote text");
    let author = get_string_attr(attrs, "author", "");
    let width = get_float_attr(attrs, "width", 400.0);
    let padding = get_float_attr(attrs, "padding", 20.0);
    let style = get_string_attr(attrs, "style", "default"); // default, modern, minimal, elegant
    let show_quote_marks = get_bool_attr(attrs, "show_quote_marks", true);
    let font_size = get_float_attr(attrs, "font_size", 14.0);

    let mut id = get_string_attr(attrs, "id", "");
    if id.is_empty() {
        id = uuid::Uuid::new_v4().to_string();
    }

    // Define styles based on variant
    let (bg_color, border_color, text_color, author_color, border_width, border_radius) = match style.as_str() {
        "modern" => ("#f8f9fa", "#007bff", "#333333", "#007bff", 2.0, 8.0),
        "minimal" => ("#ffffff", "#e9ecef", "#2c3e50", "#6c757d", 1.0, 4.0),
        "elegant" => ("#fefefe", "#d4af37", "#2c3e50", "#d4af37", 2.0, 12.0),
        _ => ("#f5f5f5", "#6c757d", "#333333", "#666666", 1.0, 6.0), // default
    };

    // Calculate inner width for text wrapping (total width minus outer padding)
    let text_width = width - (padding * 2.0);

    let mut children = Vec::new();

    // Create the main quote text with auto-wrapping
    let quote_content = if show_quote_marks {
        format!("\"{}\"", &quote_text)
    } else {
        quote_text.clone()
    };

    let quote_text_node = builder.new_text(
        format!("{}_quote_text", id),
        &quote_content,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size,
            text_color: text_color.to_string(),
            line_width: 200, // This will be overridden by the box auto-wrapping
            line_spacing: 4.0,
        },
    );

    // CRITICAL: Wrap the quote text in its own fixed-width box for auto-wrapping
    let quote_text_box = builder.new_box(
        format!("{}_quote_text_box", id),
        quote_text_node,
        BoxOptions {
            fill_color: Fill::Color("transparent".to_string()),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            padding: 0.0,
            border_radius: 0.0,
            width_behavior: SizeBehavior::Fixed(text_width), // Fixed width enables auto-wrapping
            height_behavior: SizeBehavior::Content,
        },
    );
    children.push(quote_text_box);

    // Add author attribution if provided
    if !author.is_empty() {
        let author_text = format!("— {}", &author);
        let author_node = builder.new_text(
            format!("{}_author", id),
            &author_text,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: font_size * 0.85, // Slightly smaller than quote text
                text_color: author_color.to_string(),
                line_width: 200, // This will be overridden by the box auto-wrapping
                line_spacing: 0.0,
            },
        );

        // CRITICAL: Wrap the author text in its own fixed-width box for auto-wrapping
        let author_text_box = builder.new_box(
            format!("{}_author_text_box", id),
            author_node,
            BoxOptions {
                fill_color: Fill::Color("transparent".to_string()),
                stroke_color: "transparent".to_string(),
                stroke_width: 0.0,
                padding: 0.0,
                border_radius: 0.0,
                width_behavior: SizeBehavior::Fixed(text_width), // Fixed width enables auto-wrapping
                height_behavior: SizeBehavior::Content,
            },
        );
        children.push(author_text_box);
    }

    // Create vertical layout for the wrapped text boxes
    let content_stack = builder.new_vstack(
        format!("{}_content", id),
        children,
        HorizontalAlignment::Left, // Left-align for natural reading flow
    );

    // Create the outer styled container
    let quote_box = builder.new_box(
        id,
        content_stack,
        BoxOptions {
            fill_color: Fill::Color(bg_color.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: border_width,
            padding,
            border_radius,
            width_behavior: SizeBehavior::Fixed(width), // Overall fixed width
            height_behavior: SizeBehavior::Content,     // Content height accommodates wrapped text
        },
    );

    let quote_preview = if quote_text.len() > 30 { 
        format!("{}...", &quote_text[..30]) 
    } else { 
        quote_text.clone() 
    };

    println!("✅ Quote block '{}' created with auto-wrapping at {}px width", 
             quote_preview, 
             width);
    
    Ok(quote_box)
}
// ============================================================================
// REGISTRATION FUNCTION
// ============================================================================

/// Register all infographic components with a DiagramBuilder
pub fn register_infographic_components(builder: &mut DiagramBuilder) {
    // Data visualization components
    builder.register_custom_component("stat_card", create_stat_card_component);
    builder.register_custom_component("progress_bar", create_progress_bar_component);
    builder.register_custom_component("icon_stat", create_icon_stat_component);

    // Layout & structure components
    builder.register_custom_component("hero_section", create_hero_section_component);
    builder.register_custom_component("timeline", create_timeline_component);
    builder.register_custom_component("callout_box", create_callout_box_component);

    // Visual enhancement components
    builder.register_custom_component("badge", create_badge_component);
    builder.register_custom_component("quote_block", create_quote_block_component);

    println!("Infographic components registered successfully!");
}
