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
use anyhow::{bail, Result};
use serde_json::{from_value, Map, Value};
use uuid::fmt::Simple;
use uuid::uuid;

// Structure to represent an item with optional children
#[derive(Clone, Debug)]
pub struct BranchItem {
    pub name: String,
    pub children: Vec<BranchItem>,
}

impl BranchItem {
    pub fn new(name: String) -> Self {
        BranchItem {
            name,
            children: Vec::new(),
        }
    }

    pub fn with_children(name: String, children: Vec<BranchItem>) -> Self {
        BranchItem { name, children }
    }
}

pub fn create_ishikawa(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode> {
    // Parse attributes
    let problem = get_string_attr(attrs, &["problem"], "");

    // IDs
    let spine_id = format!("{}_spine", id);
    let head_id = format!("{}_head", id);

    // 1. Create spine line (700px width, centered at y=50)
    let spine = builder.new_line(
        spine_id.clone(),
        (0.0, 50.0),   // start point
        (700.0, 50.0), // end point
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        },
    );

    let head_text = builder.new_text(format!("{}_head_text", id), &problem, TextOptions::new());

    // 2. Create head box with text (box = rect + text centered)
    let head = builder.new_box(
        head_id.clone(),
        head_text,
        BoxOptions {
            padding: 10.0,
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Fixed(60.0),
            border_radius: 0.0,
            ..Default::default()
        },
    );

    // Test branch with nested items
    let left_items = vec![
        BranchItem::with_children(
            "item1".to_string(),
            vec![
                BranchItem::new("subitem1".to_string()),
                BranchItem::new("subitem2".to_string()),
            ],
        ),
        BranchItem::new("item2".to_string()),
    ];

    let right_items = vec![
        BranchItem::new("item3".to_string()),
        BranchItem::with_children(
            "item4".to_string(),
            vec![BranchItem::new("subitem3".to_string()),
            BranchItem::new("subitem4".to_string()),
            BranchItem::new("subitem5".to_string()),
            BranchItem::new("subitem6".to_string()),
            
            ],
        ),
    ];

    let branch = create_branch(
        format!("test_branch_{}", id).as_str(),
        "categoria",
        left_items,
        right_items,
        builder,
    )
    .unwrap();

    // 3. Create children with positions
    let children_with_pos = vec![
        (spine.clone(), Some(Point::new(0.0, 0.0))),
        (head.clone(), Some(Point::new(700.0, 20.0))), // x=700 (end of line), y=20 (50-30 to center)
        (branch.clone(), None),
    ];

    let constraints = vec![SimpleConstraint::Above(branch.entity_id.clone(), spine_id)];

    // 4. Create container (no constraints needed, box handles text centering)
    let container = builder.new_constraint_layout_container(
        id.to_string(),
        children_with_pos,
        constraints,
    );

    Ok(container)
}

