/**
 * Main data structure for the layout engine.
 * Contains the positions and sizes of all the elements.
 * They are allocated in the stack for super fast access.
 *
 * Check the utils module for utilties for accessing the entity id information.
 * The session also stores host dependent information, such as the text measurement function.
 * Other components must set this information before using the session.
 *
 *
 */
//use TextOptions
use crate::shape_text::TextOptions;
//wrap library features in a struct

//new type EntityID that is a u64
pub type EntityID = u64;

const MAX_CAPACITY: usize = 10000;
pub struct Session {
    pub measure_text: Option<fn(&str, &TextOptions) -> (f64, f64)>,
    pub entities: [EntityID; MAX_CAPACITY],
    pub positions: [f64; MAX_CAPACITY * 2],
    pub sizes: [f64; MAX_CAPACITY * 2],
}

/* New architecture (data driven)
 * We have an array of entities, each entity is an id
 * The id has 64 bits, we can use 32 bits for the type and 32 bits for the index
 * To get the type: id >> 32
 * To get the index: id & 0xFFFFFFFF
 * We have a type enum with all the types
*/

//Note: add new items to the end of the enum to avoid breaking the serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    DiagramLayout,
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

impl Session {
    pub fn new() -> Session {
        Session {
            measure_text: Some(|text, _text_options| {
                panic!("measure_text not implemented");
            }),
            entities: [0; MAX_CAPACITY],
            positions: [0.0; MAX_CAPACITY * 2],
            sizes: [0.0; MAX_CAPACITY * 2],
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
        self.entities = [0; MAX_CAPACITY];
        self.positions = [0.0; MAX_CAPACITY * 2];
        self.sizes = [0.0; MAX_CAPACITY * 2];
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
}

pub fn get_index(entity_id: EntityID) -> usize {
    (entity_id & 0xFFFFFFFF) as usize
}

pub fn get_type(entity_id: EntityID) -> EntityType {
    match (entity_id >> 32) as u32 {
        0 => EntityType::DiagramLayout,
        1 => EntityType::BoxShape,
        2 => EntityType::TextShape,
        3 => EntityType::LineShape,
        4 => EntityType::ArrowShape,
        5 => EntityType::EllipseShape,
        6 => EntityType::ImageShape,
        7 => EntityType::GroupShape,
        8 => EntityType::VerticalStackShape,
        9 => EntityType::HorizontalStackShape,
        _ => panic!("Invalid entity type"),
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let mut session = Session::new();
        session.set_measure_text_fn(|text, textOptions| {
            (
                text.len() as f64 * textOptions.font_size,
                textOptions.font_size,
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
        let mut session = Session::get_instance();
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
        let mut session = Session::get_instance();
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
        let mut session = Session::get_instance();
        let id = session.new_entity(EntityType::DiagramLayout);
        assert_eq!(id, 0);
        let index = get_index(id);
        assert_eq!(index, 0);
        let entity_type = get_type(id);
        assert_eq!(entity_type, EntityType::DiagramLayout);
    }
}
