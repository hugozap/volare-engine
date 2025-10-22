use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use crate::document::style::*;
use crate::parser::{get_array_attr, get_string_attr, JsonLinesParser};
use crate::*;
use anyhow::{bail, Result};
use serde_json::{from_value, Map, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use volare_engine_layout::transform::Transform;

// Basic activity types
#[derive(Clone, Debug)]
pub enum ActivityType {
    Normal,
    Decision,
    Merge,
    Start,
    End,
}

// Structure for an activity node
#[derive(Clone, Debug)]
pub struct Activity {
    pub id: String,
    pub label: String,
    pub activity_type: ActivityType,
}

// Structure for a flow between activities
#[derive(Clone, Debug)]
pub struct Flow {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

// Structure for a swimlane
#[derive(Clone, Debug)]
pub struct Swimlane {
    pub name: String,
    pub activities: Vec<Activity>,
}

impl ActivityType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "decision" => ActivityType::Decision,
            "merge" => ActivityType::Merge,
            "start" => ActivityType::Start,
            "end" => ActivityType::End,
            _ => ActivityType::Normal,
        }
    }
}

// Parse a single activity from JSON
fn parse_activity(activity_json: &Value) -> Result<Activity> {
    if let Value::Object(obj) = activity_json {
        let id = get_string_attr(obj, &["id"], "");
        let label = get_string_attr(obj, &["label", "name"], "");
        let type_str = get_string_attr(obj, &["type"], "normal");

        if id.is_empty() {
            bail!("Activity must have an 'id' attribute");
        }

        Ok(Activity {
            id,
            label,
            activity_type: ActivityType::from_str(&type_str),
        })
    } else {
        bail!("Activity must be an object")
    }
}

// Parse swimlanes from attributes
fn parse_swimlanes(attrs: &Map<String, Value>) -> Result<Vec<Swimlane>> {
    let swimlanes_value = attrs.get("swimlanes");
    let mut swimlanes = Vec::new();

    if let Some(Value::Array(swimlanes_json)) = swimlanes_value {
        for lane_value in swimlanes_json {
            if let Value::Object(lane_obj) = lane_value {
                let name = get_string_attr(&lane_obj, &["name"], "");

                let mut activities = Vec::new();
                if let Some(Value::Array(activities_json)) = lane_obj.get("activities") {
                    for activity_value in activities_json {
                        activities.push(parse_activity(activity_value)?);
                    }
                }

                swimlanes.push(Swimlane { name, activities });
            }
        }
    }

    Ok(swimlanes)
}

// Parse flows from attributes
fn parse_flows(attrs: &Map<String, Value>) -> Result<Vec<Flow>> {
    let flows_value = attrs.get("flows");
    let mut flows = Vec::new();

    if let Some(Value::Array(flows_json)) = flows_value {
        for flow_value in flows_json {
            if let Value::Object(flow_obj) = flow_value {
                let from = get_string_attr(&flow_obj, &["from", "source"], "");
                let to = get_string_attr(&flow_obj, &["to", "target"], "");
                let condition = {
                    let cond = get_string_attr(&flow_obj, &["condition", "label"], "");
                    if cond.is_empty() {
                        None
                    } else {
                        Some(cond)
                    }
                };

                if from.is_empty() || to.is_empty() {
                    bail!("Flow must have 'from' and 'to' attributes");
                }

                flows.push(Flow {
                    from,
                    to,
                    condition,
                });
            }
        }
    }

    Ok(flows)
}

