// src/diagrams/calendar.rs
// Calendar custom component for Volare Engine

use crate::diagram_builder::{DiagramBuilder, DiagramTreeNode};
use crate::parser::{get_string_attr, JsonLinesParser};
use crate::*;
use anyhow::{bail, Result};
use serde_json::{Map, Value};

// Calendar event structure
#[derive(Clone, Debug)]
struct CalendarEvent {
    id: String,
    title: String,
    start_time: String,
    end_time: String,
    color: String,
    all_day: bool,
}

// Day cell data
#[derive(Clone, Debug)]
struct DayCell {
    day: u32,
    is_current_month: bool,
    is_current_day: bool,
    is_weekend: bool,
    events: Vec<CalendarEvent>,
}

// Month data structure
#[derive(Clone, Debug)]
struct MonthData {
    year: u32,
    month: u32,
    month_name: String,
    days_in_month: u32,
    first_day_of_week: u32, // 0=Monday, 6=Sunday
    grid: Vec<DayCell>,     // 35 or 42 cells (5 or 6 weeks)
    current_day: u32,
    current_month: u32,
    current_year: u32,
}

/// Parse events from attributes
fn parse_events(attrs: &Map<String, Value>) -> Result<Vec<CalendarEvent>> {
    let events_value = attrs.get("events");
    let mut events = Vec::new();

    if let Some(Value::Array(events_json)) = events_value {
        for event_value in events_json {
            if let Value::Object(event_obj) = event_value {
                let id = get_string_attr(&event_obj, &["id"], "");
                let title = get_string_attr(&event_obj, &["title", "name"], "");
                let start_time = get_string_attr(&event_obj, &["start_time", "start"], "");
                let end_time = get_string_attr(&event_obj, &["end_time", "end"], "");
                let color = get_string_attr(&event_obj, &["color"], "#0d6efd");
                let all_day = event_obj
                    .get("all_day")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                events.push(CalendarEvent {
                    id,
                    title,
                    start_time,
                    end_time,
                    color,
                    all_day,
                });
            }
        }
    }

    Ok(events)
}

/// Get month name from month number (1-12)
fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Calculate days in month
fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Leap year calculation
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Calculate first day of week for a given month (0=Monday, 6=Sunday)
/// Using Zeller's congruence algorithm
fn first_day_of_week(year: u32, month: u32) -> u32 {
    let q = 1; // First day of month
    let m = if month < 3 { month + 12 } else { month };
    let y = if month < 3 { year - 1 } else { year };
    let k = y % 100;
    let j = y / 100;

    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;

    // Convert Saturday=0 to Monday=0 format
    // Zeller: 0=Saturday, 1=Sunday, 2=Monday, ..., 6=Friday
    // Target: 0=Monday, 1=Tuesday, ..., 5=Saturday, 6=Sunday
    match h {
        0 => 5, // Saturday
        1 => 6, // Sunday
        2 => 0, // Monday
        3 => 1, // Tuesday
        4 => 2, // Wednesday
        5 => 3, // Thursday
        6 => 4, // Friday
        _ => 0,
    }
}

/// Parse date string (ISO format: YYYY-MM-DD)
fn parse_date(date_str: &str) -> Result<(u32, u32, u32)> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        bail!("Invalid date format. Expected YYYY-MM-DD");
    }

    let year = parts[0]
        .parse::<u32>()
        .map_err(|_| anyhow::anyhow!("Invalid year"))?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| anyhow::anyhow!("Invalid month"))?;
    let day = parts[2]
        .parse::<u32>()
        .map_err(|_| anyhow::anyhow!("Invalid day"))?;

    if month < 1 || month > 12 {
        bail!("Month must be between 1 and 12");
    }
    if day < 1 || day > days_in_month(year, month) {
        bail!("Invalid day for the given month");
    }

    Ok((year, month, day))
}

