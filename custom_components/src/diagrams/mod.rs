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

// Constants for fishbone diagram
const ITEM_BOX_WIDTH: f32 = 100.0;
const CATEGORY_BOX_WIDTH: f32 = 100.0;

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

// Structure for a category with its items
#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub left_items: Vec<BranchItem>,
    pub right_items: Vec<BranchItem>,
}

impl Category {
    pub fn new(name: String, left_items: Vec<BranchItem>, right_items: Vec<BranchItem>) -> Self {
        Category {
            name,
            left_items,
            right_items,
        }
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

    // 1. Create spine line (700px width, centered at y=200)
    let spine_y = 200.0;
    let spine = builder.new_line(
        spine_id.clone(),
        (0.0, spine_y),
        (700.0, spine_y),
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        },
    );

    let head_text = builder.new_text(format!("{}_head_text", id), &problem, TextOptions::new());

    // 2. Create head box with text
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

    // Test categories
    let top_categories = vec![
        Category::new(
            "Personas".to_string(),
            vec![
                BranchItem::with_children(
                    "Capacitación".to_string(),
                    vec![
                        BranchItem::new("Falta cursos".to_string()),
                        BranchItem::new("Sin presupuesto".to_string()),
                    ],
                ),
                BranchItem::new("Alta rotación".to_string()),
            ],
            vec![
                BranchItem::new("Bajos salarios".to_string()),
                BranchItem::new("Falta motivación".to_string()),
            ],
        ),
        Category::new(
            "Procesos".to_string(),
            vec![BranchItem::new("Documentación incompleta".to_string())],
            vec![BranchItem::new("Falta de revisiones".to_string())],
        ),
    ];

    let bottom_categories = vec![
        Category::new(
            "Tecnología".to_string(),
            vec![
                BranchItem::new("Hardware obsoleto".to_string()),
                BranchItem::with_children(
                    "Software".to_string(),
                    vec![
                        BranchItem::new("Sin parches".to_string()),
                        BranchItem::new("Versiones antiguas".to_string()),
                    ],
                ),
            ],
            vec![BranchItem::new("Falta integración".to_string())],
        ),
        Category::new(
            "Ambiente".to_string(),
            vec![BranchItem::new("Temperatura inestable".to_string())],
            vec![BranchItem::new("Humedad alta".to_string())],
        ),
    ];

    // Create branches
    let mut all_branches = Vec::new();
    let mut constraints = Vec::new();
    let spacing = 150.0;

    // Top branches - use constraints to position ABOVE the spine
    for (i, category) in top_categories.iter().enumerate() {
        let x = 100.0 + (i as f32 * spacing);
        let branch = create_top_branch(
            &format!("{}_top_{}", id, i),
            &category.name,
            category.left_items.clone(),
            category.right_items.clone(),
            builder,
        )?;
        
        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, Some(Point::new(x, 0.0)))); // Initial position
        
        // Constraint: branch should be ABOVE the spine
        constraints.push(SimpleConstraint::Above(branch_id.clone(), spine_id.clone()));
        constraints.push(SimpleConstraint::VerticalSpacing(branch_id, spine_id.clone(), 10.0));
    }

    // Bottom branches - use constraints to position BELOW the spine
    for (i, category) in bottom_categories.iter().enumerate() {
        let x = 100.0 + (i as f32 * spacing);
        let branch = create_bottom_branch(
            &format!("{}_bottom_{}", id, i),
            &category.name,
            category.left_items.clone(),
            category.right_items.clone(),
            builder,
        )?;
        
        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, Some(Point::new(x, 0.0)))); // Initial position
        
        // Constraint: branch should be BELOW the spine
        constraints.push(SimpleConstraint::Below(branch_id.clone(), spine_id.clone()));
        constraints.push(SimpleConstraint::VerticalSpacing(spine_id.clone(), branch_id, 10.0));
    }

    // 3. Create children with positions
    let mut children_with_pos = vec![
        (spine.clone(), Some(Point::new(0.0, 0.0))),
        (head.clone(), Some(Point::new(700.0, spine_y - 30.0))),
    ];
    children_with_pos.extend(all_branches);

    // 4. Create container with constraints
    let container = builder.new_constraint_layout_container(
        id.to_string(),
        children_with_pos,
        constraints, // Now we pass the constraints instead of empty vec
    );

    Ok(container)
}