// Calculate which row each activity should be placed in based on flow dependencies
fn calculate_activity_rows(activities: &[Activity], flows: &[Flow]) -> HashMap<String, usize> {
    println!("  🧮 Calculating activity rows...");

    let mut activity_rows: HashMap<String, usize> = HashMap::new();

    // Build a map of incoming flows for each activity
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for flow in flows {
        incoming
            .entry(flow.to.clone())
            .or_insert_with(Vec::new)
            .push(flow.from.clone());
    }

    println!("    Incoming flows:");
    for (target, sources) in &incoming {
        println!("      {} ← {:?}", target, sources);
    }

    // Find start nodes (activities with no incoming flows)
    let start_nodes: Vec<String> = activities
        .iter()
        .filter(|act| !incoming.contains_key(&act.id))
        .map(|act| act.id.clone())
        .collect();

    println!("    Start nodes: {:?}", start_nodes);

    if start_nodes.is_empty() {
        println!("    ⚠️  Warning: No start nodes found!");
        // Fallback: assign all to row 0
        for activity in activities {
            activity_rows.insert(activity.id.clone(), 0);
        }
        return activity_rows;
    }

    // Assign row 0 to start nodes
    for start_id in &start_nodes {
        activity_rows.insert(start_id.clone(), 0);
    }

    // Process activities in topological order
    // Handle cycles by allowing placement based on ANY processed dependency
    let mut queue: VecDeque<String> = start_nodes.clone().into();
    let mut visited: HashSet<String> = HashSet::new();

    while let Some(current_id) = queue.pop_front() {
        if visited.contains(&current_id) {
            continue;
        }
        visited.insert(current_id.clone());

        let current_row = *activity_rows.get(&current_id).unwrap();

        // Find all outgoing flows from current activity
        for flow in flows {
            if flow.from == current_id {
                let target_id = &flow.to;

                // Skip if already assigned
                if activity_rows.contains_key(target_id) {
                    continue;
                }

                // Get all dependencies of target
                let deps = incoming.get(target_id).cloned().unwrap_or_default();

                // Check if AT LEAST ONE dependency has been processed
                // This allows us to handle cycles in the flow graph
                let processed_deps: Vec<&String> = deps
                    .iter()
                    .filter(|dep| activity_rows.contains_key(*dep))
                    .collect();

                if !processed_deps.is_empty() {
                    // Calculate: MAX(processed dependency rows) + 1
                    let max_dep_row = processed_deps
                        .iter()
                        .filter_map(|dep| activity_rows.get(*dep))
                        .max()
                        .unwrap_or(&0);

                    let target_row = max_dep_row + 1;

                    // Set the row
                    activity_rows.insert(target_id.clone(), target_row);

                    println!("    {} → row {} (after {:?})", target_id, target_row, processed_deps);

                    queue.push_back(target_id.clone());
                }
            }
        }
    }

    // Summary
    println!("    ✅ Row assignments:");
    let mut rows_summary: HashMap<usize, Vec<String>> = HashMap::new();
    for (act_id, row) in &activity_rows {
        rows_summary
            .entry(*row)
            .or_insert_with(Vec::new)
            .push(act_id.clone());
    }
    let mut sorted_rows: Vec<usize> = rows_summary.keys().cloned().collect();
    sorted_rows.sort();
    for row in sorted_rows {
        println!("      Row {}: {:?}", row, rows_summary.get(&row).unwrap());
    }

    activity_rows
}

