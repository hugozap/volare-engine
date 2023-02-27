use std::cell::RefCell;
use std::rc::Rc;

/**
 * This object encapsulates diagram creation logic in a user friendly API
 * Usage:
 *```rust 
 * let builder = DiagramBuilder::new();
 * let group = builder.new_group(
 *   builder.new_box(builder.new_text("Hello World!"), BoxOptions{fill_color: "white".to_string(), stroke_color: "black".to_string(), stroke_width: 1.0, padding: 10.0, round_corners: false, border_radius: 0.0}),
 * );
 * 
 *
 *
 */
//use TextOptions
use crate::{components::*, get_entity_index_from_id};
//wrap library features in a struct
use crate::layout;

const MAX_ENTITIES: usize = 1000;
pub struct Session {
    pub measure_text: Option<fn(&str, &TextOptions) -> (f64, f64)>,
    pub entities: [EntityID; MAX_ENTITIES],
    pub positions: [f64; MAX_ENTITIES * 2],
    pub sizes: [f64; MAX_ENTITIES * 2],


    // Components
    boxes: Vec<ShapeBox>,
    groups: Vec<ShapeGroup>,
    texts: Vec<ShapeText>,
    textlines: Vec<TextLine>,
    horizontal_stacks: Vec<HorizontalStack>,
    vertical_stacks: Vec<VerticalStack>,
    ellipses: Vec<ShapeEllipse>,
    lines: Vec<ShapeLine>,
    arrows: Vec<ShapeArrow>,
    tables: Vec<Table>,
    images: Vec<ShapeImage>,
    
}

// Stores the type of entity and the index of the entity in the corresponding vector
// Used when building the diagram tree.
#[derive(Debug, Clone)]
 pub struct DiagramTreeNode{

    pub entity_type: EntityType,
    // Index of the entity in the corresponding vector
    pub index: usize,
    pub children: Vec<Box<DiagramTreeNode>>,
 }

 impl DiagramTreeNode {
    fn new(entity_type: EntityType, index: usize) -> DiagramTreeNode {
        DiagramTreeNode {
            entity_type,
            index,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: DiagramTreeNode) {
        self.children.push(Box::new(child));
    }
 }

/* New architecture (data driven)
 * We have an array of entities, each entity is an id
 * The id has 64 bits, we can use 32 bits for the type and 32 bits for the index
 * To get the type: id >> 32
 * To get the index: id & 0xFFFFFFFF
 * We have a type enum with all the types
*/

impl Session {
    pub fn new() -> Session {
        Session {
            measure_text: Some(|_text, _text_options| {
                (0.0,0.0)
            }),
            entities: [0; MAX_ENTITIES],
            positions: [0.0; MAX_ENTITIES * 2],
            sizes: [0.0; MAX_ENTITIES * 2],
             boxes: Vec::new(),
            groups: Vec::new(),
            texts: Vec::new(),
            textlines: Vec::new(),
            horizontal_stacks: Vec::new(),
            vertical_stacks: Vec::new(),
            ellipses: Vec::new(),
            lines: Vec::new(),
            arrows: Vec::new(),
            tables: Vec::new(),
            images: Vec::new(),
       }
    }

    /* Create a new entity of a given type
     * Returns the id of the new entity
     * We have another array with the positions of the entities
     * in the same index. So they are fast to access
     */
    pub fn new_entity(&mut self, entity_type: EntityType) -> EntityID {
        let index = self.entities.iter().position(|&x| x == 0).unwrap();
        let id = ((entity_type as u64) << 32) | (index as u64);
        self.entities[index] = id;
        id
    }

    pub fn clear_cache(&mut self) {
        self.entities = [0; MAX_ENTITIES];
        self.positions = [0.0; MAX_ENTITIES * 2];
        self.sizes = [0.0; MAX_ENTITIES * 2];
    }

    //set the measure_text function
    pub fn set_measure_text_fn(&mut self, measure_text: fn(&str, &TextOptions) -> (f64, f64)) {
        println!("Setting measure text function");
        self.measure_text = Option::Some(measure_text);

    }

    

    //get the position of an entity
    pub fn get_position(&self, entity_id: EntityID) -> (f64, f64) {
        let index = get_entity_index_from_id(entity_id);
        (self.positions[index * 2], self.positions[index * 2 + 1])
    }

    pub fn set_position(&mut self, entity_id: EntityID, x: f64, y: f64) {
        let index = get_entity_index_from_id(entity_id);
        self.positions[index * 2] = x;
        self.positions[index * 2 + 1] = y;
    }

    //get the size of an entity
    pub fn get_size(&self, entity_id: EntityID) -> (f64, f64) {
        let index = get_entity_index_from_id(entity_id);
        (self.sizes[index * 2], self.sizes[index * 2 + 1])
    }

    pub fn set_size(&mut self, entity_id: EntityID, width: f64, height: f64) {
        let index = get_entity_index_from_id(entity_id);
        self.sizes[index * 2] = width;
        self.sizes[index * 2 + 1] = height;
    }


    // Returns the entityID given the entity type and the index
    pub fn get_entity_id(&self, entity_type: EntityType, index: usize) -> EntityID {
        match entity_type {
            EntityType::GroupShape => self.groups[index].get_id(),
            EntityType::TextShape => self.texts[index].get_id(),
            EntityType::HorizontalStackShape => self.horizontal_stacks[index].get_id(),
            EntityType::VerticalStackShape => self.vertical_stacks[index].get_id(),
            EntityType::EllipseShape => self.ellipses[index].get_id(),
            EntityType::LineShape => self.lines[index].get_id(),
            EntityType::ArrowShape => self.arrows[index].get_id(),
            EntityType::TableShape => self.tables[index].get_id(),
            EntityType::ImageShape => self.images[index].get_id(),
            EntityType::BoxShape => self.boxes[index].get_id(),
            EntityType::TextLine =>  self.textlines[index].get_id(),
        }
    }
  

    // This methods are used to build the diagram tree
    // TODO: review the architecture of this

    // Wraps an element in a box
    pub fn new_box(&mut self, child: DiagramTreeNode,options: BoxOptions) -> DiagramTreeNode {
        let box_index = self.boxes.len();
        let box_id = self.new_entity(EntityType::BoxShape);
        // we need to get the wrapped entity id
        
       
        let sbox = ShapeBox::new(box_id, self.get_entity_id(child.entity_type, child.index ), options);
        self.boxes.push(sbox);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::BoxShape,
            index: box_index,
            children: Vec::new(),
        };
        node.children.push(Box::new(child.clone()));
        node
    }


    // Creates a new Text element
    // text: the text to display
    // options: the options for the text
    // ```rust
    // let text = session.new_text("Hello World", TextOptions::new());
    // ```
    pub fn new_text(&mut self, text: &str, options: TextOptions) -> DiagramTreeNode {
        let text_index = self.texts.len();
        let text_id = self.new_entity(EntityType::TextShape);
        let text = ShapeText::new(text_id, text, options);
        self.texts.push(text);
        DiagramTreeNode::new(EntityType::TextShape, text_index)
    }

    pub fn new_line(&mut self, options: LineOptions) -> DiagramTreeNode {
        let line_index = self.lines.len();
        let line_id = self.new_entity(EntityType::LineShape);
        let line = ShapeLine::new(line_id, options);
        self.lines.push(line);
        DiagramTreeNode::new(EntityType::LineShape, line_index)
    }

    pub fn new_elipse(&mut self,center: (f64,f64) , radius: (f64,f64),  options: EllipseOptions) -> DiagramTreeNode {
        let ellipse_index = self.ellipses.len();
        let ellipse_id = self.new_entity(EntityType::EllipseShape);
        let ellipse = ShapeEllipse::new(ellipse_id, center, radius, options);
        self.ellipses.push(ellipse);
        DiagramTreeNode::new(EntityType::EllipseShape, ellipse_index)
    }

    // Creates a new Group.
    pub fn new_group(&mut self, children: Vec<DiagramTreeNode>) -> DiagramTreeNode {
        let group_index = self.groups.len();
        let group_id = self.new_entity(EntityType::GroupShape);
        let mut sgroup = ShapeGroup{
         entity: group_id,
         elements: Vec::new(),   
        };
         let mut node = DiagramTreeNode {
            entity_type: EntityType::GroupShape,
            index: group_index,
            children: Vec::new(),
        };
 
     
        //set children
        for child in children {
            let entity_id = self.get_entity_id(child.entity_type, child.index);
            sgroup.elements.push(entity_id);
            node.add_child(child)
        }

        self.groups.push(sgroup);

        
        node
    }  
}

