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
    let problem = get_string_attr(attrs, &["problem"], "..");

    // IDs
    let spine_id = format!("{}_spine", id);
    let head_id = format!("{}_head", id);

    // 1. Create spine as a rectangle that can grow
    let spine = builder.new_rectangle(
        spine_id.clone(),
        RectOptions {
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Fixed(2.0),
            fill_color: Fill::Color("black".to_string()),
            stroke_width: 0.0,
            ..Default::default()
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
                        BranchItem::new("Falta cursos..".to_string()),
                        BranchItem::new("Calidad cursos regular".to_string()),
                    ],
                ),
                BranchItem::new("Alta rotación".to_string()),
                BranchItem::new("Otra causa".to_string()),
            ],
            vec![
                BranchItem::new("Bajos salarios".to_string()),
                // BranchItem::new("Falta motivación".to_string()),
            ],
        ),
        // Category::new(
        //     "Procesos".to_string(),
        //     vec![BranchItem::new("Documentación incompleta".to_string())],
        //     vec![BranchItem::new("Falta de revisiones".to_string())],
        // ),
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

    // Track branch IDs separately
    let mut top_branch_ids = Vec::new();
    let mut bottom_branch_ids = Vec::new();

    // Top branches
    for (i, category) in top_categories.iter().enumerate() {
        let branch = create_top_branch(
            &format!("{}_top_{}", id, i),
            &category.name,
            category.left_items.clone(),
            category.right_items.clone(),
            builder,
        )?;

        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, None));
        top_branch_ids.push(branch_id.clone());

        constraints.push(SimpleConstraint::Above(branch_id.clone(), spine_id.clone()));
    }

    // Bottom branches
    for (i, category) in bottom_categories.iter().enumerate() {
        let branch = create_bottom_branch(
            &format!("{}_bottom_{}", id, i),
            &category.name,
            category.left_items.clone(),
            category.right_items.clone(),
            builder,
        )?;

        let branch_id = branch.entity_id.clone();
        all_branches.push((branch, None));
        bottom_branch_ids.push(branch_id.clone());

        // Vertical constraint: branch below spine
        constraints.push(SimpleConstraint::Below(branch_id.clone(), spine_id.clone()));
    }

    // Distribuir horizontalmente las ramas SUPERIORES
    if !top_branch_ids.is_empty() {
        // Primera rama superior alineada con el derecho de la espina
        constraints.push(SimpleConstraint::AlignRight(vec![
            spine_id.clone(),
            top_branch_ids[0].clone(),
        ]));

        // Espaciar ramas superiores entre sí
        if top_branch_ids.len() > 1 {
            let spacing = 80.0;
            for i in 1..top_branch_ids.len() {
                constraints.push(SimpleConstraint::RightOf(
                    top_branch_ids[i].clone(),
                    top_branch_ids[i - 1].clone(),
                ));
                // constraints.push(SimpleConstraint::HorizontalSpacing(
                //     top_branch_ids[i - 1].clone(),
                //     top_branch_ids[i].clone(),
                //     spacing,
                // ));
            }
        }
    }

    // Distribuir horizontalmente las ramas INFERIORES
    if !bottom_branch_ids.is_empty() {
        // Primera rama inferior alineada con el inicio de la espina
        constraints.push(SimpleConstraint::AlignLeft(vec![
            spine_id.clone(),
            bottom_branch_ids[0].clone(),
        ]));

        // Espaciar ramas inferiores entre sí
        if bottom_branch_ids.len() > 1 {
            let spacing = 80.0;
            for i in 1..bottom_branch_ids.len() {
                constraints.push(SimpleConstraint::RightOf(
                    bottom_branch_ids[i].clone(),
                    bottom_branch_ids[i - 1].clone(),
                ));
                // constraints.push(SimpleConstraint::HorizontalSpacing(
                //     bottom_branch_ids[i - 1].clone(),
                //     bottom_branch_ids[i].clone(),
                //     spacing,
                // ));
            }
        }
    }

    // La espina termina donde empieza la cabeza
    constraints.push(SimpleConstraint::LeftOf(spine_id.clone(), head_id.clone()));

    constraints.push(SimpleConstraint::AlignCenterVertical(vec![
        head_id.clone(),
        spine_id.clone(),
    ]));

    // Última rama superior debe estar a la izquierda de la cabeza
    if !top_branch_ids.is_empty() {
        let last_top = &top_branch_ids[top_branch_ids.len() - 1];
        constraints.push(SimpleConstraint::LeftOf(last_top.clone(), head_id.clone()));
        // constraints.push(SimpleConstraint::HorizontalSpacing(
        //     last_top.clone(),
        //     head_id.clone(),
        //     50.0,
        // ));
    }

    // Última rama inferior debe estar a la izquierda de la cabeza
    if !bottom_branch_ids.is_empty() {
        let last_bottom = &bottom_branch_ids[bottom_branch_ids.len() - 1];
        constraints.push(SimpleConstraint::LeftOf(
            last_bottom.clone(),
            head_id.clone(),
        ));
    }

    // 3. Create children with positions
    let mut children_with_pos = vec![(spine.clone(), None), (head.clone(), None)];
    children_with_pos.extend(all_branches);

    // Head positioned at the end of spine
    constraints.push(SimpleConstraint::RightOf(head_id.clone(), spine_id.clone()));
    constraints.push(SimpleConstraint::AlignCenterVertical(vec![
        spine_id.clone(),
        head_id.clone(),
    ]));

    constraints.push(SimpleConstraint::FixedWidth(spine_id.clone(), 600.0));

    // 4. Create container with constraints
    let container =
        builder.new_constraint_layout_container(id.to_string(), children_with_pos, constraints);

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
    let line_start_id = format!("{}_line_start", id);
    let line_end_id = format!("{}_line_end", id);
    let header_id = format!("{}_header", id);
    let left_col_id = format!("{}_left_col", id);
    let right_col_id = format!("{}_right_col", id);
    let spacer_rect_id = format!("{}_vertical_spacer", id);

    let spacer_rect = builder.new_rectangle(
        spacer_rect_id.clone(),
        RectOptions {
            width_behavior: SizeBehavior::Fixed(20.0),
            height_behavior: SizeBehavior::Content,
            fill_color: Fill::Color("transparent".to_owned()),
            stroke_color: "transparent".to_owned(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    );

    let start_point = builder.new_point(line_start_id.clone());
    let end_point = builder.new_point(line_end_id.clone());
    // 1. Create vertical line as a rectangle (going UP)
    let line = builder.new_line(
        line_id,
        LinePointReference::PointID(line_start_id.clone()),
        LinePointReference::PointID(line_end_id.clone()),
        LineOptions {
            stroke_color: BODY_COLOR.to_owned(),
            stroke_width: 1.0,
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

    let left_col_top_spacer = builder.new_spacer(
        format!("{}_left_padding", id.clone()),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_top_spacer = builder.new_spacer(
        format!("{}_right_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );
    let left_col_bottom_spacer = builder.new_spacer(
        format!("{}_left_padding", id.clone()),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_bottom_spacer = builder.new_spacer(
        format!("{}_right_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    // 3. Create left column items
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node = create_left_item(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }

    left_nodes.insert(0, left_col_top_spacer);
    left_nodes.push(left_col_bottom_spacer);

    // 4. Create right column items
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node = create_right_item(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    right_nodes.insert(0, right_col_top_spacer);
    right_nodes.push(right_col_bottom_spacer);

    // 5. Create vstacks
    let left_col = builder.new_vstack(left_col_id.clone(), left_nodes, HorizontalAlignment::Right);
    let right_col =
        builder.new_vstack(right_col_id.clone(), right_nodes, HorizontalAlignment::Left);

    // 6. Create constraint container
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
        // Minimum heights
        SimpleConstraint::MinHeight(left_col_id.clone(), 50.0),
        SimpleConstraint::MinHeight(spacer_rect_id.clone(), 50.0),
        // Spacer grows to match the tallest column (FIXED: spacer is first)
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), left_col_id.clone()]),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), right_col_id.clone()]),
        // Align tops so all three start at the same y
        SimpleConstraint::AlignTop(vec![
            left_col_id.clone(),
            spacer_rect_id.clone(),
            right_col_id.clone(),
        ]),
        // Align bottoms so all three end at the same y (same height)
        SimpleConstraint::AlignBottom(vec![
            spacer_rect_id.clone(),
            left_col_id.clone(),
            right_col_id.clone(),
        ]),
        // Horizontal positioning: left_col | spacer | right_col
        SimpleConstraint::LeftOf(left_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::RightOf(right_col_id.clone(), spacer_rect_id.clone()),
        // Header above the columns/spacer
        SimpleConstraint::Above(header_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::AlignCenterHorizontal(vec![header_id.clone(), spacer_rect_id.clone()]),
        // Line spans the full height of the spacer
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

/// Creates a top branch (line goes UP, text above the line)
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
            width_behavior: SizeBehavior::Fixed(20.0),
            height_behavior: SizeBehavior::Content,
            fill_color: Fill::Color("transparent".to_owned()),
            stroke_color: "transparent".to_owned(),
            stroke_width: 1.0,
            border_radius: 0.0,
        },
    );

    let start_point = builder.new_point(line_start_id.clone());
    let end_point = builder.new_point(line_end_id.clone());
    // 1. Create vertical line as a rectangle (going UP)
    let line = builder.new_line(
        line_id,
        LinePointReference::PointID(line_start_id.clone()),
        LinePointReference::PointID(line_end_id.clone()),
        LineOptions {
            stroke_color: BODY_COLOR.to_owned(),
            stroke_width: 1.0,
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

    let left_col_top_spacer = builder.new_spacer(
        format!("{}_left_padding", id.clone()),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_top_spacer = builder.new_spacer(
        format!("{}_right_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );
    let left_col_bottom_spacer = builder.new_spacer(
        format!("{}_left_padding", id.clone()),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    let right_col_bottom_spacer = builder.new_spacer(
        format!("{}_right_padding", id),
        SpacerOptions {
            width: 0.0,
            height: 30.0,
            direction: SpacerDirection::Vertical,
        },
    );

    // 3. Create left column items
    let mut left_nodes = Vec::new();
    for (i, item) in left_items.iter().enumerate() {
        let item_node = create_left_item(&format!("{}_left_{}", id, i), item, builder)?;
        left_nodes.push(item_node);
    }
    left_nodes.insert(0, left_col_top_spacer);
    left_nodes.push(left_col_bottom_spacer);

    // 4. Create right column items
    let mut right_nodes = Vec::new();
    for (i, item) in right_items.iter().enumerate() {
        let item_node = create_right_item(&format!("{}_right_{}", id, i), item, builder)?;
        right_nodes.push(item_node);
    }

    right_nodes.insert(0, right_col_top_spacer);
    left_nodes.push(right_col_bottom_spacer);

    // 5. Create vstacks
    let left_col = builder.new_vstack(left_col_id.clone(), left_nodes, HorizontalAlignment::Right);
    let right_col =
        builder.new_vstack(right_col_id.clone(), right_nodes, HorizontalAlignment::Left);

    // 6. Create constraint container
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
        // Minimum heights
        SimpleConstraint::MinHeight(left_col_id.clone(), 50.0),
        SimpleConstraint::MinHeight(spacer_rect_id.clone(), 50.0),
        // Spacer grows to match the tallest column (FIXED: spacer is first)
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), left_col_id.clone()]),
        SimpleConstraint::AtLeastSameHeight(vec![spacer_rect_id.clone(), right_col_id.clone()]),
        // Align tops so all three start at the same y
        SimpleConstraint::AlignBottom(vec![
            left_col_id.clone(),
            spacer_rect_id.clone(),
            right_col_id.clone(),
        ]),
        // Align bottoms so all three end at the same y (same height)
        SimpleConstraint::AlignTop(vec![
            spacer_rect_id.clone(),
            left_col_id.clone(),
            right_col_id.clone(),
        ]),
        // Horizontal positioning: left_col | spacer | right_col
        SimpleConstraint::LeftOf(left_col_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::RightOf(right_col_id.clone(), spacer_rect_id.clone()),
        // Header above the columns/spacer
        SimpleConstraint::Below(header_id.clone(), spacer_rect_id.clone()),
        SimpleConstraint::AlignCenterHorizontal(vec![header_id.clone(), spacer_rect_id.clone()]),
        // Line spans the full height of the spacer
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
                stroke_width: 0.0,
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
                stroke_width: 0.0,
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
                stroke_width: 0.0,
                stroke_color:"transparent".to_string(),
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
                stroke_width: 0.0,
                stroke_color:"transparent".to_string(),
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
