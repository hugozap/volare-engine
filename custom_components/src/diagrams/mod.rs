use std::fmt::format;

use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use crate::document::style::{
    BG_ACCENT, BG_PRIMARY, BG_SECONDARY, BORDER_LIGHT_COLOR, BORDER_STRONG_COLOR,
    DOCUMENT_WIDTH_DEFAULT, FONT_SANS, FONT_WEIGHT_BOLD_LIGHT, FONT_WEIGHT_BOLD_MAX,
    FONT_WEIGHT_BOLD_MD, FONT_WEIGHT_NORMAL, LINE_HEIGHT_NORMAL, LINE_HEIGHT_RELAXED,
    LINE_HEIGHT_TIGHT, MUTED_TEXT, PADDING_NORMAL, PRIMARY_TEXT, SECONDARY_TEXT, SPACE_3XL,
    SPACE_MD, SPACE_SM, SPACE_XS, TEXT_2XL, TEXT_3XL, TEXT_BASE, TEXT_LG, TEXT_XL, TEXT_XS,
    WIDTH_FULL, WIDTH_LG, WIDTH_MD, WIDTH_PROPERTY_PANEL, WIDTH_SM, WIDTH_XL,
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
const CONNECTOR_GAP: f32 = 20.0; // Gap between item box and connector line

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
    pub items: Vec<BranchItem>,
}

