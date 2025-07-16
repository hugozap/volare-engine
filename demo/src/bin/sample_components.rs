// src/bin/useful_components_demo.rs
// A library of practical custom components for real-world usage

use demo::measure_text::measure_text_svg_character_advance;
use serde_json::{json, Map, Value};
use std::fs::File;
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

fn get_array_attr(attrs: &Map<String, Value>, key: &str) -> Vec<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// Card Component - A flexible container with header, body, and footer
fn create_card_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let title = get_string_attr(attrs, "title", "");
    let subtitle = get_string_attr(attrs, "subtitle", "");
    let content = get_string_attr(attrs, "content", "");
    let footer = get_string_attr(attrs, "footer", "");
    let width = get_float_attr(attrs, "width", 300.0);
    let padding = get_float_attr(attrs, "padding", 16.0);
    let border_radius = get_float_attr(attrs, "border_radius", 8.0);
    let shadow = get_bool_attr(attrs, "shadow", true);
    let variant = get_string_attr(attrs, "variant", "default");

    // Define card styles based on variant
    let (bg_color, border_color) = match variant.as_str() {
        "primary" => ("#ffffff", "#007bff"),
        "success" => ("#f8fff8", "#28a745"),
        "warning" => ("#fffdf0", "#ffc107"),
        "danger" => ("#fff8f8", "#dc3545"),
        _ => ("#ffffff", "#e0e0e0"),
    };

    let mut children = Vec::new();

    // Add title if provided
    if !title.is_empty() {
        let title_text = builder.new_text(
            &title,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 18.0,
                text_color: "#333333".to_string(),
                line_width: (width - padding * 2.0) as usize,
                line_spacing: 0.0,
            },
        );
        children.push(title_text);
    }

    // Add subtitle if provided
    if !subtitle.is_empty() {
        let subtitle_text = builder.new_text(
            &subtitle,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 14.0,
                text_color: "#666666".to_string(),
                line_width: (width - padding * 2.0) as usize,
                line_spacing: 0.0,
            },
        );
        children.push(subtitle_text);
    }

    // Add content if provided
    if !content.is_empty() {
        let content_text = builder.new_text(
            &content,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 14.0,
                text_color: "#444444".to_string(),
                line_width: (width - padding * 2.0) as usize,
                line_spacing: 4.0,
            },
        );
        children.push(content_text);
    }

    // Add footer if provided
    if !footer.is_empty() {
        let footer_text = builder.new_text(
            &footer,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 12.0,
                text_color: "#888888".to_string(),
                line_width: (width - padding * 2.0) as usize,
                line_spacing: 0.0,
            },
        );
        children.push(footer_text);
    }

    // Create layout
    let content_stack = builder.new_vstack(children, HorizontalAlignment::Left);

    // Apply shadow effect by creating multiple boxes
    if shadow {
        // Shadow box (slightly offset)
        let shadow_box = builder.new_box(
            content_stack,
            BoxOptions {
                fill_color: Fill::Color("#00000010".to_string()),
                stroke_color: "transparent".to_string(),
                stroke_width: 0.0,
                padding,
                border_radius,
                width_behavior: SizeBehavior::Fixed(width),
                height_behavior: SizeBehavior::Content,
            },
        );

        // Main card box
        let main_content = builder.new_rectangle(RectOptions {
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Content,
            fill_color: Fill::Color(bg_color.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: 1.0,
            border_radius,
        });

        // Use free container to overlay them
        let card = builder.new_free_container(vec![
            (shadow_box, (2.0, 2.0)),   // Shadow slightly offset
            (main_content, (0.0, 0.0)), // Main card on top
        ]);
        Ok(card)
    } else {
        let card = builder.new_box(
            content_stack,
            BoxOptions {
                fill_color: Fill::Color(bg_color.to_string()),
                stroke_color: border_color.to_string(),
                stroke_width: 1.0,
                padding,
                border_radius,
                width_behavior: SizeBehavior::Fixed(width),
                height_behavior: SizeBehavior::Content,
            },
        );
        Ok(card)
    }
}