// element list accessors
impl Session {
    pub fn get_text(&self, index: usize) -> &ShapeText {
        &self.texts[index]
    }

    pub fn get_group(&self, index: usize) -> &ShapeGroup {
        &self.groups[index]
    }

    pub fn get_horizontal_stack(&self, index: usize) -> &HorizontalStack {
        &self.horizontal_stacks[index]
    }

    pub fn get_vertical_stack(&self, index: usize) -> &VerticalStack {
        &self.vertical_stacks[index]
    }

    pub fn get_ellipse(&self, index: usize) -> &ShapeEllipse {
        &self.ellipses[index]
    }

    pub fn get_line(&self, index: usize) -> &ShapeLine {
        &self.lines[index]
    }

    pub fn get_arrow(&self, index: usize) -> &ShapeArrow {
        &self.arrows[index]
    }

    pub fn get_table(&self, index: usize) -> &Table {
        &self.tables[index]
    }

    pub fn get_image(&self, index: usize) -> &ShapeImage {
        &self.images[index]
    }

    pub fn get_box(&self, index: usize) -> &ShapeBox {
        &self.boxes[index]
    }
}


//test
#[cfg(test)]
mod tests {
    use crate::{get_entity_type_from_id, get_entity_index_from_id};

    use super::*;

    #[test]
    fn test_session() {
        let mut session = Session::new();
        
        
        session.set_measure_text_fn(|text, text_options| {
            let textW:f64 = text.len() as f64 * text_options.font_size as f64;

            (
                textW,
                text_options.font_size.into(),
            )
        });
        let (w, h) = session.measure_text.unwrap()(
            "hello",
            &TextOptions {
                font_size: 12.0,
                ..Default::default()
            },
        );
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }

    //Test entities
    #[test]
    fn test_session_entities() {
        let mut session = Session::new();
        let id = session.new_entity(EntityType::GroupShape);
        //the id has 32 bits for the index and 32 bits for the type
        let index = get_entity_index_from_id(id);
        assert_eq!(index, 0);
        let entity_type = get_entity_type_from_id(id);
        assert_eq!(entity_type, EntityType::GroupShape);
    }
}