impl Category {
    pub fn new(name: String, items: Vec<BranchItem>) -> Self {
        Category { name, items }
    }
}
pub fn create_ishikawa(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode> {
    // Parse attributes
    let problem = get_string_attr(attrs, &["problem"], "..");
    let problem = get_string_attr(attrs, &["problem"], "..");

    // Parse categories from attributes
    let categories_value = attrs.get("categories");
    let mut categories: Vec<Category> = Vec::new();

    if let Some(Value::Array(categories_json)) = categories_value {
        for cat_value in categories_json {
            if let Value::Object(cat_obj) = cat_value {
                let name = get_string_attr(&cat_obj, &["name"], "");

                let items_json = if let Some(Value::Array(items)) = cat_obj.get("items") {
                    items.clone()
                } else {
                    Vec::new()
                };

                let items = parse_branch_items(&items_json)?;

                categories.push(Category { name, items });
            }
        }
    }

    // IDs
    let spine_id = format!("{}_spine", id);
    let head_id = format!("{}_head", id);

    let spine_start_point_id = format!("{}_spine_start", id);
    let spine_end_point_id = format!("{}_spine_end", id);

    let spine_start = builder.new_point(spine_start_point_id.clone());
    let spine_end = builder.new_point(spine_end_point_id.clone());

    // 1. Create spine as a rectangle that can grow
    let spine = builder.new_line(
        spine_id.clone(),
        LinePointReference::PointID(spine_start_point_id.clone()),
        LinePointReference::PointID(spine_end_point_id.clone()),
        LineOptions {
            stroke_color: BODY_COLOR.to_string(),
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

    // Automatically distribute categories between top and bottom
    let (top_categories, bottom_categories) = distribute_categories(categories);

    // Create branches
    let mut all_branches = Vec::new();
    let mut constraints = Vec::new();

    // Track branch IDs separately
    let mut top_branch_ids = Vec::new();
    let mut bottom_branch_ids = Vec::new();

    // constraints.push(SimpleConstraint::AlignLeft(vec![
    //     head_id.clone(),              // Reference (first)
    //     spine_start_point_id.clone(), // Aligns to reference
    // ]));

    constraints.push(SimpleConstraint::FixedWidth(
        spine_start_point_id.clone(),
        0.0,
    ));
    constraints.push(SimpleConstraint::FixedWidth(
        spine_end_point_id.clone(),
        0.0,
    ));

    constraints.push(SimpleConstraint::AlignLeft(vec![
        head_id.clone(),
        spine_start_point_id.clone(),
    ]));
// Top branches
    for (i, category) in top_categories.iter().enumerate() {
        // Automatically distribute items between left and right
        let (left_items, right_items) = distribute_items(category.items.clone());
        
        let branch = create_top_branch(
            &format!("{}_top_{}", id, i),
            &category.name,
            left_items,
            right_items,
            builder,
        )?;

        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, None));
        top_branch_ids.push(branch_id.clone());

        constraints.push(SimpleConstraint::Above(
            branch_id.clone(),
            spine_start_point_id.clone(),
        ));
    }

    // Bottom branches
    for (i, category) in bottom_categories.iter().enumerate() {
        // Automatically distribute items between left and right
        let (left_items, right_items) = distribute_items(category.items.clone());
        
        let branch = create_bottom_branch(
            &format!("{}_bottom_{}", id, i),
            &category.name,
            left_items,
            right_items,
            builder,
        )?;

        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, None));
        bottom_branch_ids.push(branch_id.clone());

        // Vertical constraint: branch below spine
        constraints.push(SimpleConstraint::Below(
            branch_id.clone(),
            spine_start_point_id.clone(),
        ));
    }
    // Distribuir horizontalmente las ramas SUPERIORES
    if !top_branch_ids.is_empty() {
        // Primera rama superior alineada con el derecho de la espina
        // constraints.push(SimpleConstraint::AlignRight(vec![
        //     spine_id.clone(),
        //     top_branch_ids.last().unwrap().to_owned(),
        // ]));

        // Espaciar ramas superiores entre sí
        if top_branch_ids.len() > 1 {
            let spacing = 80.0;
            for i in 1..top_branch_ids.len() {
                // constraints.push(SimpleConstraint::RightOf(
                //     top_branch_ids[i].clone(),
                //     top_branch_ids[i - 1].clone(),
                // ));
                constraints.push(SimpleConstraint::HorizontalSpacing(
                    top_branch_ids[i - 1].clone(),
                    top_branch_ids[i].clone(),
                    spacing,
                ));
            }
        }
    }

    // Distribuir horizontalmente las ramas INFERIORES
    if !bottom_branch_ids.is_empty() {
        // Primera rama inferior alineada con el inicio de la espina
        // constraints.push(SimpleConstraint::AlignLeft(vec![
        //     spine_id.clone(),
        //     bottom_branch_ids.last().unwrap().to_owned(),

        // ]));

        // constraints.push(SimpleConstraint::AlignLeft(vec![
        //     spine_id.clone(),
        //     bottom_branch_ids[0].clone(), // ✅ FIRST branch
        // ]));

        // constraints.push(SimpleConstraint::HorizontalSpacing(
        //     bottom_branch_ids.last().unwrap().to_string(),
        //     head_id.clone(),
        //     50.0,
        // ));

        // Espaciar ramas inferiores entre sí
        if bottom_branch_ids.len() > 1 {
            let spacing = 80.0;
            for i in 1..bottom_branch_ids.len() {
                // constraints.push(SimpleConstraint::RightOf(
                //     bottom_branch_ids[i].clone(),
                //     bottom_branch_ids[i - 1].clone(),
                // ));
                constraints.push(SimpleConstraint::HorizontalSpacing(
                    bottom_branch_ids[i - 1].clone(),
                    bottom_branch_ids[i].clone(),
                    spacing,
                ));
            }
        }
    }

    constraints.push(SimpleConstraint::AlignCenterVertical(vec![
        head_id.clone(),
        spine_start_point_id.clone(),
        spine_end_point_id.clone(),
    ]));

    // Última rama superior debe estar a la izquierda de la cabeza
    if !top_branch_ids.is_empty() {
        let last_top = &top_branch_ids.last().unwrap();
        // constraints.push(SimpleConstraint::LeftOf(last_top.clone(), head_id.clone()));
        // constraints.push(SimpleConstraint::HorizontalSpacing(
        //     last_top.to_string(),
        //     head_id.clone(),
        //     50.0,
        // ));
    }

    // Última rama inferior debe estar a la izquierda de la cabeza
    if !bottom_branch_ids.is_empty() {
        let last_bottom = &bottom_branch_ids.last().unwrap();
        constraints.push(SimpleConstraint::HorizontalSpacing(
            last_bottom.to_string(),
            head_id.clone(),
            50.0,
        ));
    }

    // Position spine end point - align with the leftmost first branch
    let mut leftmost_branches = vec![spine_end_point_id.clone()];
    if !top_branch_ids.is_empty() {
        leftmost_branches.push(top_branch_ids[0].clone());
    }
    if !bottom_branch_ids.is_empty() {
        leftmost_branches.push(bottom_branch_ids[0].clone());
    }

    if leftmost_branches.len() > 1 {
        constraints.push(SimpleConstraint::AlignLeft(leftmost_branches));
    }

    // 3. Create children with positions
    let mut children_with_pos = vec![
        (spine.clone(), None),
        (spine_start.clone(), None),
        (spine_end.clone(), None),
        (head.clone(), None),
    ];
    children_with_pos.extend(all_branches);

    // 4. Create container with constraints
    let container =
        builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);

    Ok(container)
}

