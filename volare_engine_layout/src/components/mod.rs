pub mod table;

use core::fmt;
use std::{any::Any, collections::HashMap, sync::Arc};

use serde_json::{Map, Value};

pub use crate::components::table::*;
//new type EntityID that is a u64
pub type EntityID = String;
pub type Float = f32;

//Export table and table options

pub trait Entity {
    fn get_id(&self) -> EntityID;
    fn get_type(&self) -> EntityType;
    //as_any
    fn as_any(&self) -> &dyn Any;
}

pub struct Point {
    pub x: Float,
    pub y: Float,
}

//impl clone
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

//impl new
impl Point {
    pub fn new(x: Float, y: Float) -> Self {
        Point { x, y }
    }
}

pub struct Size {
    pub w: Float,
    pub h: Float,
}

//impl clone
impl Clone for Size {
    fn clone(&self) -> Self {
        Size {
            w: self.w,
            h: self.h,
        }
    }
}

//impl new
impl Size {
    pub fn new(w: Float, h: Float) -> Self {
        Size { w, h }
    }
}

//Note: add new items to the end of the enum to avoid breaking the serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    BoxShape,
    RectShape,
    TextShape,
    LineShape,
    ArrowShape,
    EllipseShape,
    ImageShape,
    GroupShape,
    VerticalStackShape,
    HorizontalStackShape,
    TableShape,
    TextLine,
    PolyLine,
    FreeContainer,
    ArcShape,
    SpacerShape,
}

#[derive(Debug, Copy, PartialEq)]
pub enum SizeBehavior {
    /// Fixed size - element has a predetermined size that doesn't change
    Fixed(Float),
    /// Content size - element sizes itself based on its content (current default behavior)
    Content,
    /// Grow size - element takes all available space from its parent
    Grow,
}

impl Default for SizeBehavior {
    fn default() -> Self {
        SizeBehavior::Content
    }
}

impl Eq for SizeBehavior {}

impl Clone for SizeBehavior {
    fn clone(&self) -> Self {
        match self {
            SizeBehavior::Fixed(v) => SizeBehavior::Fixed(*v),
            SizeBehavior::Content => SizeBehavior::Content,
            SizeBehavior::Grow => SizeBehavior::Grow,
        }
    }
}

impl SizeBehavior {
    pub fn unwrap_fixed(&self) -> Result<f32, &'static str> {
        match self {
            SizeBehavior::Fixed(val) => Ok(*val),
            _ => Err("Called unwrap_fixed on non-Fixed SizeBehavior"),
        }
    }
}

// Add spacer component
pub struct ShapeSpacer {
    pub entity: EntityID,
    pub spacer_options: SpacerOptions,
}

impl ShapeSpacer {
    pub fn new(entity: EntityID, spacer_options: SpacerOptions) -> ShapeSpacer {
        ShapeSpacer {
            entity,
            spacer_options,
        }
    }
}