// Create visual node for an activity based on its type
fn create_activity_node(
    activity: &Activity,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    match activity.activity_type {
        ActivityType::Normal => {
            // Standard rectangular activity box
            let text = builder.new_text(
                format!("{}_text", activity.id),
                &activity.label,
                TextOptions {
                    font_size: 12.0,
                    text_color: PRIMARY_TEXT.to_owned(),
                    line_width: 400,
                    line_spacing: 8.0,
                    ..Default::default()
                },
            );

            let box_node = builder.new_box(
                format!("{}_inner", activity.id),
                text,
                BoxOptions {
                    padding: 12.0,
                    fill_color: Fill::Color("#B3E5FC".to_owned()),
                    stroke_color: "#01579B".to_owned(),
                    stroke_width: 2.0,
                    border_radius: 4.0,
                    width_behavior: SizeBehavior::Fixed(140.0),
                    horizontal_alignment: HorizontalAlignment::Center,
                    ..Default::default()
                },
            );

            // Wrap in a group to prevent constraint solver from resizing it
            let group = builder.new_group(activity.id.clone(), vec![box_node]);

            Ok(group)
        }

        ActivityType::Decision | ActivityType::Merge => {
            // Diamond shape using polyline
            // Create a diamond: center at (25, 25), size 50x50
            //  Top: (25, 0)
            //  Right: (50, 25)
            //  Bottom: (25, 50)
            //  Left: (0, 25)
            //  Close: back to (25, 0)
            let diamond = builder.new_polyline(
                format!("{}_inner", activity.id),
                vec![
                    (25.0, 0.0),    // Top
                    (50.0, 25.0),   // Right
                    (25.0, 50.0),   // Bottom
                    (0.0, 25.0),    // Left
                    (25.0, 0.0),    // Close path
                ],
                LineOptions {
                    stroke_color: "#F57F17".to_owned(),
                    stroke_width: 2.0,
                },
            );

            // Wrap in a group to prevent constraint solver from resizing it
            let group = builder.new_group(activity.id.clone(), vec![diamond]);

            Ok(group)
        }

        ActivityType::Start => {
            // Small filled circle
            let circle = builder.new_ellipse(
                format!("{}_inner", activity.id),
                (15.0, 15.0),
                EllipseOptions {
                    fill_color: "#4CAF50".to_owned(),
                    stroke_color: "#2E7D32".to_owned(),
                    stroke_width: 2.0,
                },
            );

            // Wrap in a group to prevent constraint solver from resizing it
            let group = builder.new_group(activity.id.clone(), vec![circle]);

            Ok(group)
        }

        ActivityType::End => {
            // Circle with thick border (double circle effect)
            let circle = builder.new_ellipse(
                format!("{}_inner", activity.id),
                (15.0, 15.0),
                EllipseOptions {
                    fill_color: "#F44336".to_owned(),
                    stroke_color: "#C62828".to_owned(),
                    stroke_width: 4.0,
                },
            );

            // Wrap in a group to prevent constraint solver from resizing it
            let group = builder.new_group(activity.id.clone(), vec![circle]);

            Ok(group)
        }
    }
}
fn create_layout_constraints(
    swimlanes: &[Swimlane],
    activity_rows: &HashMap<String, usize>,
) -> Vec<SimpleConstraint> {
    println!("  📐 Creating layout constraints...");

    let mut constraints = Vec::new();

    // Group activities by row
    let mut rows_map: HashMap<usize, Vec<String>> = HashMap::new();
    for (activity_id, row) in activity_rows {
        rows_map
            .entry(*row)
            .or_insert_with(Vec::new)
            .push(activity_id.clone());
    }

    // Fixed activity widths
    for lane in swimlanes {
        for activity in lane.activities.clone() {
            if matches!(activity.activity_type, ActivityType::Normal) {
                constraints.push(SimpleConstraint::FixedWidth(activity.id, 150.0));
            }
        }
    }

    // Get sorted row indices
    let mut row_indices: Vec<usize> = rows_map.keys().cloned().collect();
    row_indices.sort();

    println!("    Total rows: {}", row_indices.len());

    // 1. Stack rows vertically with proper spacing to prevent overlap
    // Strategy: Make all activities in a row have at least the same height (matching the tallest)
    // This ensures consistent spacing between rows
    if row_indices.len() > 1 {
        for i in 1..row_indices.len() {
            let prev_row = row_indices[i - 1];
            let curr_row = row_indices[i];

            if let (Some(prev_acts), Some(curr_acts)) =
                (rows_map.get(&prev_row), rows_map.get(&curr_row))
            {
                // Ensure all activities in the previous row have at least the same height
                // Apply the constraint to the inner elements because groups can't be resized
                if prev_acts.len() > 1 {
                    let inner_ids: Vec<String> = prev_acts
                        .iter()
                        .map(|id| format!("{}_inner", id))
                        .collect();
                    constraints.push(SimpleConstraint::AtLeastSameHeight(inner_ids));
                }

                // Use the first activity of each row for spacing calculation
                // Since all have at least the same height, this works correctly
                if let (Some(prev_rep), Some(curr_rep)) = (prev_acts.first(), curr_acts.first()) {
                    constraints.push(SimpleConstraint::VerticalSpacing(
                        prev_rep.clone(),
                        curr_rep.clone(),
                        60.0,
                    ));

                    println!("    Row {} below row {} (spacing: 60)", curr_row, prev_row);
                }
            }
        }
    }

    // 2. Align activities within each row horizontally (same Y position)
    for (row_idx, activities_in_row) in &rows_map {
        if activities_in_row.len() > 1 {
            constraints.push(SimpleConstraint::AlignTop(activities_in_row.clone()));
            println!(
                "    Row {}: aligned {} activities horizontally",
                row_idx,
                activities_in_row.len()
            );
        }
    }

    // 3. Align activities within each swimlane vertically (same X position)
    for swimlane in swimlanes {
        let lane_activities: Vec<String> = swimlane
            .activities
            .iter()
            .map(|act| act.id.clone())
            .collect();

        if lane_activities.len() > 1 {
            constraints.push(SimpleConstraint::AlignCenterHorizontal(lane_activities));
            println!(
                "    Lane '{}': aligned {} activities vertically",
                swimlane.name,
                swimlane.activities.len()
            );
        }
    }

    // 4. Space swimlanes apart horizontally
    let mut lane_representatives: Vec<String> = Vec::new();
    for swimlane in swimlanes {
        if let Some(first_activity) = swimlane.activities.first() {
            lane_representatives.push(first_activity.id.clone());
        }
    }

    if lane_representatives.len() > 1 {
        for i in 1..lane_representatives.len() {
            constraints.push(SimpleConstraint::HorizontalSpacing(
                lane_representatives[i - 1].clone(),
                lane_representatives[i].clone(),
                150.0,
            ));
            println!(
                "    Swimlane {} spaced from swimlane {} (spacing: 150)",
                i,
                i - 1
            );
        }
    }

    println!("    Total constraints: {}", constraints.len());

    constraints
}

