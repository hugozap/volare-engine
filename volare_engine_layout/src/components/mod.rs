
use std::any::Any;
//new type EntityID that is a u64
pub type EntityID = usize;

pub trait Entity {
    fn get_id(&self) -> EntityID;
    fn get_type(&self) -> EntityType;
    //as_any
    fn as_any(&self) -> &dyn Any;
}

pub struct Point {
    pub x: f64,
    pub y: f64,
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
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

pub struct Size {
    pub w: f64,
    pub h: f64,
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
    pub fn new(w: f64, h: f64) -> Self {
        Size { w, h }
    }
}

//default implementation of Entity
impl dyn Entity {

    pub fn as_group(&self) -> Option<&ShapeGroup> {
        self.as_any().downcast_ref::<ShapeGroup>()
    }

    pub fn as_horizontal_stack(&self) -> Option<&HorizontalStack> {
        self.as_any().downcast_ref::<HorizontalStack>()
    }

    pub fn as_vertical_stack(&self) -> Option<&VerticalStack> {
        self.as_any().downcast_ref::<VerticalStack>()
    }

    pub fn as_text(&self) -> Option<&ShapeText> {
        self.as_any().downcast_ref::<ShapeText>()
    }

    pub fn as_line(&self) -> Option<&ShapeLine> {
        self.as_any().downcast_ref::<ShapeLine>()
    }

    pub fn as_arrow(&self) -> Option<&ShapeArrow> {
        self.as_any().downcast_ref::<ShapeArrow>()
    }

    pub fn as_ellipse(&self) -> Option<&ShapeEllipse> {
        self.as_any().downcast_ref::<ShapeEllipse>()
    }

    pub fn as_image(&self) -> Option<&ShapeImage> {
        self.as_any().downcast_ref::<ShapeImage>()
    }
    pub fn as_table(&self) -> Option<&Table> {
        self.as_any().downcast_ref::<Table>()
    }
}

//Note: add new items to the end of the enum to avoid breaking the serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    BoxShape,
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
}

pub fn get_entity_type(entity_id: EntityID) -> EntityType {
    match (entity_id >> 32) as u32 {
        0 => EntityType::BoxShape,
        1 => EntityType::TextShape,
        2 => EntityType::LineShape,
        3 => EntityType::ArrowShape,
        4 => EntityType::EllipseShape,
        5 => EntityType::ImageShape,
        6 => EntityType::GroupShape,
        7 => EntityType::VerticalStackShape,
        8 => EntityType::HorizontalStackShape,
        9 => EntityType::TableShape,
        _ => panic!("Invalid entity type"),
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
            entity: self.entity,
            wrapped_entity: self.wrapped_entity,
            box_options: self.box_options.clone(),
        }
    }
}

impl Entity for ShapeBox {
    fn get_id(&self) -> EntityID {
        self.entity
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

#[derive(Default, Debug)]
pub struct BoxOptions {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
    pub padding: f64,
    pub border_radius: f64,
}

impl Clone for BoxOptions {
    fn clone(&self) -> Self {
        BoxOptions {
            fill_color: self.fill_color.clone(),
            stroke_color: self.stroke_color.clone(),
            stroke_width: self.stroke_width,
            padding: self.padding,
            border_radius: self.border_radius,
        }
    }
}

impl BoxOptions {
    pub fn new() -> BoxOptions {
        BoxOptions {
            fill_color: String::from("white"),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            padding: 10.0,
            border_radius: 0.0,
        }
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
            entity: self.entity,
            elements: self.elements.clone(),
        }
    }
}

impl Entity for ShapeGroup {
    fn get_id(&self) -> EntityID {
        self.entity
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
            entity: self.entity,
            text: self.text.clone(),
        }
    }
}

impl Entity for TextLine {
    fn get_id(&self) -> EntityID {
        self.entity
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
            entity: self.entity,
            text: self.text.clone(),
            text_options: self.text_options.clone(),
            lines: self.lines.clone(),
        }
    }
}


impl ShapeText {
    pub fn new(entity: EntityID, text: &str, text_options: TextOptions, lines: &[EntityID]) -> ShapeText {
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
        self.entity
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
}

impl Clone for TextOptions {
    fn clone(&self) -> Self {
        TextOptions {
            font_family: self.font_family.clone(),
            font_size: self.font_size,
            text_color: self.text_color.clone(),
            line_width: self.line_width,
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
        }
    }
}

pub struct VerticalStack {
    pub entity: EntityID,
    //List of entity ids
    pub elements: Vec<EntityID>,
}

impl Clone for VerticalStack {
    fn clone(&self) -> Self {
        VerticalStack {
            entity: self.entity,
            elements: self.elements.clone(),
        }
    }
}

impl Entity for VerticalStack {
    fn get_id(&self) -> EntityID {
        self.entity
    }