/// Automatically distribute items between left and right columns
fn distribute_items(items: Vec<BranchItem>) -> (Vec<BranchItem>, Vec<BranchItem>) {
    let total = items.len();
    let mid = (total + 1) / 2; // Round up for left side

    let mut left_items = Vec::new();
    let mut right_items = Vec::new();

    for (i, item) in items.into_iter().enumerate() {
        if i < mid {
            left_items.push(item);
        } else {
            right_items.push(item);
        }
    }

    (left_items, right_items)
}

/// Automatically distribute categories between top and bottom branches
fn distribute_categories(categories: Vec<Category>) -> (Vec<Category>, Vec<Category>) {
    let total = categories.len();
    let mid = (total + 1) / 2; // Round up for top

    let mut top_categories = Vec::new();
    let mut bottom_categories = Vec::new();

    for (i, category) in categories.into_iter().enumerate() {
        if i < mid {
            top_categories.push(category);
        } else {
            bottom_categories.push(category);
        }
    }

    (top_categories, bottom_categories)
}

/// Recursively parse branch items from JSON
fn parse_branch_items(items_json: &[Value]) -> Result<Vec<BranchItem>> {
    let mut items = Vec::new();
    
    for item_value in items_json {
        if let Value::Object(item_obj) = item_value {
            let name = get_string_attr(&item_obj, &["name"], "");
            
            // Parse children array
            let children = if let Some(Value::Array(children_json)) = item_obj.get("children") {
                parse_branch_items(children_json)?
            } else {
                Vec::new()
            };
            
            items.push(BranchItem {
                name,
                children,
            });
        }
    }
    
    Ok(items)
}