// Create a connector between two activities with smart type and port selection
fn create_flow_connector(
    flow: &Flow,
    activity_rows: &HashMap<String, usize>,
    swimlanes: &[Swimlane],
    all_flows: &[Flow],
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    let connector_id = format!("flow_{}_{}", flow.from, flow.to);

    // Get row information
    let from_row = activity_rows.get(&flow.from).cloned();
    let to_row = activity_rows.get(&flow.to).cloned();

    // Determine which swimlane each activity is in
    let from_lane = find_activity_lane(&flow.from, swimlanes);
    let to_lane = find_activity_lane(&flow.to, swimlanes);

    // Check if source has multiple outgoing flows (is a decision point)
    let outgoing_flows: Vec<&Flow> = all_flows.iter().filter(|f| f.from == flow.from).collect();

    let is_decision_branch = outgoing_flows.len() > 1;

    // Determine connector type, ports, and routing strategy
    let (connector_type, source_port, target_port, routing_strategy) =
        match (from_row, to_row, from_lane, to_lane) {
            (Some(from_r), Some(to_r), Some(from_l), Some(to_l)) => {
                let same_lane = from_l == to_l;
                let is_below = to_r > from_r;

                if same_lane && is_below {
                    // Same lane, target below: straight down
                    (
                        ConnectorType::Straight,
                        Port::Bottom,
                        Port::Top,
                        OrthogonalRoutingStrategy::VHV,
                    )
                } else if same_lane && !is_below {
                    // Same lane, target above: straight up
                    (
                        ConnectorType::Straight,
                        Port::Top,
                        Port::Bottom,
                        OrthogonalRoutingStrategy::VHV,
                    )
                } else if is_below && is_decision_branch {
                    let branch_index = outgoing_flows
                        .iter()
                        .position(|f| f.to == flow.to)
                        .unwrap_or(0);

                    let total_branches = outgoing_flows.len();

                    if total_branches == 2 {
                        // Binary decision
                        if branch_index == 0 {
                            // First branch: use side port based on target position
                            let source_port = if from_l < to_l {
                                Port::Right
                            } else {
                                Port::Left
                            };
                            (
                                ConnectorType::Orthogonal,
                                source_port,
                                Port::Top,
                                OrthogonalRoutingStrategy::HV,
                            )
                        } else {
                            // Second branch: always use Bottom port
                            (
                                ConnectorType::Orthogonal,
                                Port::Bottom,
                                Port::Top,
                                OrthogonalRoutingStrategy::VHV,
                            )
                        }
                    } else if total_branches == 3 {
                        // Three-way decision
                        let target_lanes: Vec<(usize, &str)> = outgoing_flows
                            .iter()
                            .filter_map(|f| {
                                find_activity_lane(&f.to, swimlanes)
                                    .map(|lane| (lane, f.to.as_str()))
                            })
                            .collect();

                        // Sort by lane to get left-to-right order
                        let mut sorted_targets = target_lanes.clone();
                        sorted_targets.sort_by_key(|(lane, _)| *lane);

                        // Check if all targets are on the same side of the decision
                        let all_targets_right =
                            sorted_targets.iter().all(|(lane, _)| *lane > from_l);
                        let all_targets_left =
                            sorted_targets.iter().all(|(lane, _)| *lane < from_l);

                        // Find position of current target in sorted order
                        let position = sorted_targets
                            .iter()
                            .position(|(_, id)| *id == flow.to.as_str())
                            .unwrap_or(1);

                        if all_targets_right {
                            // All targets to the right: use Right, Bottom, Right (or BottomRight)
                            match position {
                                0 => (
                                    ConnectorType::Orthogonal,
                                    Port::Right,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ), // Closest
                                1 => (
                                    ConnectorType::Orthogonal,
                                    Port::Bottom,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::VHV,
                                ), // Middle
                                _ => (
                                    ConnectorType::Orthogonal,
                                    Port::BottomRight,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ), // Farthest
                            }
                        } else if all_targets_left {
                            // All targets to the left: use Left, Bottom, Left (or BottomLeft)
                            match position {
                                0 => (
                                    ConnectorType::Orthogonal,
                                    Port::BottomLeft,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ), // Farthest
                                1 => (
                                    ConnectorType::Orthogonal,
                                    Port::Bottom,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::VHV,
                                ), // Middle
                                _ => (
                                    ConnectorType::Orthogonal,
                                    Port::Left,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ), // Closest
                            }
                        } else {
                            // Mixed: targets on both sides - use standard Left, Bottom, Right
                            match position {
                                0 => (
                                    ConnectorType::Orthogonal,
                                    Port::Left,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ),
                                1 => (
                                    ConnectorType::Orthogonal,
                                    Port::Bottom,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::VHV,
                                ),
                                _ => (
                                    ConnectorType::Orthogonal,
                                    Port::Right,
                                    Port::Top,
                                    OrthogonalRoutingStrategy::HV,
                                ),
                            }
                        }
                    } else if total_branches >= 4 {
                        // Four or more branches: distribute around diamond
                        let target_lanes: Vec<(usize, &str)> = outgoing_flows
                            .iter()
                            .filter_map(|f| {
                                find_activity_lane(&f.to, swimlanes)
                                    .map(|lane| (lane, f.to.as_str()))
                            })
                            .collect();

                        let mut sorted_targets = target_lanes.clone();
                        sorted_targets.sort_by_key(|(lane, _)| *lane);

                        let position = sorted_targets
                            .iter()
                            .position(|(_, id)| *id == flow.to.as_str())
                            .unwrap_or(0);

                        // Distribute around 4 ports based on position
                        match position % 4 {
                            0 => (
                                ConnectorType::Orthogonal,
                                Port::Left,
                                Port::Top,
                                OrthogonalRoutingStrategy::HV,
                            ),
                            1 => (
                                ConnectorType::Orthogonal,
                                Port::Bottom,
                                Port::Top,
                                OrthogonalRoutingStrategy::VHV,
                            ),
                            2 => (
                                ConnectorType::Orthogonal,
                                Port::Right,
                                Port::Top,
                                OrthogonalRoutingStrategy::HV,
                            ),
                            _ => (
                                ConnectorType::Orthogonal,
                                Port::Top,
                                Port::Bottom,
                                OrthogonalRoutingStrategy::VHV,
                            ),
                        }
                    } else {
                        // Fallback for unexpected cases
                        (
                            ConnectorType::Orthogonal,
                            Port::Bottom,
                            Port::Top,
                            OrthogonalRoutingStrategy::VHV,
                        )
                    }
                } else if is_below {
                    // Different lanes, target below, not a branch (e.g., merge)
                    (
                        ConnectorType::Orthogonal,
                        Port::Bottom,
                        Port::Top,
                        OrthogonalRoutingStrategy::VHV,
                    )
                } else {
                    // Different lanes, target above
                    (
                        ConnectorType::Orthogonal,
                        Port::Top,
                        Port::Bottom,
                        OrthogonalRoutingStrategy::VHV,
                    )
                }
            }
            _ => {
                // Default: straight down
                (
                    ConnectorType::Straight,
                    Port::Bottom,
                    Port::Top,
                    OrthogonalRoutingStrategy::VHV,
                )
            }
        };

    let connector = builder.new_connector_with_label(
        connector_id.clone(),
        flow.from.clone(),
        flow.to.clone(),
        flow.condition.clone().unwrap_or("".to_string()),
        ConnectorOptions {
            connector_type,
            stroke_color: "#424242".to_owned(),
            stroke_width: 2.0,
            source_port,
            target_port,
            arrow_end: true,
            arrow_start: false,
            arrow_size: 8.0,
            curve_offset: None,
            routing_strategy,
        },
    );

    println!("Created connector with label {:?} {}", connector.entity_type, connector_id);

    Ok(connector)
}
// Helper function to find which swimlane an activity belongs to
fn find_activity_lane(activity_id: &str, swimlanes: &[Swimlane]) -> Option<usize> {
    for (lane_idx, lane) in swimlanes.iter().enumerate() {
        if lane.activities.iter().any(|act| act.id == activity_id) {
            return Some(lane_idx);
        }
    }
    None
}
pub fn create_activity_diagram(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode> {
    println!("🔧 Creating activity diagram: {}", id);

    // Parse input data
    let swimlanes = parse_swimlanes(attrs)?;
    let flows = parse_flows(attrs)?;

    println!("  📊 Swimlanes: {}", swimlanes.len());
    for (i, lane) in swimlanes.iter().enumerate() {
        println!(
            "    Lane {}: '{}' with {} activities",
            i,
            lane.name,
            lane.activities.len()
        );
        for act in &lane.activities {
            println!(
                "      - {} ({}): {:?}",
                act.id, act.label, act.activity_type
            );
        }
    }

    println!("  🔗 Flows: {}", flows.len());
    for flow in &flows {
        let cond_str = flow
            .condition
            .as_ref()
            .map(|s| format!(" [{}]", s))
            .unwrap_or_default();
        println!("    {} → {}{}", flow.from, flow.to, cond_str);
    }

    // Get all activities
    let all_activities: Vec<Activity> = swimlanes
        .iter()
        .flat_map(|lane| lane.activities.clone())
        .collect();

    // Calculate rows
    let activity_rows = calculate_activity_rows(&all_activities, &flows);

    // Create visual nodes for all activities
    println!("  🎨 Creating visual nodes...");
    let mut activity_children = Vec::new();

    for activity in &all_activities {
        let node = create_activity_node(activity, builder)?;
        activity_children.push((node, None));
        println!(
            "    Created {:?} node: {}",
            activity.activity_type, activity.id
        );
    }

    // Create swimlane visual elements
    println!("  🏊 Creating swimlane visuals...");
    let mut lane_backgrounds = Vec::new();
    let mut lane_headers = Vec::new();
    let mut lane_visual_constraints = Vec::new();

    // Calculate lane width based on activity spacing
    // Each lane should align with its activities
    let _lane_spacing = 150.0; // This matches the spacing in create_layout_constraints

    for (lane_idx, swimlane) in swimlanes.iter().enumerate() {
        // Skip empty lanes
        if swimlane.activities.is_empty() {
            continue;
        }

        // Calculate lane width based on widest activity type in the lane
        // Default widths by activity type:
        // - Normal activities: 140px (Fixed width) + padding = ~200px
        // - Decision/Merge: 50px diamond + padding = ~100px
        // - Start/End: 30px circles + padding = ~100px
        let lane_width = if swimlane.activities.iter().any(|a| matches!(a.activity_type, ActivityType::Normal)) {
            // Has normal activities - use wider lane
            200.0
        } else {
            // Only has small elements (start, end, decision, merge)
            100.0
        };

        // Create lane background with alternating colors
        let bg_color = if lane_idx % 2 == 0 {
            "#FAFAFA"
        } else {
            "#FFFFFF"
        };

        let lane_bg = builder.new_rectangle(
            format!("lane_{}_bg", lane_idx),
            RectOptions {
                width_behavior: SizeBehavior::Fixed(lane_width),
                height_behavior: SizeBehavior::Fixed(800.0),
                fill_color: Fill::Color(bg_color.to_owned()),
                stroke_color: "#E0E0E0".to_owned(),
                stroke_width: 1.0,
                border_radius: 0.0,
            },
        );
        lane_backgrounds.push((lane_bg, None));

        // Create lane header
        let header_text = builder.new_text(
            format!("lane_{}_header_text", lane_idx),
            &swimlane.name,
            TextOptions {
                font_size: 14.0,
                text_color: "#666666".to_owned(),
                line_width: 200,
                ..Default::default()
            },
        );

        let lane_header_box = builder.new_box(
            format!("lane_{}_header_inner", lane_idx),
            header_text,
            BoxOptions {
                padding: 8.0,
                fill_color: Fill::Color("#E8E8E8".to_owned()),
                stroke_color: "#CCCCCC".to_owned(),
                stroke_width: 1.0,
                border_radius: 0.0,
                horizontal_alignment: HorizontalAlignment::Center,
                ..Default::default()
            },
        );

        // Wrap in a group to prevent constraint solver from resizing it
        let lane_header = builder.new_group(
            format!("lane_{}_header", lane_idx),
            vec![lane_header_box]
        );
        lane_headers.push((lane_header, None));
    }

    // Get list of non-empty lane indices
    let non_empty_lanes: Vec<usize> = (0..swimlanes.len())
        .filter(|i| !swimlanes[*i].activities.is_empty())
        .collect();

    // Align all headers at top
    let header_ids: Vec<String> = non_empty_lanes
        .iter()
        .map(|i| format!("lane_{}_header", i))
        .collect();

    if !header_ids.is_empty() {
        lane_visual_constraints.push(SimpleConstraint::AlignTop(header_ids.clone()));
    }

    // Align all backgrounds at top with headers
    // This ensures backgrounds start from the same Y position as headers
    let bg_ids: Vec<String> = non_empty_lanes
        .iter()
        .map(|i| format!("lane_{}_bg", i))
        .collect();

    if !bg_ids.is_empty() && !header_ids.is_empty() {
        // Align backgrounds with each other
        lane_visual_constraints.push(SimpleConstraint::AlignTop(bg_ids.clone()));

        // Align the first background with the first header at the top
        // This anchors the entire swimlane structure
        lane_visual_constraints.push(SimpleConstraint::AlignTop(vec![
            bg_ids[0].clone(),
            header_ids[0].clone(),
        ]));
    }

    // Space backgrounds adjacently (touching, no gaps)
    for i in 1..non_empty_lanes.len() {
        let prev_lane = non_empty_lanes[i - 1];
        let curr_lane = non_empty_lanes[i];

        lane_visual_constraints.push(SimpleConstraint::HorizontalSpacing(
            format!("lane_{}_bg", prev_lane),
            format!("lane_{}_bg", curr_lane),
            0.0,
        ));
    }

    // Align each header with its background center
    for lane_idx in &non_empty_lanes {
        lane_visual_constraints.push(SimpleConstraint::AlignCenterHorizontal(vec![
            format!("lane_{}_header", lane_idx),
            format!("lane_{}_bg", lane_idx),
        ]));
    }

    // NOW align background centers with activity centers
    for (lane_idx, swimlane) in swimlanes.iter().enumerate() {
        if let Some(first_activity) = swimlane.activities.first() {
            let bg_id = format!("lane_{}_bg", lane_idx);

            // Center background with first activity in lane
            lane_visual_constraints.push(SimpleConstraint::AlignCenterHorizontal(vec![
                bg_id.clone(),
                first_activity.id.clone(),
            ]));
        }
    }

    // Just position first activity below headers
    if let Some(first_row_activity) = activity_rows
        .iter()
        .find(|(_, row)| **row == 0)
        .map(|(id, _)| id.clone())
    {
        if let Some(first_header) = header_ids.first() {
            lane_visual_constraints.push(SimpleConstraint::Below(
                first_row_activity.clone(),
                first_header.clone(),
            ));
            lane_visual_constraints.push(SimpleConstraint::VerticalSpacing(
                first_header.clone(),
                first_row_activity,
                30.0,
            ));
        }
    }

    // Make lane backgrounds stretch dynamically to cover all content
    let helper_node = if !bg_ids.is_empty() {
        // 1. Ensure all backgrounds have the same height
        lane_visual_constraints.push(SimpleConstraint::SameHeight(bg_ids.clone()));

        // 2. Align all background bottoms together
        lane_visual_constraints.push(SimpleConstraint::AlignBottom(bg_ids.clone()));

        // 3. Find the last (bottommost) activity in the diagram and create a spacing constraint
        // We'll create a dummy point below the last activity and align bg bottoms to it
        if let Some((_bottom_activity_id, max_row)) = activity_rows.iter().max_by_key(|(_, row)| *row) {
            if let Some((last_activity_id, _)) = activity_rows.iter().find(|(_, row)| *row == max_row) {
                // Create an invisible helper point positioned below the last activity
                // This point will serve as the target for the background bottom
                let helper_id = format!("bg_bottom_helper");
                let _helper_point = builder.new_point(helper_id.clone());

                // Position the helper 60px below the last activity
                // helper.y = last_activity.y + last_activity.height + 60
                lane_visual_constraints.push(SimpleConstraint::VerticalSpacing(
                    last_activity_id.clone(),
                    helper_id.clone(),
                    60.0,
                ));

                // Align background bottom with helper point (top)
                // bg.y + bg.height = helper.y + helper.height (where helper.height = 0)
                // This means: bg.y + bg.height = helper.y
                if let Some(first_bg) = bg_ids.first() {
                    lane_visual_constraints.push(SimpleConstraint::AlignBottom(vec![
                        helper_id.clone(),
                        first_bg.clone(),
                    ]));
                }

                Some((helper_id, _helper_point))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Combine all nodes: backgrounds first (render behind), then headers, then activities
    let mut all_children = Vec::new();
    all_children.extend(lane_backgrounds); // Backgrounds render first (behind)
    all_children.extend(lane_headers); // Headers next
    all_children.extend(activity_children); // Activities on top

    // Add helper node if it exists (it won't be visible but needs to be in the layout for constraints)
    if let Some((_helper_id, helper_point)) = helper_node {
        all_children.push((helper_point, None));
    }

    // Create constraints for activities layout
    let mut activity_constraints = create_layout_constraints(&swimlanes, &activity_rows);

    // Add lane visual constraints
    activity_constraints.extend(lane_visual_constraints);

    println!("  📐 Total constraints: {}", activity_constraints.len());

    // Create constraint container for everything
    println!("  📦 Creating constraint layout container...");
    let diagram_container =
        builder.new_constraint_layout_container(id.to_string(), all_children, activity_constraints);

    // Create connectors and register them at root level
    println!("  🔗 Creating connectors and registering at root level...");
    for flow in &flows {
        let connector = create_flow_connector(flow, &activity_rows, &swimlanes, &flows, builder)?;
        builder.register_root_level_node(connector);
        let cond_str = flow
            .condition
            .as_ref()
            .map(|s| format!(" [{}]", s))
            .unwrap_or_default();
        println!(
            "    Connector registered: {} → {}{}",
            flow.from, flow.to, cond_str
        );
    }

    println!(
        "  ✅ Activity diagram created with {} swimlanes, {} activities, {} connectors",
        swimlanes.len(),
        all_activities.len(),
        flows.len()
    );

    let wrapper = builder.new_box(
        format!("{}_wrapper", id),
        diagram_container,
        BoxOptions {
            fill_color: Fill::Color("white".to_string()),
            ..Default::default()
        },
    );

    Ok(wrapper)
}