/// Calculate month data including the calendar grid
fn calculate_month_data(date_str: &str, events: Vec<CalendarEvent>) -> Result<MonthData> {
    let (year, month, current_day) = parse_date(date_str)?;

    let month_name = get_month_name(month);
    let days = days_in_month(year, month);
    let first_day = first_day_of_week(year, month);

    // Calculate previous and next month info
    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    };
    let prev_month_days = days_in_month(prev_year, prev_month);

    let (_next_year, _next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    // Build grid
    let mut grid = Vec::new();

    // Add days from previous month
    for i in 0..first_day {
        let day = prev_month_days - first_day + i + 1;
        grid.push(DayCell {
            day,
            is_current_month: false,
            is_current_day: false,
            is_weekend: i >= 5, // Saturday or Sunday
            events: Vec::new(),
        });
    }

    // Add days from current month
    for day in 1..=days {
        let day_index = first_day + day - 1;
        let is_weekend = (day_index % 7) >= 5;

        // Filter events for this day
        let day_events: Vec<CalendarEvent> = events
            .iter()
            .filter(|e| {
                // Simple date matching - extract day from start_time
                if let Some(date_part) = e.start_time.split('T').next() {
                    if let Ok((e_year, e_month, e_day)) = parse_date(date_part) {
                        return e_year == year && e_month == month && e_day == day;
                    }
                }
                false
            })
            .cloned()
            .collect();

        grid.push(DayCell {
            day,
            is_current_month: true,
            is_current_day: day == current_day,
            is_weekend,
            events: day_events,
        });
    }

    // Add days from next month to complete the grid
    let cells_needed = if grid.len() <= 35 { 35 } else { 42 };
    let remaining = cells_needed - grid.len();

    for day in 1..=remaining {
        let day_index = grid.len();
        let is_weekend = (day_index % 7) >= 5;
        grid.push(DayCell {
            day: day as u32,
            is_current_month: false,
            is_current_day: false,
            is_weekend,
            events: Vec::new(),
        });
    }

    Ok(MonthData {
        year,
        month,
        month_name: month_name.to_string(),
        days_in_month: days,
        first_day_of_week: first_day,
        grid,
        current_day,
        current_month: month,
        current_year: year,
    })
}

