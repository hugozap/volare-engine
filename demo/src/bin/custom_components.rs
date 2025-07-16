// src/bin/custom_component_demo.rs
// Demo program showing how to register and use custom components

use demo::measure_text::measure_text_svg_character_advance;
use serde_json::{json, Map, Value};
use std::{fmt::format, fs::File};
use volare_engine_layout::*;

// Helper function to extract attributes (since we can't access CustomComponentRegistry helpers directly)
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

/// Custom Component 1: Badge
/// Creates a rounded pill-shaped element with text
fn create_badge_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    println!("🏷️  Creating badge component with attrs: {:?}", attrs);

    // Extract attributes
    let text = get_string_attr(attrs, "text", "Badge");
    let background = get_string_attr(attrs, "background", "blue");
    let color = get_string_attr(attrs, "color", "white");
    let font_size = get_float_attr(attrs, "font_size", 12.0);
    let padding = get_float_attr(attrs, "padding", 8.0);

    let mut id = get_string_attr(attrs, "id", "");
    if id.is_empty() {
        id =  uuid::Uuid::new_v4().to_string()
    }

    // Create text element
    let text_options = TextOptions {
        font_family: "AnonymicePro Nerd Font".to_string(),
        font_size,
        text_color: color,
        line_width: 200,
        line_spacing: 0.0,
    };
    let text_node = builder.new_text(
        format!("{}_text", id),
        &text, text_options);

    // Wrap in rounded box
    let box_options = BoxOptions {
        fill_color: Fill::Color(background),
        stroke_color: "transparent".to_string(),
        stroke_width: 0.0,
        padding,
        border_radius: font_size,               // Make it pill-shaped
        width_behavior: SizeBehavior::Content,  // Auto width based on text
        height_behavior: SizeBehavior::Content, // Auto height based on text
    };
    let badge = builder.new_box(
        id,
        text_node, box_options);

    println!("✅ Badge '{}' created successfully", text);
    Ok(badge)
}

/// Custom Component 2: Alert Box
/// Creates an alert with optional icon, title, and message
fn create_alert_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    println!("⚠️  Creating alert component with attrs: {:?}", attrs);

    let alert_type = get_string_attr(attrs, "alert-type", "info");
    let title = get_string_attr(attrs, "title", "Alert");
    let message = get_string_attr(attrs, "message", "Alert message");
    let width = get_float_attr(attrs, "width", 300.0);
    let show_icon = get_bool_attr(attrs, "show_icon", true);
    let mut id = get_string_attr(attrs, "id", "");

    if id.is_empty() {
        id = uuid::Uuid::new_v4().to_string()
    }

    //TODO aqui seria util tener
    // let context = builder.CreateComponentContext(id)
    // y todos los elems internos les asigna prefijo el id
    // util para no tener que concatenarlos

    // Define alert styles
    let (bg_color, border_color, icon) = match alert_type.as_str() {
        "success" => ("#d4edda", "#28a745", "✓"),
        "warning" => ("#fff3cd", "#ffc107", "⚠"),
        "error" => ("#f8d7da", "#dc3545", "✗"),
        "info" | _ => ("#d1ecf1", "#17a2b8", "ℹ"),
    };

    let mut children = Vec::new();

    // Create header with optional icon
    if show_icon {
        let icon_text = builder.new_text(
            format!("{}_showicon", id),
            icon,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 18.0,
                text_color: border_color.to_string(),
                line_width: 50,
                line_spacing: 0.0,
            },
        );

        let title_text = builder.new_text(
            format!("{}_title", id),
            &title,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 16.0,
                text_color: "#333".to_string(),
                line_width: (width - 50.0) as usize,
                line_spacing: 0.0,
            },
        );

        let header = builder.new_hstack(
            format!("{}_header", id),
            vec![icon_text, title_text],
            VerticalAlignment::Center,
        );
        children.push(header);
    } else {
        let title_text = builder.new_text(
            format!("{}_title", id),
            &title,
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: 16.0,
                text_color: "#333".to_string(),
                line_width: width as usize,
                line_spacing: 0.0,
            },
        );
        children.push(title_text);
    }

    // Add message
    let message_text = builder.new_text(
        format!("{}_addmsg", id),
        &message,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size: 14.0,
            text_color: "#666".to_string(),
            line_width: width as usize,
            line_spacing: 2.0,
        },
    );
    children.push(message_text);

    // Create vertical layout
    let content = builder.new_vstack(
        format!("{}_contentstack", id),
        children,
        HorizontalAlignment::Left,
    );

    // Wrap in styled box
    let alert_box = builder.new_box(
        id,
        content,
        BoxOptions {
            fill_color: Fill::Color(bg_color.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: 1.0,
            padding: 16.0,
            border_radius: 8.0,
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Content, // Auto height based on content
        },
    );

    println!("✅ Alert '{}' ({}) created successfully", title, alert_type);
    Ok(alert_box)
}

