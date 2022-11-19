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
use crate::components::*;
//wrap library features in a struct

const MAX_ENTITIES: usize = 10000;
pub struct Session {
    pub measure_text: Option<fn(&str, &TextOptions) -> (f64, f64)>,
    pub entities: [EntityID; MAX_ENTITIES],
    pub positions: [f64; MAX_ENTITIES * 2],
    pub sizes: [f64; MAX_ENTITIES * 2],


    // Components
    boxes: Vec<ShapeBox>,
    groups: Vec<ShapeGroup>,
    texts: Vec<ShapeText>,
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
    entity_type: EntityType,
    index: usize,
    children: Vec<Box<DiagramTreeNode>>,
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
        self.measure_text = Option::Some(measure_text);
    }

    //get singleton instance
    pub fn get_instance() -> &'static mut Session {
        static mut INSTANCE: Option<Session> = None;
        unsafe {
            INSTANCE.get_or_insert_with(Session::new);
            INSTANCE.as_mut().unwrap()
        }
    }

    //get the position of an entity
    pub fn get_position(&self, entity_id: EntityID) -> (f64, f64) {
        let index = get_index(entity_id);
        (self.positions[index * 2], self.positions[index * 2 + 1])
    }

    pub fn set_position(&mut self, entity_id: EntityID, x: f64, y: f64) {
        let index = get_index(entity_id);
        self.positions[index * 2] = x;
        self.positions[index * 2 + 1] = y;
    }

    //get the size of an entity
    pub fn get_size(&self, entity_id: EntityID) -> (f64, f64) {
        let index = get_index(entity_id);
        (self.sizes[index * 2], self.sizes[index * 2 + 1])
    }

    pub fn set_size(&mut self, entity_id: EntityID, width: f64, height: f64) {
        let index = get_index(entity_id);
        self.sizes[index * 2] = width;
        self.sizes[index * 2 + 1] = height;
    }
  
    // Returns the Entity for the specified type and index
    pub fn get_entity(&self, entity_type: EntityType, index: usize) -> &dyn Entity {
        match entity_type {
            EntityType::GroupShape => &self.groups[index],
            EntityType::TextShape => &self.texts[index],
            EntityType::HorizontalStackShape => &self.horizontal_stacks[index],
            EntityType::VerticalStackShape => &self.vertical_stacks[index],
            EntityType::EllipseShape => &self.ellipses[index],
            EntityType::LineShape => &self.lines[index],
            EntityType::ArrowShape => &self.arrows[index],
            EntityType::TableShape => &self.tables[index],
            EntityType::ImageShape => &self.images[index],
            EntityType::BoxShape => &self.boxes[index],
        }
    }

    // This methods are used to build the diagram tree
    // TODO: review the architecture of this

    // Wraps an element in a box
    pub fn new_box(&mut self, child: DiagramTreeNode,options: BoxOptions) -> DiagramTreeNode {
        let box_index = self.boxes.len();
        let box_id = self.new_entity(EntityType::BoxShape);
        let child_entity = self.get_entity(child.entity_type, child.index);
        let sbox = ShapeBox::new(box_id, child_entity.get_id(), options);
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
            let child_entity = self.get_entity(child.entity_type, child.index);
            sgroup.elements.push(child_entity.get_id());
            node.add_child(child)
        }

        self.groups.push(sgroup);

        
        node
    }  
}

pub fn get_index(entity_id: EntityID) -> usize {
    (entity_id & 0xFFFFFFFF) as usize
}


//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let mut session = Session::new();
        session.set_measure_text_fn(|text, text_options| {
            (
                text.len() as f64 * text_options.font_size,
                text_options.font_size,
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

    #[test]
    fn test_session_singleton() {
        let session = Session::get_instance();
        session.set_measure_text_fn(|text, text_options| {
            (
                text.len() as f64 * text_options.font_size,
                text_options.font_size,
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

    #[test]
    fn test_session_singleton_2() {
        let session = Session::get_instance();
        session.set_measure_text_fn(|text, text_options| {
            (
                text.len() as f64 * text_options.font_size,
                text_options.font_size,
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
        let session = Session::get_instance();
        let id = session.new_entity(EntityType::GroupShape);
        assert_eq!(id, 0);
        let index = get_index(id);
        assert_eq!(index, 6);
        let entity_type = get_entity_type(id);
        assert_eq!(entity_type, EntityType::GroupShape);
    }
}
