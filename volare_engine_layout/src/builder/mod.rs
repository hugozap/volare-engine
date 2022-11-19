use crate::components::*;
use crate::session::*;
/* The Builder object has an API to create the Diagram tree.
 * This is the object used by consumers of this library */

 /**
  * The base node element used to represent the Diagram Tree structure.
  * Each node has a type and the index of the element in the corresponding vector.
  */
 struct DiagramTreeNode{
    entity_type: EntityType,
    index: usize,
    children: Vec<Box<DiagramTreeNode>>,
 }

 //TODO: What would be a better name for this?
 //Maybe we don't need the singleton session object?
 struct DiagramBuilder {

    //This lists have all the elements of the diagram
    boxes: Vec<ShapeBox>,
    groups: Vec<ShapeGroup>,
    texts: Vec<ShapeText>,
    horizontal_stacks: Vec<HorizontalStack>,
    vertical_stacks: Vec<VerticalStack>,
    ellipses: Vec<ShapeEllipse>,
    lines: Vec<ShapeLine>,
    arrows: Vec<ShapeArrow>,
    tables: Vec<Table>,

 }

 impl DiagramBuilder {
    pub fn new() -> DiagramBuilder {
        Session::get_instance().reset();
        DiagramBuilder {
            boxes: Vec::new(),
            groups: Vec::new(),
            texts: Vec::new(),
            horizontal_stacks: Vec::new(),
            vertical_stacks: Vec::new(),
            ellipses: Vec::new(),
            lines: Vec::new(),
            arrows: Vec::new(),
            tables: Vec::new(),
        }
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
        }
    }

    // Wraps an element in a box
    pub fn new_box(&mut self, options: &BoxOptions, child: &DiagramTreeNode) -> DiagramTreeNode {
        let session = Session::get_instance();
        let box_index = self.boxes.len();
        let box_id = session.new_entity(EntityType::Box);
        let child_entity = self.get_entity(child.entity_type, child.index);
        let sbox = ShapeBox::new(box_id, child_entity.get_id(), options);
        self.boxes.push(sbox);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::Box,
            index: box_index,
            children: Vec::new(),
        };
        node.children.push(Box::new(child.clone()));
        node
    }

    pub fn new_group(&mut self, children: Vec<&DiagramTreeNode>) -> DiagramTreeNode {
        let session = Session::get_instance();
        let group_index = self.groups.len();
        let group_id = session.new_entity(EntityType::Group);
        let sgroup = ShapeGroup{
            id: group_id,
            children: Vec::new(),
        };
        
        //set children
        for child in children {
            let child_entity = self.get_entity(child.entity_type, child.index);
            sgroup.children.push(child_entity.get_id());
        }

        self.groups.push(sgroup);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::Group,
            index: group_index,
            children: Vec::new(),
        };
        for child in children {
            node.children.push(Box::new(child.clone()));
        }
        node
    }
 }