/// Create calendar component (factory function)
pub fn create_calendar(
    id: &str,
    attrs: &Map<String, Value>,
    builder: &mut DiagramBuilder,
    parser: &JsonLinesParser,
) -> Result<DiagramTreeNode> {
    // Parse attributes
    let view = get_string_attr(attrs, &["view"], "month");
    let date = get_string_attr(attrs, &["date"], "2025-11-06");
    let events = parse_events(attrs)?;

    // Currently only month view is implemented
    if view != "month" {
        bail!("Only 'month' view is currently supported");
    }

    // Calculate month data
    let month_data = calculate_month_data(&date, events)?;

    // Style constants
    let cell_width = 106.0;
    let cell_height = 82.0;
    let header_height = 60.0;
    let weekday_height = 40.0;
    let spacing = 2.0;

    let primary_text = "#212529";
    let muted_text = "#adb5bd";
    let secondary_text = "#495057";
    let border_color = "#dee2e6";
    let bg_primary = "#ffffff";
    let bg_secondary = "#f8f9fa";
    let bg_weekend = "#e3f2fd";
    let weekend_text = "#1976d2";
    let current_day_bg = "#fff3cd";
    let current_day_border = "#ffc107";

    // Create month header
    let header_text = builder.new_text(
        format!("{}_header_text", id),
        &format!("{} {}", month_data.month_name, month_data.year),
        TextOptions {
            font_size: 32.0,
            font_family: "Arial".to_string(),
            text_color: primary_text.to_string(),
            font_weight: 700, // Bold
            line_width: 100,
            line_spacing: 1.2,
        },
    );

    let header_box = builder.new_box(
        format!("{}_header", id),
        header_text,
        BoxOptions {
            width_behavior: SizeBehavior::Fixed(760.0),
            height_behavior: SizeBehavior::Fixed(header_height),
            fill_color: Fill::Color(bg_secondary.to_string()),
            stroke_color: border_color.to_string(),
            stroke_width: 0.0,
            border_radius: 8.0,
            padding: 0.0,
            horizontal_alignment: HorizontalAlignment::Center,
        },
    );

    // Create weekday header
    let weekday_names = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let mut weekday_children = Vec::new();

    for (i, day_name) in weekday_names.iter().enumerate() {
        let is_weekend = i >= 5;
        let bg_color = if is_weekend { bg_weekend } else { "#e9ecef" };
        let text_color = if is_weekend {
            weekend_text
        } else {
            secondary_text
        };

        let day_text = builder.new_text(
            format!("{}_weekday_text_{}", id, i),
            day_name,
            TextOptions {
                font_size: 14.0,
                font_family: "Arial".to_string(),
                text_color: text_color.to_string(),
                font_weight: 700, // Bold
                line_width: 20,
                line_spacing: 1.0,
            },
        );

        let day_box = builder.new_box(
            format!("{}_weekday_{}", id, i),
            day_text,
            BoxOptions {
                width_behavior: SizeBehavior::Fixed(cell_width),
                height_behavior: SizeBehavior::Fixed(weekday_height),
                fill_color: Fill::Color(bg_color.to_string()),
                stroke_color: border_color.to_string(),
                stroke_width: 1.0,
                padding: 0.0,
                border_radius: 0.0,
                horizontal_alignment: HorizontalAlignment::Center,
            },
        );

        weekday_children.push(day_box);
    }

    let weekday_header = builder.new_hstack(
        format!("{}_weekday_header", id),
        weekday_children,
        VerticalAlignment::Center,
    );

    // Create calendar grid (weeks)
    let weeks = month_data.grid.len() / 7;
    let mut week_rows = Vec::new();

    for week in 0..weeks {
        let mut day_cells = Vec::new();

        for day_in_week in 0..7 {
            let cell_index = week * 7 + day_in_week;
            let cell = &month_data.grid[cell_index];

            // Determine cell styling
            let text_color = if !cell.is_current_month {
                muted_text
            } else {
                primary_text
            };

            let bg_color = if cell.is_current_day {
                current_day_bg
            } else if cell.is_weekend {
                bg_weekend
            } else if cell.is_current_month {
                bg_primary
            } else {
                bg_secondary
            };

            let border_width = if cell.is_current_day { 2.0 } else { 1.0 };
            let border = if cell.is_current_day {
                current_day_border
            } else {
                border_color
            };

            let is_bold = cell.is_current_day;

            // Create day number text
            let day_text = builder.new_text(
                format!("{}_day_text_{}_{}", id, week, day_in_week),
                &cell.day.to_string(),
                TextOptions {
                    font_size: 14.0,
                    font_family: "Arial".to_string(),
                    text_color: text_color.to_string(),
                    font_weight: if is_bold { 700 } else { 400 },
                    line_width: 20,
                    line_spacing: 1.0,
                },
            );

            // Create a group for day content (text + event indicators)
            let mut cell_content_children = vec![day_text];

            // Add event names if there are events
            if !cell.events.is_empty() {
                for (event_idx, event) in cell.events.iter().take(3).enumerate() {
                    // Create colored box for event
                    let event_text = builder.new_text(
                        format!("{}_event_{}_{}_text", id, cell_index, event_idx),
                        &event.title,
                        TextOptions {
                            font_size: 10.0,
                            font_family: "Arial".to_string(),
                            text_color: "#ffffff".to_string(),
                            font_weight: 400,
                            line_width: 15,
                            line_spacing: 1.0,
                        },
                    );

                    let event_box = builder.new_box(
                        format!("{}_event_{}_{}", id, cell_index, event_idx),
                        event_text,
                        BoxOptions {
                            width_behavior: SizeBehavior::Content,
                            height_behavior: SizeBehavior::Content,
                            fill_color: Fill::Color(event.color.clone()),
                            stroke_color: event.color.clone(),
                            stroke_width: 0.0,
                            padding: 2.0,
                            border_radius: 2.0,
                            horizontal_alignment: HorizontalAlignment::Left,
                        },
                    );

                    cell_content_children.push(event_box);
                }
            }

            // Create vertical stack for cell content
            let cell_content = builder.new_vstack(
                format!("{}_cell_content_{}_{}", id, week, day_in_week),
                cell_content_children,
                HorizontalAlignment::Left,
            );

            // Create cell box
            let cell_box = builder.new_box(
                format!("{}_cell_{}_{}", id, week, day_in_week),
                cell_content,
                BoxOptions {
                    width_behavior: SizeBehavior::Fixed(cell_width),
                    height_behavior: SizeBehavior::Fixed(cell_height),
                    fill_color: Fill::Color(bg_color.to_string()),
                    stroke_color: border.to_string(),
                    stroke_width: border_width,
                    padding: 5.0,
                    border_radius: 0.0,
                    horizontal_alignment: HorizontalAlignment::Left,
                },
            );

            day_cells.push(cell_box);
        }

        // Create week row
        let week_row = builder.new_hstack(
            format!("{}_week_{}", id, week),
            day_cells,
            VerticalAlignment::Top,
        );

        week_rows.push(week_row);
    }

    // Create calendar grid container
    let calendar_grid =
        builder.new_vstack(format!("{}_grid", id), week_rows, HorizontalAlignment::Left);

    // Create main container
    let calendar_container = builder.new_vstack(
        format!("{}_calendar_container", id.to_string()),
        vec![header_box, weekday_header, calendar_grid],
        HorizontalAlignment::Left,
    );

    let box_container = builder.new_box(
        id.to_string(),
        calendar_container,
        BoxOptions {
            fill_color: Fill::Color("white".to_string()),
            stroke_color: "".to_owned(),
            stroke_width: 0.0,
            padding: 0.0,
            border_radius: 0.0,
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Content,
            horizontal_alignment: HorizontalAlignment::Center,
        },
    );

    Ok(box_container)
}