/// List Component - Creates a styled list with bullets or numbers
fn create_list_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let items = get_array_attr(attrs, "items");
    let list_type = get_string_attr(attrs, "list_type", "bullet"); // bullet, number, check
    let spacing = get_float_attr(attrs, "spacing", 4.0);
    let width = get_float_attr(attrs, "width", 300.0);
    let font_size = get_float_attr(attrs, "font_size", 14.0);

    if items.is_empty() {
        return Err("List component requires 'items' array".to_string());
    }

    let mut list_children = Vec::new();

    for (index, item) in items.iter().enumerate() {
        // Create bullet/number
        let marker = match list_type.as_str() {
            "number" => format!("{}.", index + 1),
            "check" => "✓".to_string(),
            "arrow" => "→".to_string(),
            _ => "•".to_string(), // bullet
        };

        let marker_text = builder.new_text(
            &marker,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size,
                text_color: "#666666".to_string(),
                line_width: 30,
                line_spacing: 0.0,
            },
        );

        let item_text = builder.new_text(
            item,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size,
                text_color: "#333333".to_string(),
                line_width: (width - 40.0) as usize,
                line_spacing: 0.0,
            },
        );

        let list_item = builder.new_hstack(vec![marker_text, item_text], VerticalAlignment::Top);
        list_children.push(list_item);
    }

    let list = builder.new_vstack(list_children, HorizontalAlignment::Left);
    Ok(list)
}

/// Form Field Component - Creates labeled input-like elements
fn create_form_field_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let label = get_string_attr(attrs, "label", "Field");
    let placeholder = get_string_attr(attrs, "placeholder", "Enter value...");
    let field_type = get_string_attr(attrs, "field_type", "text"); // text, email, password, textarea
    let required = get_bool_attr(attrs, "required", false);
    let width = get_float_attr(attrs, "width", 250.0);
    let error = get_string_attr(attrs, "error", "");

    let mut children = Vec::new();

    // Create label
    let label_text = if required {
        format!("{} *", label)
    } else {
        label
    };

    let label_node = builder.new_text(
        &label_text,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 14.0,
            text_color: if required { "#333333" } else { "#666666" }.to_string(),
            line_width: width as usize,
            line_spacing: 0.0,
        },
    );
    children.push(label_node);

    // Create input field representation
    let field_height = match field_type.as_str() {
        "textarea" => 60.0,
        _ => 36.0,
    };

    let placeholder_text = builder.new_text(
        &placeholder,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 14.0,
            text_color: "#999999".to_string(),
            line_width: (width - 24.0) as usize,
            line_spacing: 0.0,
        },
    );

    let input_field = builder.new_box(
        placeholder_text,
        BoxOptions {
            fill_color: Fill::Color("#ffffff".to_string()),
            stroke_color: if !error.is_empty() {
                "#dc3545"
            } else {
                "#cccccc"
            }
            .to_string(),
            stroke_width: 1.0,
            padding: 12.0,
            border_radius: 4.0,
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Fixed(field_height),
        },
    );
    children.push(input_field);

    // Add error message if present
    if !error.is_empty() {
        let error_text = builder.new_text(
            &error,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 12.0,
                text_color: "#dc3545".to_string(),
                line_width: width as usize,
                line_spacing: 0.0,
            },
        );
        children.push(error_text);
    }

    let form_field = builder.new_vstack(children, HorizontalAlignment::Left);
    Ok(form_field)
}

/// Stats Card Component - Displays key metrics with icons
fn create_stats_card_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let title = get_string_attr(attrs, "title", "Metric");
    let value = get_string_attr(attrs, "value", "0");
    let change = get_string_attr(attrs, "change", "");
    let icon = get_string_attr(attrs, "icon", "📊");
    let trend = get_string_attr(attrs, "trend", "neutral"); // up, down, neutral
    let color = get_string_attr(attrs, "color", "#007bff");

    // Determine trend color and symbol
    let (trend_color, trend_symbol) = match trend.as_str() {
        "up" => ("#28a745", "↗"),
        "down" => ("#dc3545", "↘"),
        _ => ("#6c757d", "→"),
    };

    // Create icon
    let icon_text = builder.new_text(
        &icon,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 12.0,
            text_color: color,
            line_width: 50,
            line_spacing: 0.0,
        },
    );

    // Create value and title section
    let value_text = builder.new_text(
        &value,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 28.0,
            text_color: "#333333".to_string(),
            line_width: 150,
            line_spacing: 0.0,
        },
    );

    let title_text = builder.new_text(
        &title,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 14.0,
            text_color: "#666666".to_string(),
            line_width: 150,
            line_spacing: 0.0,
        },
    );

    let mut right_children = vec![value_text, title_text];

    // Add change indicator if provided
    if !change.is_empty() {
        let change_text = format!("{} {}", trend_symbol, change);
        let change_node = builder.new_text(
            &change_text,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 12.0,
                text_color: trend_color.to_string(),
                line_width: 150,
                line_spacing: 0.0,
            },
        );
        right_children.push(change_node);
    }

    let right_section = builder.new_vstack(right_children, HorizontalAlignment::Left);
    let content = builder.new_hstack(vec![icon_text, right_section], VerticalAlignment::Center);

    let stats_card = builder.new_box(
        content,
        BoxOptions {
            fill_color: Fill::Color("#ffffff".to_string()),
            stroke_color: "#e0e0e0".to_string(),
            stroke_width: 1.0,
            padding: 20.0,
            border_radius: 8.0,
            width_behavior: SizeBehavior::Fixed(300.0),
            height_behavior: SizeBehavior::Content,
        },
    );

    Ok(stats_card)
}