/// Custom Component 3: Progress Bar
/// Creates a progress bar with background and fill
fn create_progress_bar_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    println!("📊 Creating progress bar component with attrs: {:?}", attrs);

    let width = get_float_attr(attrs, "width", 200.0);
    let height = get_float_attr(attrs, "height", 20.0);
    let progress = get_float_attr(attrs, "progress", 0.5).min(1.0).max(0.0);
    let bg_color = get_string_attr(attrs, "bg_color", "lightgray");
    let fill_color = get_string_attr(attrs, "fill_color", "blue");
    let show_text = get_bool_attr(attrs, "show_text", false);
    let mut id = get_string_attr(attrs, "id", "");
    if id.is_empty() {
        // Generate a unique ID if not provided
         id = format!("progress_bar_{}", uuid::Uuid::new_v4());
    }

    // Create background bar
    let bg_rect = builder.new_rectangle(
        format!("bg_{}", id),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(width),
            height_behavior: SizeBehavior::Fixed(height),
            fill_color: Fill::Color(bg_color),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            border_radius: height / 2.0,
        },
    );

    // Create progress fill
    let fill_width = width * progress;
    let fill_rect = builder.new_rectangle(
        format!("fill_{}", id),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(fill_width),
            height_behavior: SizeBehavior::Fixed(height),
            fill_color: Fill::Color(fill_color),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            border_radius: height / 2.0,
        },
    );

    let mut elements = vec![(bg_rect, (0.0, 0.0)), (fill_rect, (0.0, 0.0))];

    // Add percentage text if requested
    if show_text {
        let percentage = (progress * 100.0) as i32;
        let text_node = builder.new_text(
            format!("text_{}", id),
            &format!("{}%", percentage),
            TextOptions {
                font_family: "AnonymicePro Nerd Font".to_string(),
                font_size: height * 0.7,
                text_color: "black".to_string(),
                line_width: 100,
                line_spacing: 0.0,
            },
        );
        // Center the text roughly
        let text_x = (width - 30.0) / 2.0; // Rough centering
        let text_y = height * 0.15;
        elements.push((text_node, (text_x, text_y)));
    }

    let progress_bar = builder.new_free_container(id.to_string(), elements);

    println!(
        "✅ Progress bar ({}%) created successfully",
        (progress * 100.0) as i32
    );
    Ok(progress_bar)
}

