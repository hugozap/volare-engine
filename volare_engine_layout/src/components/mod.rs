
use std::any::Any;
//new type EntityID that is a u64
pub type EntityID = u64;

pub trait Entity {
    fn get_id(&self) -> EntityID;
    fn get_type(&self) -> EntityType;
    //as_any
    fn as_any(&self) -> &dyn Any;
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
pub struct ShapeBox {
    pub entity: EntityID,
    //Each box wraps another entity
    pub wrapped_entity: u64,
    pub box_options: BoxOptions,
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
    pub entity: EntityID,
    pub elements: Vec<u64>,
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

pub struct ShapeText {
    pub entity: EntityID,
    pub text: String,
    pub text_options: TextOptions,
}

impl ShapeText{
    pub fn new(entity: EntityID, text: &str, text_options: TextOptions) -> ShapeText {
        ShapeText {
            entity,
            text: text.to_string(),
            text_options,
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
#[derive(Default)]
pub struct TextOptions {
    pub font_family: String,
    pub font_size: f64,
    pub text_color: String,
}

pub struct VerticalStack {
    pub entity: u64,
    //List of entity ids
    pub elements: Vec<EntityID>,
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
    pub entity: u64,
    //List of entity ids
    pub elements: Vec<EntityID>,
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


/* A table contains a list of rows, each row has a cell 
* which is a group that contains other elements.

Tables are defined with an array of cells and the number of columns
*/
pub struct Table {
    pub entity: EntityID,
    pub cols: usize, 
    pub cells: Vec<EntityID>
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