impl Clone for ShapeSpacer {
    fn clone(&self) -> Self {
        ShapeSpacer {
            entity: self.entity.clone(),
            spacer_options: self.spacer_options.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct SpacerOptions {
    pub width: Float,
    pub height: Float,
    pub direction: SpacerDirection, // Horizontal, Vertical, or Both
}

impl Clone for SpacerOptions {
    fn clone(&self) -> Self {
        SpacerOptions {
            width: self.width,
            height: self.height,
            direction: self.direction.clone(),
        }
    }
}

#[derive(Debug)]
pub enum SpacerDirection {
    Horizontal, // Takes up width, minimal height
    Vertical,   // Takes up height, minimal width
    Both,       // Takes up both dimensions
}

impl Default for SpacerDirection {
    fn default() -> Self {
        SpacerDirection::Vertical
    }
}

impl Clone for SpacerDirection {
    fn clone(&self) -> Self {
        match self {
            SpacerDirection::Horizontal => SpacerDirection::Horizontal,
            SpacerDirection::Vertical => SpacerDirection::Vertical,
            SpacerDirection::Both => SpacerDirection::Both,
        }
    }
}

/**
 * Boxes show a rectangle around the wrapped entity
 */
#[derive(Debug)]
pub struct ShapeBox {
    pub entity: EntityID,
    //Each box wraps another entity
    pub wrapped_entity: EntityID,
    pub box_options: BoxOptions,
}

impl Clone for ShapeBox {
    fn clone(&self) -> Self {
        ShapeBox {
            entity: self.entity.clone(),
            wrapped_entity: self.wrapped_entity.clone(),
            box_options: self.box_options.clone(),
        }
    }
}

impl Entity for ShapeBox {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::BoxShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeBox {
    pub fn new(entity: EntityID, wrapped_entity: EntityID, box_options: BoxOptions) -> ShapeBox {
        ShapeBox {
            entity,
            wrapped_entity,
            box_options,
        }
    }
}

#[derive(Debug)]
pub enum GradientStop {
    ColorStop { offset: Float, color: String },
    OpacityStop { offset: Float, opacity: Float },
}

#[derive(Debug)]
pub struct LinearGradient {
    pub x1: Float,
    pub y1: Float,
    pub x2: Float,
    pub y2: Float,
    pub stops: Vec<GradientStop>,
}

impl LinearGradient {
    pub fn new(x1: Float, y1: Float, x2: Float, y2: Float, stops: Vec<GradientStop>) -> Self {
        LinearGradient {
            x1,
            y1,
            x2,
            y2,
            stops,
        }
    }
}

impl Clone for GradientStop {
    fn clone(&self) -> Self {
        match self {
            GradientStop::ColorStop { offset, color } => GradientStop::ColorStop {
                offset: *offset,
                color: color.clone(),
            },
            GradientStop::OpacityStop { offset, opacity } => GradientStop::OpacityStop {
                offset: *offset,
                opacity: *opacity,
            },
        }
    }
}

#[derive(Debug)]
pub struct RadialGradient {
    pub cx: Float,
    pub cy: Float,
    pub r: Float,
    pub stops: Vec<GradientStop>,
}

impl Clone for RadialGradient {
    fn clone(&self) -> Self {
        RadialGradient {
            cx: self.cx,
            cy: self.cy,
            r: self.r,
            stops: self.stops.clone(),
        }
    }
}

impl Clone for LinearGradient {
    fn clone(&self) -> Self {
        LinearGradient {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            stops: self.stops.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Fill {
    Color(String),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

impl Clone for Fill {
    fn clone(&self) -> Self {
        match self {
            Fill::Color(color) => Fill::Color(color.clone()),
            Fill::LinearGradient(gradient) => Fill::LinearGradient(gradient.clone()),
            Fill::RadialGradient(gradient) => Fill::RadialGradient(gradient.clone()),
        }
    }
}
//default trait for fill
impl Default for Fill {
    fn default() -> Self {
        Fill::Color(String::from("white"))
    }
}

//display for fill
impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fill::Color(color) => write!(f, "{}", color),
            Fill::LinearGradient(gradient) => write!(f, "{:?}", gradient),
            Fill::RadialGradient(gradient) => write!(f, "{:?}", gradient),
        }
    }
}

impl PartialEq for Fill {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Fill::Color(a), Fill::Color(b)) => a == b,
            (Fill::LinearGradient(a), Fill::LinearGradient(b)) => a == b,
            (Fill::RadialGradient(a), Fill::RadialGradient(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Fill {}

impl PartialEq for LinearGradient {
    fn eq(&self, other: &Self) -> bool {
        self.x1 == other.x1
            && self.y1 == other.y1
            && self.x2 == other.x2
            && self.y2 == other.y2
            && self.stops == other.stops
    }
}

impl Eq for LinearGradient {}

impl PartialEq for RadialGradient {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy && self.r == other.r && self.stops == other.stops
    }
}

impl Eq for RadialGradient {}

impl PartialEq for GradientStop {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                GradientStop::ColorStop {
                    offset: a_offset,
                    color: a_color,
                },
                GradientStop::ColorStop {
                    offset: b_offset,
                    color: b_color,
                },
            ) => a_offset == b_offset && a_color == b_color,
            (
                GradientStop::OpacityStop {
                    offset: a_offset,
                    opacity: a_opacity,
                },
                GradientStop::OpacityStop {
                    offset: b_offset,
                    opacity: b_opacity,
                },
            ) => a_offset == b_offset && a_opacity == b_opacity,
            _ => false,
        }
    }
}

impl Eq for GradientStop {}

#[derive(Debug)]
pub struct BoxOptions {
    pub fill_color: Fill,
    pub stroke_color: String,
    pub stroke_width: Float,
    pub padding: Float,
    pub border_radius: Float,
    // Add size behavior fields
    pub width_behavior: SizeBehavior,
    pub height_behavior: SizeBehavior,
}

impl Clone for BoxOptions {
    fn clone(&self) -> Self {
        BoxOptions {
            fill_color: self.fill_color.clone(),
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
            padding: self.padding,
            border_radius: self.border_radius,
            width_behavior: self.width_behavior.clone(),
            height_behavior: self.height_behavior.clone(),
        }
    }
}

impl Default for BoxOptions {
    fn default() -> Self {
        BoxOptions::new()
    }
}

impl BoxOptions {
    pub fn new() -> BoxOptions {
        BoxOptions {
            fill_color: Fill::Color(String::from("white")),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            padding: 10.0,
            border_radius: 0.0,
            width_behavior: SizeBehavior::Content,
            height_behavior: SizeBehavior::Content,
        }
    }
}

/* A group of entities */

//RectOptions
#[derive(Default, Debug)]
pub struct RectOptions {
    pub width_behavior: SizeBehavior,
    pub height_behavior: SizeBehavior,
    pub fill_color: Fill,
    pub stroke_color: String,
    pub stroke_width: Float,
    pub border_radius: Float,
}

impl Clone for RectOptions {
    fn clone(&self) -> Self {
        RectOptions {
            width_behavior: self.width_behavior.clone(),
            height_behavior: self.height_behavior.clone(),
            fill_color: self.fill_color.clone(),
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
            border_radius: self.border_radius,
        }
    }
}

impl RectOptions {
    pub fn new() -> RectOptions {
        RectOptions {
            width_behavior: SizeBehavior::Fixed(100.0),
            height_behavior: SizeBehavior::Fixed(100.0),
            // Default fill color is white
            fill_color: Fill::Color(String::from("white")),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            border_radius: 0.0,
        }
    }
}

pub struct ShapeRect {
    pub entity: EntityID,
    pub rect_options: RectOptions,
}

impl ShapeRect {
    pub fn new(entity: EntityID, rect_options: RectOptions) -> ShapeRect {
        ShapeRect {
            entity,
            rect_options,
        }
    }
}

impl Clone for ShapeRect {
    fn clone(&self) -> Self {
        ShapeRect {
            entity: self.entity.clone(),
            rect_options: self.rect_options.clone(),
        }
    }
}

impl Entity for ShapeRect {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::RectShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/* A group of entities */
pub struct ShapeGroup {
    pub entity: EntityID,
    pub elements: Vec<EntityID>,
}

impl Clone for ShapeGroup {
    fn clone(&self) -> Self {
        ShapeGroup {
            entity: self.entity.clone(),
            elements: self.elements.clone(),
        }
    }
}

impl Entity for ShapeGroup {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::GroupShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Represents a line after adding breaks
#[derive(Debug)]
pub struct TextLine {
    pub entity: EntityID,
    pub text: String,
}

impl Clone for TextLine {
    fn clone(&self) -> Self {
        TextLine {
            entity: self.entity.clone(),
            text: self.text.clone(),
        }
    }
}

impl Entity for TextLine {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::TextLine
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
//add copy trait
#[derive(Debug)]
pub struct ShapeText {
    pub entity: EntityID,
    pub text: String,
    pub text_options: TextOptions,
    pub lines: Vec<EntityID>,
}

impl Clone for ShapeText {
    fn clone(&self) -> Self {
        ShapeText {
            entity: self.entity.clone(),
            text: self.text.clone(),
            text_options: self.text_options.clone(),
            lines: self.lines.clone(),
        }
    }
}

impl ShapeText {
    pub fn new(
        entity: EntityID,
        text: &str,
        text_options: TextOptions,
        lines: &[EntityID],
    ) -> ShapeText {
        ShapeText {
            entity,
            text: text.to_string(),
            text_options,
            lines: lines.to_vec(),
        }
    }
}

impl Entity for ShapeText {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::TextShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

//struct with text options: font family, font size
#[derive(Default, Debug)]
pub struct TextOptions {
    pub font_family: String,
    pub font_size: f32,
    pub text_color: String,
    // (number of max characters per line)used to know when to insert breaks
    pub line_width: usize,
    pub line_spacing: f32, // spacing between lines
}

impl Clone for TextOptions {
    fn clone(&self) -> Self {
        TextOptions {
            font_family: self.font_family.clone(),
            font_size: self.font_size,
            text_color: self.text_color.clone(),
            line_width: self.line_width,
            line_spacing: self.line_spacing,
        }
    }
}

impl TextOptions {
    pub fn new() -> TextOptions {
        TextOptions {
            font_family: String::from("Roboto"),
            font_size: 12.0,
            text_color: String::from("black"),
            line_width: 20,
            line_spacing: 0.0,
        }
    }
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl Clone for HorizontalAlignment {
    fn clone(&self) -> Self {
        match self {
            HorizontalAlignment::Left => HorizontalAlignment::Left,
            HorizontalAlignment::Center => HorizontalAlignment::Center,
            HorizontalAlignment::Right => HorizontalAlignment::Right,
        }
    }
}

pub struct VerticalStack {
    pub entity: EntityID,
    //List of entity ids
    pub elements: Vec<EntityID>,
    pub horizontal_alignment: HorizontalAlignment,
}

impl Clone for VerticalStack {
    fn clone(&self) -> Self {
        VerticalStack {
            entity: self.entity.clone(),
            elements: self.elements.clone(),
            horizontal_alignment: self.horizontal_alignment.clone(),
        }
    }
}

impl Entity for VerticalStack {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::VerticalStackShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl Clone for VerticalAlignment {
    fn clone(&self) -> Self {
        match self {
            VerticalAlignment::Top => VerticalAlignment::Top,
            VerticalAlignment::Center => VerticalAlignment::Center,
            VerticalAlignment::Bottom => VerticalAlignment::Bottom,
        }
    }
}
impl fmt::Display for VerticalAlignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerticalAlignment::Top => write!(f, "Top"),
            VerticalAlignment::Center => write!(f, "Center"),
            VerticalAlignment::Bottom => write!(f, "Bottom"),
        }
    }
}
//enum for horizontal stack

pub struct HorizontalStack {
    pub entity: EntityID,
    //List of entity ids
    pub elements: Vec<EntityID>,
    pub vertical_alignment: VerticalAlignment, // Optional vertical alignment (e.g., "top", "center", "bottom")
}

impl Clone for HorizontalStack {
    fn clone(&self) -> Self {
        HorizontalStack {
            entity: self.entity.clone(),
            elements: self.elements.clone(),
            vertical_alignment: self.vertical_alignment.clone(),
        }
    }
}

impl Entity for HorizontalStack {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::HorizontalStackShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ShapeLine {
    pub entity: EntityID,
    pub start: (Float, Float),
    pub end: (Float, Float),
    pub line_options: LineOptions,
}

impl Clone for ShapeLine {
    fn clone(&self) -> Self {
        ShapeLine {
            entity: self.entity.clone(),
            start: self.start,
            end: self.end,
            line_options: self.line_options.clone(),
        }
    }
}

impl ShapeLine {
    pub fn new(
        line_id: EntityID,
        start: (Float, Float),
        end: (Float, Float),
        options: LineOptions,
    ) -> ShapeLine {
        ShapeLine {
            entity: line_id,
            start,
            end,
            line_options: options,
        }
    }
}

impl Entity for ShapeLine {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::LineShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct LineOptions {
    pub stroke_color: String,
    pub stroke_width: Float,
}

impl Clone for LineOptions {
    fn clone(&self) -> Self {
        LineOptions {
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
        }
    }
}

impl LineOptions {
    pub fn new() -> LineOptions {
        LineOptions {
            stroke_color: String::from("black"),
            stroke_width: 1.0,
        }
    }
}

pub struct PolyLine {
    pub entity: EntityID,
    pub points: Vec<(Float, Float)>,
    pub line_options: LineOptions,
}

impl PolyLine {
    pub fn new(
        entity: EntityID,
        points: Vec<(Float, Float)>,
        line_options: LineOptions,
    ) -> PolyLine {
        PolyLine {
            entity,
            points,
            line_options,
        }
    }
}

impl Entity for PolyLine {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::PolyLine
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for PolyLine {
    fn clone(&self) -> Self {
        PolyLine {
            entity: self.entity.clone(),
            points: self.points.clone(),
            line_options: self.line_options.clone(),
        }
    }
}

pub struct ShapeArrow {
    pub entity: EntityID,
    pub start: (Float, Float),
    pub end: (Float, Float),
    pub arrow_options: ArrowOptions,
}

impl Clone for ShapeArrow {
    fn clone(&self) -> Self {
        ShapeArrow {
            entity: self.entity.clone(),
            start: self.start,
            end: self.end,
            arrow_options: self.arrow_options.clone(),
        }
    }
}

impl Entity for ShapeArrow {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::ArrowShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct ArrowOptions {
    pub stroke_color: String,
    pub stroke_width: Float,
    pub arrow_size: Float,
}

impl Clone for ArrowOptions {
    fn clone(&self) -> Self {
        ArrowOptions {
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
            arrow_size: self.arrow_size,
        }
    }
}

impl ArrowOptions {
    pub fn new() -> ArrowOptions {
        ArrowOptions {
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            arrow_size: 10.0,
        }
    }
}

pub struct ShapeEllipse {
    pub entity: EntityID,
    pub radius: (Float, Float),
    pub ellipse_options: EllipseOptions,
}

impl Clone for ShapeEllipse {
    fn clone(&self) -> Self {
        ShapeEllipse {
            entity: self.entity.clone(),
            radius: self.radius,
            ellipse_options: self.ellipse_options.clone(),
        }
    }
}

impl ShapeEllipse {
    pub fn new(
        entity: EntityID,
        radius: (Float, Float),
        ellipse_options: EllipseOptions,
    ) -> ShapeEllipse {
        ShapeEllipse {
            entity,
            radius,
            ellipse_options,
        }
    }
}

impl Entity for ShapeEllipse {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::EllipseShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct EllipseOptions {
    //TODO: convert to Fill
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: Float,
}

impl Clone for EllipseOptions {
    fn clone(&self) -> Self {
        EllipseOptions {
            fill_color: self.fill_color.clone(),
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
        }
    }
}

impl EllipseOptions {
    pub fn new() -> EllipseOptions {
        EllipseOptions {
            fill_color: String::from("white"),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
        }
    }
}

pub struct ShapeImage {
    pub entity: EntityID,
    //base64 encoded image or empty if using file_path
    pub image: String,
    //path to image file on disk (optional)
    pub file_path: Option<String>,
    pub width_behavior: SizeBehavior,
    pub height_behavior: SizeBehavior,
}

impl Clone for ShapeImage {
    fn clone(&self) -> Self {
        ShapeImage {
            entity: self.entity.clone(),
            image: self.image.clone(),
            file_path: self.file_path.clone(),
            width_behavior: self.width_behavior.clone(),
            height_behavior: self.height_behavior.clone(),
        }
    }
}

impl Entity for ShapeImage {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::ImageShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeImage {
    pub fn new(entity: EntityID, image: String, size: (SizeBehavior, SizeBehavior)) -> ShapeImage {
        ShapeImage {
            entity,
            image,
            file_path: None,
            width_behavior: size.0,
            height_behavior: size.1,
        }
    }

    pub fn from_file(
        entity: EntityID,
        file_path: String,
        size: (SizeBehavior, SizeBehavior),
    ) -> ShapeImage {
        ShapeImage {
            entity,
            image: String::new(), // Empty as we're using file_path instead
            file_path: Some(file_path),
            width_behavior: size.0,
            height_behavior: size.1,
        }
    }
}

/// A container that allows children to be positioned with absolute coordinates
/// Children's positions are relative to the container's top-left corner
pub struct FreeContainer {
    pub entity: EntityID,
    pub children: Vec<(EntityID, (Float, Float))>, // Each child has a position relative to the container
    pub background_color: Option<String>,          // Optional background color
    pub border_color: Option<String>,              // Optional border color
    pub border_width: Float,                       // Border width (0 for no border)
}

impl Clone for FreeContainer {
    fn clone(&self) -> Self {
        FreeContainer {
            entity: self.entity.clone(),
            children: self.children.clone(),
            background_color: self.background_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width,
        }
    }
}

impl Entity for FreeContainer {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::FreeContainer
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FreeContainer {
    /// Create a new empty FreeContainer
    pub fn new(entity: EntityID) -> Self {
        FreeContainer {
            entity,
            children: Vec::new(),
            background_color: None,
            border_color: None,
            border_width: 0.0,
        }
    }

    /// Add a child to the container at the specified position
    pub fn add_child(&mut self, child_id: EntityID, position: (Float, Float)) {
        self.children.push((child_id, position));
    }

    /// Add multiple children at once with their positions
    pub fn with_children(
        mut self,
        children_with_positions: Vec<(EntityID, (Float, Float))>,
    ) -> Self {
        self.children.extend(children_with_positions);
        self
    }

    /// Set background color
    pub fn with_background_color(mut self, color: &str) -> Self {
        self.background_color = Some(color.to_string());
        self
    }

    /// Set border properties
    pub fn with_border(mut self, color: &str, width: Float) -> Self {
        self.border_color = Some(color.to_string());
        self.border_width = width;
        self
    }
}

// Arc options structure
#[derive(Default, Debug)]
pub struct ArcOptions {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: Float,
    pub filled: bool, // Whether to fill the arc sector or just draw the outline
}

impl Clone for ArcOptions {
    fn clone(&self) -> Self {
        ArcOptions {
            fill_color: self.fill_color.clone(),
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
            filled: self.filled,
        }
    }
}

impl ArcOptions {
    pub fn new() -> ArcOptions {
        ArcOptions {
            fill_color: String::from("none"),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            filled: false,
        }
    }
}

// Arc shape structure
pub struct ShapeArc {
    pub entity: EntityID,
    pub center: (Float, Float), // Center point of the arc
    pub radius: Float,          // Radius of the arc
    pub start_angle: Float,     // Start angle in degrees
    pub end_angle: Float,       // End angle in degrees
    pub arc_options: ArcOptions,
}

impl Clone for ShapeArc {
    fn clone(&self) -> Self {
        ShapeArc {
            entity: self.entity.clone(),
            center: self.center,
            radius: self.radius,
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            arc_options: self.arc_options.clone(),
        }
    }
}

impl ShapeArc {
    pub fn new(
        entity: EntityID,
        center: (Float, Float),
        radius: Float,
        start_angle: Float,
        end_angle: Float,
        arc_options: ArcOptions,
    ) -> ShapeArc {
        ShapeArc {
            entity,
            center,
            radius,
            start_angle,
            end_angle,
            arc_options,
        }
    }

    /// Normalize angles to 0-360 degree range
    pub fn normalize_angles(&self) -> (Float, Float) {
        let mut start = self.start_angle % 360.0;
        let mut end = self.end_angle % 360.0;

        if start < 0.0 {
            start += 360.0;
        }
        if end < 0.0 {
            end += 360.0;
        }

        (start, end)
    }

    /// Calculate the angle sweep of the arc
    pub fn angle_sweep(&self) -> Float {
        let (start, end) = self.normalize_angles();
        if end > start {
            end - start
        } else {
            360.0 - start + end
        }
    }

    /// Check if this is a major arc (> 180 degrees)
    pub fn is_major_arc(&self) -> bool {
        self.angle_sweep() > 180.0
    }
}

impl Entity for ShapeArc {
    fn get_id(&self) -> EntityID {
        self.entity.clone()
    }

    fn get_type(&self) -> EntityType {
        EntityType::ArcShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Type alias for custom component factory functions
/// Takes a map of attributes and a mutable reference to DiagramBuilder
/// Returns a Result with either a DiagramTreeNode or an error message
pub type CustomComponentFactory = Arc<
    dyn Fn(
            &Map<String, Value>,
            &mut crate::DiagramBuilder,
        ) -> Result<crate::diagram_builder::DiagramTreeNode, String>
        + Send
        + Sync,
>;

/// Registry for custom components
pub struct CustomComponentRegistry {
    factories: HashMap<String, CustomComponentFactory>,
}

impl CustomComponentRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// Register a new custom component with the given identifier and factory function
    pub fn register<F>(&mut self, component_type: &str, factory: F)
    where
        F: Fn(
                &Map<String, Value>,
                &mut crate::DiagramBuilder,
            ) -> Result<crate::diagram_builder::DiagramTreeNode, String>
            + Send
            + Sync
            + 'static,
    {
        self.factories
            .insert(component_type.to_string(), Arc::new(factory));
    }

    /// Create a component instance using the registered factory
    pub fn create_component(
        &self,
        component_type: &str,
        attributes: &Map<String, Value>,
        builder: &mut crate::DiagramBuilder,
    ) -> Result<crate::diagram_builder::DiagramTreeNode, String> {
        match self.factories.get(component_type) {
            Some(factory) => factory(attributes, builder),
            None => Err(format!("Unknown custom component type: {}", component_type)),
        }
    }

    /// Check if a component type is registered
    pub fn has_component(&self, component_type: &str) -> bool {
        self.factories.contains_key(component_type)
    }

    /// Get all registered component types
    pub fn get_registered_types(&self) -> Vec<&String> {
        self.factories.keys().collect()
    }

    pub fn get(
        &self,
        component_type: &str,
    ) -> Option<
        &Arc<
            dyn Fn(
                    &serde_json::Map<String, serde_json::Value>,
                    &mut crate::diagram_builder::DiagramBuilder,
                ) -> Result<crate::diagram_builder::DiagramTreeNode, String>
                + Send
                + Sync,
        >,
    > {
        self.factories.get(component_type)
    }
}

impl Default for CustomComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions for extracting common attribute types
impl CustomComponentRegistry {
    /// Helper to extract a string attribute with a default value
    pub fn get_string_attr(attrs: &Map<String, Value>, key: &str, default: &str) -> String {
        attrs
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    }

    /// Helper to extract a float attribute with a default value
    pub fn get_float_attr(attrs: &Map<String, Value>, key: &str, default: f64) -> Float {
        attrs.get(key).and_then(|v| v.as_f64()).unwrap_or(default) as Float
    }

    /// Helper to extract a boolean attribute with a default value
    pub fn get_bool_attr(attrs: &Map<String, Value>, key: &str, default: bool) -> bool {
        attrs.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    /// Helper to extract an integer attribute with a default value
    pub fn get_int_attr(attrs: &Map<String, Value>, key: &str, default: i64) -> i64 {
        attrs.get(key).and_then(|v| v.as_i64()).unwrap_or(default)
    }
}

#[cfg(test)]
mod custom_component_tests {
    use super::*;
    use crate::*;
    use serde_json::json;

    /// Test custom component: Badge
    /// Creates a rounded box with text and a colored background
    fn create_badge_component(
        attrs: &Map<String, Value>,
        builder: &mut DiagramBuilder,
    ) -> Result<crate::diagram_builder::DiagramTreeNode, String> {
        // Extract attributes with defaults
        let text = attrs
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("Badge")
            .to_string();

        let id = attrs
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let textNode = builder.new_text(id, &text, TextOptions::default());
        Ok(textNode)
    }

    #[test]
    fn test_custom_component_registry() {
        let mut registry = CustomComponentRegistry::new();

        // Register badge component
        registry.register("badge", create_badge_component);

        // Test that components are registered
        assert!(registry.has_component("badge"));
        assert!(!registry.has_component("unknown"));

        // Test getting registered types
        let types = registry.get_registered_types();
        assert_eq!(types.len(), 1);
        assert!(types.contains(&&"badge".to_string()));
    }

    #[test]
    fn test_create_badge_component() {
        let mut registry = CustomComponentRegistry::new();
        registry.register("badge", create_badge_component);

        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));

        // Create badge with custom attributes
        let attrs = json!({
            "text": "NEW",
            "background_color": "red",
            "text_color": "white",
            "font_size": 14.0,
            "padding": 6.0,
            "border_radius": 20.0
        });

        let attrs_map = attrs.as_object().unwrap();
        let badge = registry.create_component("badge", attrs_map, &mut builder);

        assert!(badge.is_ok());
        let badge_node = badge.unwrap();
        assert_eq!(badge_node.entity_type, EntityType::TextShape);
    }
}