/// Creates a top branch (line goes UP, text above the line)
fn create_top_branch(
    id: &str,
    category_name: &str,
    left_items: Vec<BranchItem>,
    right_items: Vec<BranchItem>,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let line_id = format!("{}_line", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);

    // Calculate line height
    let left_count = count_items(&left_items);
    let right_count = count_items(&right_items);
    let max_items = left_count.max(right_count);
    let line_height = (max_items as f32 * 30.0).max(80.0);

    // 1. Create vertical line going UP
    let line = builder.new_line(
        line_id.clone(),
        (0.0, 0.0),
        (0.0, -line_height), // Negative = going up
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        },
    );

    // 2. Create header (text above line)
    let header_text = builder.new_text(
        format!("{}_header_text", id),
        category_name,
        TextOptions {
            font_size: 12.0,
            text_color: "black".to_string(),
            line_width: CATEGORY_BOX_WIDTH as usize,
            ..Default::default()
        },
    );

    let header = builder.new_box(
        header_id.clone(),
        header_text,
        BoxOptions {
            padding: 5.0,
            width_behavior: SizeBehavior::Fixed(CATEGORY_BOX_WIDTH),
            height_behavior: SizeBehavior::Content,
            stroke_width: 1.0,
            ..Default::default()
        },
    );

    // 3. Create left column items
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node = create_left_item(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }

    // 4. Create right column items
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node = create_right_item(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    // 5. Create vstacks
    let left_col = builder.new_vstack(
        left_col_id.clone(),
        left_nodes,
        HorizontalAlignment::Right,
    );

    let right_col = builder.new_vstack(
        right_col_id.clone(),
        right_nodes,
        HorizontalAlignment::Left,
    );

    // 6. Create constraint container
    let children_with_pos = vec![
        (line.clone(), Some(Point::new(0.0, 0.0))),
        (header.clone(), None),
        (left_col.clone(), None),
        (right_col.clone(), None),
    ];

    let constraints = vec![
        // Header above the line (at the top end)
        SimpleConstraint::AlignCenterHorizontal(vec![line_id.clone(), header_id.clone()]),
        SimpleConstraint::Above(header_id.clone(), line_id.clone()),
        SimpleConstraint::VerticalSpacing(header_id.clone(), line_id.clone(), 5.0),
        // Left column to the left of line, aligned at line start (top)
        SimpleConstraint::LeftOf(left_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(left_col_id.clone(), line_id.clone(), 10.0),
        SimpleConstraint::AlignBottom(vec![line_id.clone(), left_col_id.clone()]),
        // Right column to the right of line
        SimpleConstraint::RightOf(right_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(line_id.clone(), right_col_id.clone(), 10.0),
        SimpleConstraint::AlignBottom(vec![line_id.clone(), right_col_id.clone()]),
    ];

    let branch = builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);
    Ok(branch)
}

/// Creates a bottom branch (line goes DOWN, text below the line)
fn create_bottom_branch(
    id: &str,
    category_name: &str,
    left_items: Vec<BranchItem>,
    right_items: Vec<BranchItem>,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let line_id = format!("{}_line", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);

    // Calculate line height
    let left_count = count_items(&left_items);
    let right_count = count_items(&right_items);
    let max_items = left_count.max(right_count);
    let line_height = (max_items as f32 * 30.0).max(80.0);

    // 1. Create vertical line going DOWN
    let line = builder.new_line(
        line_id.clone(),
        (0.0, 0.0),
        (0.0, line_height), // Positive = going down
        LineOptions {
            stroke_color: "black".to_string(),
            stroke_width: 2.0,
        },
    );

    // 2. Create header (text below line)
    let header_text = builder.new_text(
        format!("{}_header_text", id),
        category_name,
        TextOptions {
            font_size: 12.0,
            text_color: "black".to_string(),
            line_width: CATEGORY_BOX_WIDTH as usize,
            ..Default::default()
        },
    );

    let header = builder.new_box(
        header_id.clone(),
        header_text,
        BoxOptions {
            padding: 5.0,
            width_behavior: SizeBehavior::Fixed(CATEGORY_BOX_WIDTH),
            height_behavior: SizeBehavior::Content,
            stroke_width: 1.0,
            ..Default::default()
        },
    );

    // 3. Create left column items
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node = create_left_item(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }

    // 4. Create right column items
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node = create_right_item(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    // 5. Create vstacks
    let left_col = builder.new_vstack(
        left_col_id.clone(),
        left_nodes,
        HorizontalAlignment::Right,
    );

    let right_col = builder.new_vstack(
        right_col_id.clone(),
        right_nodes,
        HorizontalAlignment::Left,
    );

    // 6. Create constraint container
    let children_with_pos = vec![
        (line.clone(), Some(Point::new(0.0, 0.0))),
        (header.clone(), None),
        (left_col.clone(), None),
        (right_col.clone(), None),
    ];

    let constraints = vec![
        // Header below the line (at the bottom end)
        SimpleConstraint::AlignCenterHorizontal(vec![line_id.clone(), header_id.clone()]),
        SimpleConstraint::Below(header_id.clone(), line_id.clone()),
        SimpleConstraint::VerticalSpacing(line_id.clone(), header_id.clone(), 5.0),
        // Left column to the left of line, aligned at line start (top)
        SimpleConstraint::LeftOf(left_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(left_col_id.clone(), line_id.clone(), 10.0),
        SimpleConstraint::AlignTop(vec![line_id.clone(), left_col_id.clone()]),
        // Right column to the right of line
        SimpleConstraint::RightOf(right_col_id.clone(), line_id.clone()),
        SimpleConstraint::HorizontalSpacing(line_id.clone(), right_col_id.clone(), 10.0),
        SimpleConstraint::AlignTop(vec![line_id.clone(), right_col_id.clone()]),
    ];

    let branch = builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);
    Ok(branch)
}

/// Creates a left column item (children first, then text)
fn create_left_item(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    if item.children.is_empty() {
        // Leaf node
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 10.0,
                text_color: "black".to_string(),
                line_width: ITEM_BOX_WIDTH as usize,
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            id.to_string(),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(ITEM_BOX_WIDTH),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        // Has children: [children_vstack, text]
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node = create_left_item(&format!("{}_child_{}", id, i), child, builder)?;
            children_nodes.push(child_node);
        }

        let children_vstack = builder.new_vstack(
            format!("{}_children", id),
            children_nodes,
            HorizontalAlignment::Right,
        );

        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 10.0,
                text_color: "black".to_string(),
                line_width: ITEM_BOX_WIDTH as usize,
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            format!("{}_box", id),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(ITEM_BOX_WIDTH),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        let hstack = builder.new_hstack(
            id.to_string(),
            vec![children_vstack, item_box],
            VerticalAlignment::Center,
        );

        Ok(hstack)
    }
}

/// Creates a right column item (text first, then children)
fn create_right_item(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    if item.children.is_empty() {
        // Leaf node
        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 10.0,
                text_color: "black".to_string(),
                line_width: ITEM_BOX_WIDTH as usize,
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            id.to_string(),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(ITEM_BOX_WIDTH),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        // Has children: [text, children_vstack]
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node = create_right_item(&format!("{}_child_{}", id, i), child, builder)?;
            children_nodes.push(child_node);
        }

        let children_vstack = builder.new_vstack(
            format!("{}_children", id),
            children_nodes,
            HorizontalAlignment::Left,
        );

        let item_text = builder.new_text(
            format!("{}_text", id),
            &item.name,
            TextOptions {
                font_size: 10.0,
                text_color: "black".to_string(),
                line_width: ITEM_BOX_WIDTH as usize,
                ..Default::default()
            },
        );

        let item_box = builder.new_box(
            format!("{}_box", id),
            item_text,
            BoxOptions {
                padding: 3.0,
                width_behavior: SizeBehavior::Fixed(ITEM_BOX_WIDTH),
                height_behavior: SizeBehavior::Content,
                stroke_width: 1.0,
                ..Default::default()
            },
        );

        let hstack = builder.new_hstack(
            id.to_string(),
            vec![item_box, children_vstack],
            VerticalAlignment::Center,
        );

        Ok(hstack)
    }
}

/// Helper to count total items
fn count_items(items: &[BranchItem]) -> usize {
    items
        .iter()
        .map(|item| 1 + count_items(&item.children))
        .sum()
}

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    println!("📄 Diagram components registered");
}