/// Navigation Menu Component - Creates a horizontal or vertical menu
fn create_nav_menu_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let items = get_array_attr(attrs, "items");
    let orientation = get_string_attr(attrs, "orientation", "horizontal"); // horizontal, vertical
    let active_item = get_string_attr(attrs, "active_item", "");
    let style = get_string_attr(attrs, "style", "default"); // default, pills, tabs

    if items.is_empty() {
        return Err("Navigation menu requires 'items' array".to_string());
    }

    let mut nav_children = Vec::new();

    for item in items.iter() {
        let is_active = item == &active_item;

        // Style based on state and style type
        let (bg_color, text_color, border_color) = match (is_active, style.as_str()) {
            (true, "pills") => ("#007bff", "#ffffff", "#007bff"),
            (false, "pills") => ("transparent", "#007bff", "transparent"),
            (true, "tabs") => ("#ffffff", "#007bff", "#007bff"),
            (false, "tabs") => ("transparent", "#666666", "#e0e0e0"),
            (true, _) => ("transparent", "#007bff", "transparent"),
            (false, _) => ("transparent", "#666666", "transparent"),
        };

        let nav_text = builder.new_text(
            item,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 14.0,
                text_color: text_color.to_string(),
                line_width: 100,
                line_spacing: 0.0,
            },
        );

        let nav_item = builder.new_box(
            nav_text,
            BoxOptions {
                fill_color: Fill::Color(bg_color.to_string()),
                stroke_color: border_color.to_string(),
                stroke_width: if style == "tabs" { 1.0 } else { 0.0 },
                padding: 12.0,
                border_radius: if style == "pills" { 20.0 } else { 4.0 },
                width_behavior: SizeBehavior::Content,
                height_behavior: SizeBehavior::Content,
            },
        );

        nav_children.push(nav_item);
    }

    let nav_menu = match orientation.as_str() {
        "vertical" => builder.new_vstack(nav_children, HorizontalAlignment::Left),
        _ => builder.new_hstack(nav_children, VerticalAlignment::Center),
    };

    Ok(nav_menu)
}