    fn get_type(&self) -> EntityType {
        EntityType::VerticalStackShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
    

pub struct HorizontalStack {
    pub entity: EntityID,
    //List of entity ids
    pub elements: Vec<EntityID>,
}

impl Clone for HorizontalStack {
    fn clone(&self) -> Self {
        HorizontalStack {
            entity: self.entity,
            elements: self.elements.clone(),
        }
    }
}

impl Entity for HorizontalStack {
    fn get_id(&self) -> EntityID {
        self.entity
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
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub line_options: LineOptions,
}

impl Clone for ShapeLine {
    fn clone(&self) -> Self {
        ShapeLine {
            entity: self.entity,
            start: self.start,
            end: self.end,
            line_options: self.line_options.clone(),
        }
    }
}

impl ShapeLine {
    pub fn new(line_id: EntityID, options: LineOptions) -> ShapeLine {
        ShapeLine {
            entity: line_id,
            start: (0.0, 0.0),
            end: (0.0, 0.0),
            line_options: options,
        }
    }
}


impl Entity for ShapeLine {
    fn get_id(&self) -> EntityID {
        self.entity
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
    pub stroke_width: f64,
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

pub struct ShapeArrow {
    pub entity: EntityID,
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub arrow_options: ArrowOptions,
}

impl Clone for ShapeArrow {
    fn clone(&self) -> Self {
        ShapeArrow {
            entity: self.entity,
            start: self.start,
            end: self.end,
            arrow_options: self.arrow_options.clone(),
        }
    }
}

impl Entity for ShapeArrow {
    fn get_id(&self) -> EntityID {
        self.entity
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
    pub stroke_width: f64,
    pub arrow_size: f64,
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
    pub center: (f64, f64),
    pub radius: (f64, f64),
    pub ellipse_options: EllipseOptions,
}

impl Clone for ShapeEllipse {
    fn clone(&self) -> Self {
        ShapeEllipse {
            entity: self.entity,
            center: self.center,
            radius: self.radius,
            ellipse_options: self.ellipse_options.clone(),
        }
    }
}

impl ShapeEllipse {
    pub fn new(entity: EntityID, center: (f64, f64), radius: (f64, f64), ellipse_options: EllipseOptions) -> ShapeEllipse {
        ShapeEllipse {
            entity,
            center,
            radius,
            ellipse_options,
        }
    }
}

impl Entity for ShapeEllipse {
    fn get_id(&self) -> EntityID {
        self.entity
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
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
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
    //base64 encoded image
    pub image: String,
    pub preferred_size: (f64, f64),
}

impl Clone for ShapeImage {
    fn clone(&self) -> Self {
        ShapeImage {
            entity: self.entity,
            image: self.image.clone(),
            preferred_size: self.preferred_size,
        }
    }
}

impl Entity for ShapeImage {
    fn get_id(&self) -> EntityID {
        self.entity
    }

    fn get_type(&self) -> EntityType {
        EntityType::ImageShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub struct TableOptions {
    pub fill_color: String,
    pub border_color: String,
    pub border_width: usize,
    pub cell_padding: usize,
}

//defaults
impl Default for TableOptions {
    fn default() -> Self {
        TableOptions {
            fill_color: String::from("white"),
            border_color: String::from("black"),
            border_width: 1,
            cell_padding: 20,
        }
    }
}

impl Clone for TableOptions {
    fn clone(&self) -> Self {
        TableOptions {
            fill_color: self.fill_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width,
            cell_padding: self.cell_padding,
        }
    }
}
/* A table contains a list of rows, each row has a cell 
* which is a group that contains other elements.

Tables are defined with an array of cells and the number of columns
*/
pub struct Table {
    pub entity: EntityID,
    pub cols: usize, 
    pub cells: Vec<EntityID>,
    pub lines: Vec<EntityID>,
    pub table_options: TableOptions,
}

//new


impl Clone for Table {
    fn clone(&self) -> Self {
        Table {
            entity: self.entity,
            cols: self.cols,
            cells: self.cells.clone(),
            lines: self.lines.clone(),
            table_options: self.table_options.clone(),
        }
    }
}

//constructor that receives only the table options
impl Table {
    pub fn new(entity: EntityID,cells: Vec<EntityID>,  cols: usize, table_options: TableOptions) -> Table {
        Table {
            entity,
            cols,
            cells,
            lines: Vec::new(),
            table_options,
        }
    }
}

impl Entity for Table {
    fn get_id(&self) -> EntityID {
        self.entity
    }

    fn get_type(&self) -> EntityType {
        EntityType::TableShape
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}