/// Custom Component 4: Button
/// Creates a clickable button with text
fn create_button_component(
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
) -> Result<diagram_builder::DiagramTreeNode, String> {
    println!("🔘 Creating button component with attrs: {:?}", attrs);

    let text = get_string_attr(attrs, "text", "Button");
    let variant = get_string_attr(attrs, "variant", "primary");
    let size = get_string_attr(attrs, "size", "medium");
    let disabled = get_bool_attr(attrs, "disabled", false);
    let mut id = get_string_attr(attrs, "id", "");
    if id.is_empty() {
        //use uuid
        id = format!("button_{}", uuid::Uuid::new_v4());
    }

    // Define button styles based on variant
    let (bg_color, text_color, border_color) = if disabled {
        ("#cccccc", "#666666", "#999999")
    } else {
        match variant.as_str() {
            "primary" => ("#007bff", "white", "#0056b3"),
            "secondary" => ("#6c757d", "white", "#545b62"),
            "success" => ("#28a745", "white", "#1e7e34"),
            "danger" => ("#dc3545", "white", "#bd2130"),
            "warning" => ("#ffc107", "#212529", "#d39e00"),
            _ => ("#007bff", "white", "#0056b3"),
        }
    };

    // Define size-based properties
    let (font_size, padding_x, padding_y) = match size.as_str() {
        "small" => (12.0, 12.0, 6.0),
        "large" => (18.0, 24.0, 12.0),
        _ => (14.0, 16.0, 8.0), // medium
    };

    // Create button text
    let button_text = builder.new_text(
        format!("{}-{}", id, text),
        &text,
        TextOptions {
            font_family: "AnonymicePro Nerd Font".to_string(),
            font_size,
            text_color: text_color.to_string(),
            line_width: 200,
            line_spacing: 0.0,
        },
    );

    // Wrap in styled box
    let button = builder.new_box(
        format!("button_{}", id),
        button_text,
        BoxOptions {
            fill_color: Fill::Color(bg_color.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: 1.0,
            padding: f32::max(padding_x, padding_y), // Use max for uniform padding
            border_radius: 4.0,
            width_behavior: SizeBehavior::Content, // Auto width based on text
            height_behavior: SizeBehavior::Content, // Auto height based on text
        },
    );

    println!(
        "✅ Button '{}' ({}, {}) created successfully",
        text, variant, size
    );
    Ok(button)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Custom Component Demo Starting...\n");

    // Create diagram builder and set text measurement function
    let mut builder = DiagramBuilder::new();
    builder.set_measure_text_fn(measure_text_svg_character_advance);

    // Register all our custom components
    println!("📝 Registering custom components...");
    builder.register_custom_component("badge", create_badge_component);
    builder.register_custom_component("alert", create_alert_component);
    builder.register_custom_component("progress_bar", create_progress_bar_component);
    builder.register_custom_component("button", create_button_component);
    println!("✅ All custom components registered!\n");

    // Demo 1: Direct usage with Rust API
    println!("=== Demo 1: Direct Rust API Usage ===");

    // Create components directly
    let badge_attrs = json!({
        "text": "NEW",
        "background": "red",
        "color": "white",
        "font_size": 14.0,
        "padding": 10.0
    })
    .as_object()
    .unwrap()
    .clone();

    let _badge = builder.create_custom_component("badge", &badge_attrs)?;

    let alert_attrs = json!({
        "type": "success",
        "title": "Success!",
        "message": "Your custom component system is working perfectly!",
        "width": 400.0,
        "show_icon": true
    })
    .as_object()
    .unwrap()
    .clone();

    let _alert = builder.create_custom_component("alert", &alert_attrs)?;

    println!("✅ Direct API usage successful!\n");

    // Demo 2: JSON Lines usage
    println!("=== Demo 2: JSON Lines Usage ===");

    let jsonl_input = r##"
{"id":"root","type":"vstack","children":["title","demo_section","buttons_section","progress_section","alerts_section"],"h_align":"center"}
{"id":"title","type":"text","content":"🎨 Custom Components Showcase","font_size":25,"color":"darkblue"}
{"id":"demo_section","type":"hstack","children":["badge1","badge2","badge3"],"v_align":"center"}
{"id":"badge1","type":"badge","text":"NEW","background":"#ff4444","color":"white","font_size":12,"padding":8}
{"id":"badge2","type":"badge","text":"SALE","background":"#44ff44","color":"black","font_size":12,"padding":8}
{"id":"badge3","type":"badge","text":"HOT","background":"#ff8800","color":"white","font_size":12,"padding":8}
{"id":"buttons_section","type":"hstack","children":["btn1","btn2","btn3","btn4"],"v_align":"center"}
{"id":"btn1","type":"button","text":"Primary","variant":"primary","size":"medium"}
{"id":"btn2","type":"button","text":"Success","variant":"success","size":"medium"}
{"id":"btn3","type":"button","text":"Warning","variant":"warning","size":"medium"}
{"id":"btn4","type":"button","text":"Disabled","variant":"secondary","size":"medium","disabled":true}
{"id":"progress_section","type":"vstack","children":["progress1","progress2","progress3"],"h_align":"center"}
{"id":"progress1","type":"progress_bar","width":300,"height":20,"progress":0.25,"fill_color":"#ff4444","show_text":true}
{"id":"progress2","type":"progress_bar","width":300,"height":20,"progress":0.65,"fill_color":"#44ff44","show_text":true}
{"id":"progress3","type":"progress_bar","width":300,"height":20,"progress":0.90,"fill_color":"#4444ff","show_text":true}
{"id":"alerts_section","type":"vstack","children":["alert1","alert2","alert3","alert4"],"h_align":"left"}
{"id":"alert1","type":"alert","alert-type":"success","title":"Success Alert","message":"Everything is working perfectly! Your custom components are rendering correctly.","width":500,"show_icon":true}
{"id":"alert2","type":"alert","alert-type":"warning","title":"Warning Alert","message":"This is a warning message to demonstrate the warning alert style.","width":500,"show_icon":true}
{"id":"alert3","type":"alert","alert-type":"error","title":"Error Alert","message":"This shows how error messages would appear in your application.","width":500,"show_icon":true}
{"id":"alert4","type":"alert","alert-type":"info","title":"Info Alert","message":"Here's some informational content using the info alert component.","width":500,"show_icon":true}
"##;

    // Parse the JSON Lines
    let mut parser = parser::JsonLinesParser::new();
    let root_id = parser.parse_string(jsonl_input)?;

    // Create a fresh builder for parsing
    let mut parse_builder = DiagramBuilder::new();
    parse_builder.set_measure_text_fn(measure_text_svg_character_advance);

    // Register components with the parse builder
    parse_builder.register_custom_component("badge", create_badge_component);
    parse_builder.register_custom_component("alert", create_alert_component);
    parse_builder.register_custom_component("progress_bar", create_progress_bar_component);
    parse_builder.register_custom_component("button", create_button_component);

    // Build the diagram
    let diagram = parser.build(&root_id, &mut parse_builder)?;
    println!("✅ JSON Lines parsing successful!");

    // Calculate layout
    println!("📐 Calculating layout...");
    layout::layout_tree_node(&mut parse_builder, &diagram);
    println!("✅ Layout calculation complete!");

    // Demo 3: Render to SVG
    println!("\n=== Demo 3: SVG Rendering ===");

    let temp_dir = std::env::temp_dir();
    let mut svg_path = temp_dir.clone();
    svg_path.push("custom-components-showcase.svg");

    let svg_renderer = svg_renderer::SVGRenderer {};
    let mut svg_file = File::create(&svg_path)?;
    svg_renderer.render(&parse_builder, &diagram, &mut svg_file)?;

    println!("✅ SVG rendered successfully!");
    println!("📄 File saved to: {}", svg_path.to_str().unwrap());

    // Demo 4: Statistics
    println!("\n=== Demo 4: Component Statistics ===");
    let registered_types = parse_builder.get_custom_component_types();
    println!("📊 Registered custom components: {:?}", registered_types);
    println!("🔢 Total custom components: {}", registered_types.len());

    // Count components in the diagram
    fn count_custom_components(
        node: &diagram_builder::DiagramTreeNode,
        types: &[&String],
    ) -> usize {
        let mut count = 0;
        if types
            .iter()
            .any(|t| t.as_str() == format!("{:?}", node.entity_type))
        {
            count += 1;
        }
        for child in &node.children {
            count += count_custom_components(child, types);
        }
        count
    }

    let custom_count = count_custom_components(&diagram, &registered_types);
    println!("🎯 Custom components in diagram: {}", custom_count);

    println!("\n🎉 Custom Component Demo Complete!");
    println!("💡 Check the generated SVG file to see your custom components in action!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_component() {
        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(measure_text_svg_character_advance);
        builder.register_custom_component("badge", create_badge_component);

        let attrs = json!({
            "text": "Test",
            "background": "blue"
        })
        .as_object()
        .unwrap()
        .clone();

        let result = builder.create_custom_component("badge", &attrs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_components_registration() {
        let mut builder = DiagramBuilder::new();
        builder.register_custom_component("badge", create_badge_component);
        builder.register_custom_component("alert", create_alert_component);
        builder.register_custom_component("progress_bar", create_progress_bar_component);
        builder.register_custom_component("button", create_button_component);

        let types = builder.get_custom_component_types();
        assert_eq!(types.len(), 4);
        assert!(builder.has_custom_component("badge"));
        assert!(builder.has_custom_component("alert"));
        assert!(builder.has_custom_component("progress_bar"));
        assert!(builder.has_custom_component("button"));
    }

    #[test]
    fn test_jsonl_with_custom_components() {
        let input = r#"
{"id":"root","type":"badge","text":"Test Badge","background":"red"}
"#;

        let mut parser = parser::JsonLinesParser::new();
        let root_id = parser.parse_string(input).unwrap();

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(measure_text_svg_character_advance);
        builder.register_custom_component("badge", create_badge_component);

        let result = parser.build(&root_id, &mut builder);
        assert!(result.is_ok());
    }
}