/// Creates a top branch with connector lines
fn create_top_branch(
    id: &str,
    category_name: &str,
    left_items: Vec<BranchItem>,
    right_items: Vec<BranchItem>,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let line_id = format!("{}_line", id);
    let line_start_id = format!("{}_line_start", id);
    let line_end_id = format!("{}_line_end", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);
    let spacer_rect_id = format!("{}_vertical_spacer", id);

    let spacer_rect = builder.new_rectangle(
        spacer_rect_id.clone(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(0.0),
            height_behavior: SizeBehavior::Fixed(150.0),
            fill_color: Fill::Color("transparent".to_owned()),
            stroke_color: "transparent".to_owned(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    );

    let start_point = builder.new_point(line_start_id.clone());
    let end_point = builder.new_point(line_end_id.clone());

    let line = builder.new_line(
        line_id,
        LinePointReference::PointID(line_start_id.clone()),
        LinePointReference::PointID(line_end_id.clone()),
        LineOptions {
            stroke_color: BODY_COLOR.to_owned(),
            stroke_width: 1.0,
        },
    );

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

    let left_col_top_spacer = builder.new_spacer(
        format!("{}_left_top_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_top_spacer = builder.new_spacer(
        format!("{}_right_top_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let left_col_bottom_spacer = builder.new_spacer(
        format!("{}_left_bottom_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_bottom_spacer = builder.new_spacer(
        format!("{}_right_bottom_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    // Create left column items with connectors
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node =
            create_left_item_with_connector(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }

    left_nodes.insert(0, left_col_top_spacer);
    left_nodes.push(left_col_bottom_spacer);

    // Create right column items with connectors
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node =
            create_right_item_with_connector(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    right_nodes.insert(0, right_col_top_spacer);
    right_nodes.push(right_col_bottom_spacer);

    let left_col = builder.new_vstack(left_col_id.clone(), left_nodes, HorizontalAlignment::Right);
    let right_col =
        builder.new_vstack(right_col_id.clone(), right_nodes, HorizontalAlignment::Left);

    let children_with_pos = vec![
        (line.clone(), None),
        (header.clone(), None),
        (left_col.clone(), None),
        (right_col.clone(), None),
        (start_point, None),
        (end_point.clone(), None),
        (spacer_rect.clone(), None),
    ];

    let constraints = vec![
        SimpleConstraint::MinHeight(left_col_id.clone(), 50.0),
        SimpleConstraint::MinHeight(spacer_rect_id.clone(), 50.0),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), left_col_id.clone()]),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), right_col_id.clone()]),
        SimpleConstraint::AlignTop(vec![
            left_col_id.clone(),
            spacer_rect_id.clone(),
            right_col_id.clone(),
        ]),
        SimpleConstraint::AlignBottom(vec![
            spacer_rect_id.clone(),
            left_col_id.clone(),
            right_col_id.clone(),
        ]),
        SimpleConstraint::LeftOf(left_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::RightOf(right_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::Above(header_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::AlignCenterHorizontal(vec![header_id.clone(), spacer_rect_id.clone()]),
        SimpleConstraint::AlignTop(vec![spacer_rect_id.clone(), line_start_id.clone()]),
        SimpleConstraint::AlignBottom(vec![spacer_rect_id.clone(), line_end_id.clone()]),
        SimpleConstraint::AlignCenterHorizontal(vec![
            spacer_rect_id.clone(),
            line_start_id.clone(),
            line_end_id.clone(),
        ]),
    ];

    let branch =
        builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);
    Ok(branch)
}
/// Creates a bottom branch with connector lines
fn create_bottom_branch(
    id: &str,
    category_name: &str,
    left_items: Vec<BranchItem>,
    right_items: Vec<BranchItem>,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let line_id = format!("{}_line", id);
    let line_start_id = format!("{}_line_start", id);
    let line_end_id = format!("{}_line_end", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);
    let spacer_rect_id = format!("{}_vertical_spacer", id);

    let spacer_rect = builder.new_rectangle(
        spacer_rect_id.clone(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(0.0),
            height_behavior: SizeBehavior::Content,
            fill_color: Fill::Color("transparent".to_owned()),
            stroke_color: "transparent".to_owned(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    );

    let start_point = builder.new_point(line_start_id.clone());
    let end_point = builder.new_point(line_end_id.clone());

    let line = builder.new_line(
        line_id,
        LinePointReference::PointID(line_start_id.clone()),
        LinePointReference::PointID(line_end_id.clone()),
        LineOptions {
            stroke_color: BODY_COLOR.to_owned(),
            stroke_width: 1.0,
        },
    );

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

    let left_col_top_spacer = builder.new_spacer(
        format!("{}_left_top_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_top_spacer = builder.new_spacer(
        format!("{}_right_top_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let left_col_bottom_spacer = builder.new_spacer(
        format!("{}_left_bottom_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_bottom_spacer = builder.new_spacer(
        format!("{}_right_bottom_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    // Create left column items with connectors
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node =
            create_left_item_with_connector(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }
    left_nodes.insert(0, left_col_top_spacer);
    left_nodes.push(left_col_bottom_spacer);

    // Create right column items with connectors
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node =
            create_right_item_with_connector(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    right_nodes.insert(0, right_col_top_spacer);
    right_nodes.push(right_col_bottom_spacer);

    let left_col = builder.new_vstack(left_col_id.clone(), left_nodes, HorizontalAlignment::Right);
    let right_col =
        builder.new_vstack(right_col_id.clone(), right_nodes, HorizontalAlignment::Left);

    let children_with_pos = vec![
        (line.clone(), None),
        (header.clone(), None),
        (left_col.clone(), None),
        (right_col.clone(), None),
        (start_point, None),
        (end_point.clone(), None),
        (spacer_rect.clone(), None),
    ];

    let constraints = vec![
        SimpleConstraint::MinHeight(left_col_id.clone(), 50.0),
        SimpleConstraint::MinHeight(spacer_rect_id.clone(), 50.0),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), left_col_id.clone()]),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), right_col_id.clone()]),
        SimpleConstraint::AlignBottom(vec![
            left_col_id.clone(),
            spacer_rect_id.clone(),
            right_col_id.clone(),
        ]),
        SimpleConstraint::AlignTop(vec![
            spacer_rect_id.clone(),
            left_col_id.clone(),
            right_col_id.clone(),
        ]),
        SimpleConstraint::LeftOf(left_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::RightOf(right_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::Below(header_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::AlignCenterHorizontal(vec![header_id.clone(), spacer_rect_id.clone()]),
        SimpleConstraint::AlignTop(vec![spacer_rect_id.clone(), line_start_id.clone()]),
        SimpleConstraint::AlignBottom(vec![spacer_rect_id.clone(), line_end_id.clone()]),
        SimpleConstraint::AlignCenterHorizontal(vec![
            spacer_rect_id.clone(),
            line_start_id.clone(),
            line_end_id.clone(),
        ]),
    ];

    let branch =
        builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);
    Ok(branch)
}
/// Creates a left item with a horizontal connector line
fn create_left_item_with_connector(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let item_content = create_left_item(id, item, builder)?;

    // Create connector line (horizontal line pointing right)
    let connector_id = format!("{}_connector", id);
    let connector = builder.new_rectangle(
        connector_id,
        RectOptions {
            width_behavior: SizeBehavior::Fixed(CONNECTOR_GAP),
            height_behavior: SizeBehavior::Fixed(1.0),
            fill_color: Fill::Color(BODY_COLOR.to_string()),
            stroke_width: 0.0,
            ..Default::default()
        },
    );

    // Combine item and connector horizontally
    let hstack = builder.new_hstack(
        format!("{}_with_connector", id),
        vec![item_content, connector],
        VerticalAlignment::Center,
    );

    Ok(hstack)
}

/// Creates a right item with a horizontal connector line
fn create_right_item_with_connector(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let item_content = create_right_item(id, item, builder)?;

    // Create connector line (horizontal line pointing left)
    let connector_id = format!("{}_connector", id);
    let connector = builder.new_rectangle(
        connector_id,
        RectOptions {
            width_behavior: SizeBehavior::Fixed(CONNECTOR_GAP),
            height_behavior: SizeBehavior::Fixed(1.0),
            fill_color: Fill::Color(BODY_COLOR.to_string()),
            stroke_width: 0.0,
            ..Default::default()
        },
    );

    // Combine connector and item horizontally (connector first for right side)
    let hstack = builder.new_hstack(
        format!("{}_with_connector", id),
        vec![connector, item_content],
        VerticalAlignment::Center,
    );

    Ok(hstack)
}
/// Creates a left column item (children first, then text)
fn create_left_item(
    id: &str,
    item: &BranchItem,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    if item.children.is_empty() {
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
                stroke_width: 0.0,
                stroke_color: "transparent".to_string(),
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node =
                create_left_item_with_connector(&format!("{}_child_{}", id, i), child, builder)?;
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
                stroke_width: 0.0,
                stroke_color: "transparent".to_string(),
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
                stroke_width: 0.0,
                stroke_color: "transparent".to_string(),
                ..Default::default()
            },
        );

        Ok(item_box)
    } else {
        let mut children_nodes = Vec::new();
        for (i, child) in item.children.iter().enumerate() {
            let child_node =
                create_right_item_with_connector(&format!("{}_child_{}", id, i), child, builder)?;
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
                stroke_width: 0.0,
                stroke_color: "transparent".to_string(),
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