/// Creates a branch for a fishbone category
/// Returns a node with:
/// - A vertical line in the center
/// - A box with the category name at the top
/// - Left column of items (to the left of the line)
/// - Right column of items (to the right of the line)
fn create_branch(
    id: &str,
    category_name: &str,
    left_items: Vec<BranchItem>,
    right_items: Vec<BranchItem>,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    // IDs
    let line_id = format!("{}_line", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);

    // 1. Create header box with category name
    let header_text = builder.new_text(
        format!("{}_header_text", id),
        category_name,
        TextOptions {
            font_size: 14.0,
            text_color: "black".to_string(),
            ..Default::default()
        },
    );

    let header = builder.new_box(
        header_id.clone(),
        header_text,
        BoxOptions {
            padding: 5.0,
            width_behavior: SizeBehavior::Fixed(80.0),
            height_behavior: SizeBehavior::Content,
            stroke_width: 1.0,
            ..Default::default()
        },
    );

    // 2. Create left column items (recursive)
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node = create_left_item(
            &format!("{}_left_{}", id, i),
            item,
            builder,
        )?;
        left_nodes.push(item_node);
    }

    // 3. Create right column items (recursive)
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node = create_right_item(
            &format!("{}_right_{}", id, i),
            item,
            builder,
        )?;
        right_nodes.push(item_node);
    }

    // Calculate line height based on columns
    let item_height = 30.0; // Approximate height per item
    let left_count = count_items(&left_items);
    let right_count = count_items(&right_items);
    let max_items = left_count.max(right_count);
    let line_height = (max_items as f32 * item_height).max(100.0);

    // 4. Create vertical line
    let line = builder.new_line(
        line_id.clone(),
        (0.0, 0.0),
        (0.0, line_height),
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        },
    );

    // 5. Create left and right vstacks
    let left_col = builder.new_vstack(
        left_col_id.clone(),
        left_nodes,
        HorizontalAlignment::Right, // Align to the right (closer to line)
    );

    let right_col = builder.new_vstack(
        right_col_id.clone(),
        right_nodes,
        HorizontalAlignment::Left, // Align to the left (closer to line)
    );

    // 6. Create constraint container to position everything
    let children_with_pos = vec![
        (line.clone(), Some(Point::new(0.0, 30.0))), // Line starts below header
        (header.clone(), None),                      // Header will be positioned by constraints
        (left_col.clone(), None), // Left column will be positioned by constraints
        (right_col.clone(), None), // Right column will be positioned by constraints
    ];

    let constraints = vec![
        // Header centered above the line
        SimpleConstraint::AlignCenterHorizontal(vec![line_id.clone(), header_id.clone()]),
        SimpleConstraint::Above(header_id.clone(), line_id.clone()),
        SimpleConstraint::VerticalSpacing(header_id.clone(), line_id.clone(), 5.0),
        // Left column to the left of the line
        SimpleConstraint::LeftOf(left_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(left_col_id.clone(), line_id.clone(), 10.0),
        SimpleConstraint::AlignTop(vec![line_id.clone(), left_col_id.clone()]),
        // Right column to the right of the line
        SimpleConstraint::RightOf(right_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(line_id.clone(), right_col_id.clone(), 10.0),
        SimpleConstraint::AlignTop(vec![line_id.clone(), right_col_id.clone()]),
    ];

    let branch =
        builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);

    Ok(branch)
}

/// Creates a left column item (children first, then text)
/// Layout: [children_vstack] [text]
fn create_left_item(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    
    if item.children.is_empty() {
        // Leaf node: just create a simple box
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 11.0,
                text_color: "black".to_string(),
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            id.to_string(),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(80.0),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        // Has children: create hstack with [children_vstack, text]
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node = create_left_item(&format!("{}_child_{}", id, i), child, builder)?;
            children_nodes.push(child_node);
        }

        // Create vstack for children
        let children_vstack = builder.new_vstack(
            format!("{}_children", id),
            children_nodes,
            HorizontalAlignment::Right,
        );

        // Create text for this item
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 11.0,
                text_color: "black".to_string(),
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            format!("{}_box", id),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(80.0),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        // Create hstack: [children_vstack, text]
        let hstack = builder.new_hstack(
            id.to_string(),
            vec![children_vstack, item_box],
            VerticalAlignment::Center,
        );

        Ok(hstack)
    }
}

/// Creates a right column item (text first, then children)
/// Layout: [text] [children_vstack]
fn create_right_item(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    
    if item.children.is_empty() {
        // Leaf node: just create a simple box
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 11.0,
                text_color: "black".to_string(),
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            id.to_string(),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(80.0),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        // Has children: create hstack with [text, children_vstack]
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node = create_right_item(&format!("{}_child_{}", id, i), child, builder)?;
            children_nodes.push(child_node);
        }

        // Create vstack for children
        let children_vstack = builder.new_vstack(
            format!("{}_children", id),
            children_nodes,
            HorizontalAlignment::Left,
        );

        // Create text for this item
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 11.0,
                text_color: "black".to_string(),
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            format!("{}_box", id),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(80.0),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        // Create hstack: [text, children_vstack]
        let hstack = builder.new_hstack(
            id.to_string(),
            vec![item_box, children_vstack],
            VerticalAlignment::Center,
        );

        Ok(hstack)
    }
}

/// Helper function to count total items (including nested children)
fn count_items(items: &[BranchItem]) -> usize {
    items.iter().map(|item| {
        1 + count_items(&item.children)
    }).sum()
}

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    println!("📄 Diagram components registered");
}