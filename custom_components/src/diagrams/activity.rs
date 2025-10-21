use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use crate::document::style::*;
use crate::parser::{get_array_attr, get_string_attr, JsonLinesParser};
use crate::*;
use anyhow::{bail, Result};
use serde_json::{from_value, Map, Value};
use std::collections::{HashMap, HashSet, VecDeque};

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

                // Get all dependencies of target
                let deps = incoming.get(target_id).cloned().unwrap_or_default();

                // Check if ALL dependencies have been processed
                let all_deps_processed = deps.iter().all(|dep| activity_rows.contains_key(dep));

                if all_deps_processed {
                    // Calculate: MAX(all dependency rows) + 1
                    let max_dep_row = deps
                        .iter()
                        .filter_map(|dep| activity_rows.get(dep))
                        .max()
                        .unwrap_or(&0);

                    let target_row = max_dep_row + 1;

                    // Set the row
                    activity_rows.insert(target_id.clone(), target_row);

                    println!("    {} → row {} (after {:?})", target_id, target_row, deps);

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
                    ..Default::default()
                },
            );

            let box_node = builder.new_box(
                activity.id.clone(),
                text,
                BoxOptions {
                    padding: 12.0,
                    fill_color: Fill::Color("#B3E5FC".to_owned()),
                    stroke_color: "#01579B".to_owned(),
                    stroke_width: 2.0,
                    border_radius: 4.0,
                    ..Default::default()
                },
            );

            Ok(box_node)
        }

        ActivityType::Decision | ActivityType::Merge => {
            // Diamond shape (rotated square)
            // Note: We'll make it a square now, rotation would need additional support
            let rect = builder.new_rectangle(
                activity.id.clone(),
                RectOptions {
                    width_behavior: SizeBehavior::Fixed(50.0),
                    height_behavior: SizeBehavior::Fixed(50.0),
                    fill_color: Fill::Color("#FFF9C4".to_owned()),
                    stroke_color: "#F57F17".to_owned(),
                    stroke_width: 2.0,
                    border_radius: 0.0,
                },
            );

            Ok(rect)
        }

        ActivityType::Start => {
            // Small filled circle
            let circle = builder.new_ellipse(
                activity.id.clone(),
                (15.0, 15.0),
                EllipseOptions {
                    fill_color: "#4CAF50".to_owned(),
                    stroke_color: "#2E7D32".to_owned(),
                    stroke_width: 2.0,
                },
            );

            Ok(circle)
        }

        ActivityType::End => {
            // Circle with thick border (double circle effect)
            let circle = builder.new_ellipse(
                activity.id.clone(),
                (15.0, 15.0),
                EllipseOptions {
                    fill_color: "#F44336".to_owned(),
                    stroke_color: "#C62828".to_owned(),
                    stroke_width: 4.0,
                },
            );

            Ok(circle)
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

    // Get sorted row indices
    let mut row_indices: Vec<usize> = rows_map.keys().cloned().collect();
    row_indices.sort();

    println!("    Total rows: {}", row_indices.len());

    // 1. Stack rows vertically
    if row_indices.len() > 1 {
        for i in 1..row_indices.len() {
            let prev_row = row_indices[i - 1];
            let curr_row = row_indices[i];

            if let (Some(prev_acts), Some(curr_acts)) =
                (rows_map.get(&prev_row), rows_map.get(&curr_row))
            {
                if let (Some(prev_rep), Some(curr_rep)) = (prev_acts.first(), curr_acts.first()) {
                    constraints.push(SimpleConstraint::Below(curr_rep.clone(), prev_rep.clone()));

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

    // REMOVED STEP 3 - no row-based horizontal spacing

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
                    // Different lanes, target below, branching from decision
                    // Use different ports for each branch

                    let branch_index = outgoing_flows
                        .iter()
                        .position(|f| f.to == flow.to)
                        .unwrap_or(0);

                    let total_branches = outgoing_flows.len();

                    if total_branches == 2 {
                        // Binary decision
                        if branch_index == 0 {
                            // First branch: Right port with HV routing (horizontal then vertical)
                            (
                                ConnectorType::Orthogonal,
                                Port::Right,
                                Port::Top,
                                OrthogonalRoutingStrategy::HV,
                            )
                        } else {
                            // Second branch: Bottom port with VHV routing (straight or with turns)
                            (
                                ConnectorType::Orthogonal,
                                Port::Bottom,
                                Port::Top,
                                OrthogonalRoutingStrategy::VHV,
                            )
                        }
                    } else if total_branches == 3 {
                        // Three-way decision: Left, Bottom, Right
                        match branch_index {
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
                    } else if total_branches == 4 {
                        // Four-way: Left, Bottom, Right, Top
                        match branch_index {
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
                        // More than 4: distribute around all sides
                        match branch_index % 4 {
                            0 => (
                                ConnectorType::Orthogonal,
                                Port::Right,
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
                                Port::Left,
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

    let connector = builder.new_connector(
        connector_id.clone(),
        flow.from.clone(),
        flow.to.clone(),
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

    // Create constraints
    let constraints = create_layout_constraints(&swimlanes, &activity_rows);

    // Create constraint container for activities ONLY
    println!("  📦 Creating constraint layout container...");
    let activities_container =
        builder.new_constraint_layout_container(id.to_string(), activity_children, constraints);

    // Create connectors and register them at root level
    println!("  🔗 Creating connectors and registering at root level...");
    for flow in &flows {
        let connector = create_flow_connector(flow, &activity_rows, &swimlanes, &flows, builder)?; // Pass &flows
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
        "  ✅ Activity diagram created with {} connectors at root level",
        flows.len()
    );

    Ok(activities_container)
}
