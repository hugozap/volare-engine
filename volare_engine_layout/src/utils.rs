use crate::components::*;
// Returns the entity type given its id.
pub fn get_entity_type_from_id(entity_id: EntityID) -> EntityType {
    match (entity_id >> 32) as u8 {
        0 => EntityType::GroupShape,
        1 => EntityType::HorizontalStackShape,
        2 => EntityType::VerticalStackShape,
        3 => EntityType::TextShape,
        4 => EntityType::LineShape,
        5 => EntityType::ArrowShape,
        _ => EntityType::GroupShape,
    }
}