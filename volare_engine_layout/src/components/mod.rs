
//new type EntityID that is a u64
pub type EntityID = u64;

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
        _ => panic!("Invalid entity type"),
    }
}


/**
 * Boxes show a rectangle around the wrapped entity
 */
pub struct ShapeBox {
    pub entity: u64,
    //Each box wraps another entity
    pub wrapped_entity: u64,
    pub box_options: BoxOptions,
}

#[derive(Default)]
pub struct BoxOptions {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
    pub padding: f64,
    pub round_corners: bool,
    pub border_radius: f64,
}

impl BoxOptions {
    pub fn new() -> BoxOptions {
        BoxOptions {
            fill_color: String::from("white"),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            padding: 10.0,
            round_corners: false,
            border_radius: 0.0,
        }
    }
}

/* A group of entities */
pub struct ShapeGroup {
    pub entity: u64,
    pub elements: Vec<u64>,
}

pub struct ShapeText {
    pub entity: u64,
    pub text: String,
    pub text_options: TextOptions,
}

//struct with text options: font family, font size
#[derive(Default)]
pub struct TextOptions {
    pub font_family: String,
    pub font_size: f64,
    pub text_color: String,
}

pub struct VerticalStack {
    pub entity: u64,
    //List of entity ids
    pub elements: Vec<u64>,
}

pub struct HorizontalStack {
    pub entity: u64,
    //List of entity ids
    pub elements: Vec<u64>,
}

pub struct ShapeLine {
    pub entity: u64,
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub line_options: LineOptions,
}

#[derive(Default)]
pub struct LineOptions {
    pub stroke_color: String,
    pub stroke_width: f64,
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
    pub entity: u64,
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub arrow_options: ArrowOptions,
}

#[derive(Default)]
pub struct ArrowOptions {
    pub stroke_color: String,
    pub stroke_width: f64,
    pub arrow_size: f64,
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
    pub entity: u64,
    pub center: (f64, f64),
    pub radius: (f64, f64),
    pub ellipse_options: EllipseOptions,
}

#[derive(Default)]
pub struct EllipseOptions {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
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
    pub entity: u64,
    //base64 encoded image
    pub image: String,
    pub preferred_size: (f64, f64),
}


/* A table contains a list of rows, each row has a cell 
* which is a group that contains other elements.

Tables are defined with an array of cells and the number of columns
*/
pub struct Table {
    pub entity: u64,
    pub cols: usize, 
    pub cells: Vec<u64>
}