/// Header Component - Creates page headers with optional breadcrumbs
fn create_header_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    let title = get_string_attr(attrs, "title", "Page Title");
    let subtitle = get_string_attr(attrs, "subtitle", "");
    let breadcrumbs = get_array_attr(attrs, "breadcrumbs");
    let show_back = get_bool_attr(attrs, "show_back", false);

    let mut children = Vec::new();

    // Add breadcrumbs if provided
    if !breadcrumbs.is_empty() {
        let breadcrumb_text = breadcrumbs.join(" > ");
        let breadcrumb_node = builder.new_text(
            &breadcrumb_text,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 12.0,
                text_color: "#666666".to_string(),
                line_width: 600,
                line_spacing: 0.0,
            },
        );
        children.push(breadcrumb_node);
    }

    // Add back button if requested
    if show_back {
        let back_text = builder.new_text(
            "← Back",
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 14.0,
                text_color: "#007bff".to_string(),
                line_width: 100,
                line_spacing: 0.0,
            },
        );
        children.push(back_text);
    }

    // Add main title
    let title_node = builder.new_text(
        &title,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 32.0,
            text_color: "#333333".to_string(),
            line_width: 600,
            line_spacing: 0.0,
        },
    );
    children.push(title_node);

    // Add subtitle if provided
    if !subtitle.is_empty() {
        let subtitle_node = builder.new_text(
            &subtitle,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 16.0,
                text_color: "#666666".to_string(),
                line_width: 600,
                line_spacing: 0.0,
            },
        );
        children.push(subtitle_node);
    }

    let header = builder.new_vstack(children, HorizontalAlignment::Left);
    Ok(header)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Useful Components Demo Starting...\n");

    // Create diagram builder and register components
    let mut builder = DiagramBuilder::new();
    builder.set_measure_text_fn(measure_text_svg_character_advance);

    // Register all useful components
    println!("📝 Registering useful components...");
    builder.register_custom_component("card", create_card_component);
    builder.register_custom_component("list", create_list_component);
    builder.register_custom_component("form_field", create_form_field_component);
    builder.register_custom_component("stats_card", create_stats_card_component);
    builder.register_custom_component("nav_menu", create_nav_menu_component);
    builder.register_custom_component("header", create_header_component);
    println!("✅ All useful components registered!\n");

    // The JSONL content - Real-world dashboard example
    let jsonl_input = r##"{"id":"root","type":"vstack","children":["main_header","nav","dashboard_content"],"h_align":"center"}
{"id":"main_header","type":"header","title":"Analytics Dashboard","subtitle":"Monitor your key metrics and performance indicators","breadcrumbs":["Home","Analytics","Dashboard"]}
{"id":"nav","type":"nav_menu","items":["Overview","Analytics","Reports","Settings"],"active_item":"Analytics","style":"tabs","orientation":"horizontal"}
{"id":"dashboard_content","type":"vstack","children":["stats_row","charts_row","recent_activity"],"h_align":"center"}
{"id":"stats_row","type":"hstack","children":["stats1","stats2","stats3","stats4"],"v_align":"top"}
{"id":"stats1","type":"stats_card","title":"Total Users","value":"12,345","change":"+12.5%","trend":"up","icon":"👥","color":"#007bff"}
{"id":"stats2","type":"stats_card","title":"Revenue","value":"$89,432","change":"+8.2%","trend":"up","icon":"💰","color":"#28a745"}
{"id":"stats3","type":"stats_card","title":"Orders","value":"1,234","change":"-2.1%","trend":"down","icon":"📦","color":"#ffc107"}
{"id":"stats4","type":"stats_card","title":"Conversion","value":"3.24%","change":"+0.8%","trend":"up","icon":"📈","color":"#17a2b8"}
{"id":"charts_row","type":"hstack","children":["performance_card","recent_orders_card"],"v_align":"top"}
{"id":"performance_card","type":"card","title":"Performance Metrics","content":"Your application performance has improved by 23% this month. Server response times are optimal and user engagement is at an all-time high.","footer":"Last updated: 2 minutes ago","width":400,"variant":"primary","shadow":true}
{"id":"recent_orders_card","type":"card","title":"Recent Orders","width":400,"variant":"default","shadow":true,"children":["orders_list"]}
{"id":"orders_list","type":"list","items":["Order #1234 - $299.99 - Processing","Order #1235 - $156.78 - Shipped","Order #1236 - $89.50 - Delivered","Order #1237 - $445.20 - Processing"],"list_type":"number","spacing":6,"width":350}
{"id":"recent_activity","type":"vstack","children":["activity_header","activity_content"],"h_align":"left"}
{"id":"activity_header","type":"text","content":"Recent Activity","font_size":20,"color":"#333333"}
{"id":"activity_content","type":"hstack","children":["activity_list","user_form"],"v_align":"top"}
{"id":"activity_list","type":"list","items":["User john.doe logged in","New order received (#1238)","Payment processed for order #1235","User jane.smith updated profile","System backup completed","New user registration: mike.wilson"],"list_type":"arrow","width":400,"font_size":13}
{"id":"user_form","type":"vstack","children":["form_title","name_field","email_field","message_field","submit_section"],"h_align":"left"}
{"id":"form_title","type":"text","content":"Quick Contact","font_size":18,"color":"#333333"}
{"id":"name_field","type":"form_field","label":"Full Name","placeholder":"Enter your full name","required":true,"width":300}
{"id":"email_field","type":"form_field","label":"Email Address","placeholder":"your.email@company.com","field_type":"email","required":true,"width":300}
{"id":"message_field","type":"form_field","label":"Message","placeholder":"Type your message here...","field_type":"textarea","width":300}
{"id":"submit_section","type":"card","content":"Form ready to submit","footer":"All fields are validated","width":300,"variant":"success"}"##;

    // Parse and build the diagram
    let mut parser = parser::JsonLinesParser::new();
    let root_id = parser.parse_string(jsonl_input)?;

    // Create fresh builder for parsing
    let mut parse_builder = DiagramBuilder::new();
    parse_builder.set_measure_text_fn(measure_text_svg_character_advance);

    // Register components
    parse_builder.register_custom_component("card", create_card_component);
    parse_builder.register_custom_component("list", create_list_component);
    parse_builder.register_custom_component("form_field", create_form_field_component);
    parse_builder.register_custom_component("stats_card", create_stats_card_component);
    parse_builder.register_custom_component("nav_menu", create_nav_menu_component);
    parse_builder.register_custom_component("header", create_header_component);

    // Build and layout
    let diagram = parser.build(&root_id, &mut parse_builder)?;
    layout::layout_tree_node(&mut parse_builder, &diagram);

    // Render to SVG
    let temp_dir = std::env::temp_dir();
    let mut svg_path = temp_dir.clone();
    svg_path.push("useful-components-dashboard.svg");

    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&svg_path)?;
    svg_renderer.render(&parse_builder, &diagram, &mut svg_file)?;

    println!("✅ Dashboard rendered successfully!");
    println!("📄 File saved to: {}", svg_path.to_str().unwrap());
    println!("\n🎉 Useful Components Demo Complete!");

    Ok(())
}
