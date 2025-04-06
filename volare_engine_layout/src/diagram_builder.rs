use std::collections::HashMap;

/**
 * This object encapsulates diagram creation logic.
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

pub struct DiagramBuilder {
    pub measure_text: Option<fn(&str, &TextOptions) -> (f64, f64)>,
    pub entities: Vec<EntityID>,
    pub positions: HashMap<EntityID, Point>,
    pub sizes: HashMap<EntityID, Size>,
    entity_id_counter: usize,

    // Components
    boxes: HashMap<EntityID, ShapeBox>,
    rectangles: HashMap<EntityID, ShapeRect>,
    groups: HashMap<EntityID, ShapeGroup>,
    texts: HashMap<EntityID, ShapeText>,
    textlines: HashMap<EntityID, TextLine>,
    horizontal_stacks: HashMap<EntityID, HorizontalStack>,
    vertical_stacks: HashMap<EntityID, VerticalStack>,
    ellipses: HashMap<EntityID, ShapeEllipse>,
    lines: HashMap<EntityID, ShapeLine>,
    arrows: HashMap<EntityID, ShapeArrow>,
    tables: HashMap<EntityID, Table>,
    images: HashMap<EntityID, ShapeImage>,
    polylines: HashMap<EntityID, PolyLine>,
}

// Stores the type of entity and the index of the entity in the corresponding vector
// Used when building the diagram tree.
#[derive(Debug, Clone)]
pub struct DiagramTreeNode {
    pub entity_type: EntityType,
    // Index of the entity in the corresponding vector
    pub entity_id: EntityID,
    pub children: Vec<Box<DiagramTreeNode>>,
}

impl DiagramTreeNode {
    fn new(entity_type: EntityType, id: EntityID) -> DiagramTreeNode {
        DiagramTreeNode {
            entity_type,
            entity_id: id,
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

impl DiagramBuilder {
    pub fn new() -> DiagramBuilder {
        DiagramBuilder {
            entity_id_counter: 0,
            measure_text: Some(|_text, _text_options| (0.0, 0.0)),
            entities: Vec::new(),
            positions: HashMap::new(),
            sizes: HashMap::new(),
            boxes: HashMap::new(),
            rectangles: HashMap::new(),
            groups: HashMap::new(),
            texts: HashMap::new(),
            textlines: HashMap::new(),
            horizontal_stacks: HashMap::new(),
            vertical_stacks: HashMap::new(),
            ellipses: HashMap::new(),
            lines: HashMap::new(),
            arrows: HashMap::new(),
            tables: HashMap::new(),
            images: HashMap::new(),
            polylines: HashMap::new(),
        }
    }

    /* Create a new entity of a given type
     * Returns the id of the new entity
     * We have another array with the positions of the entities
     * in the same index. So they are fast to access
     */
    pub fn new_entity(&mut self, entity_type: EntityType) -> EntityID {
        self.entity_id_counter += 1;
        let id = self.entity_id_counter;
        println!("Creating new entity with id {}", id);
        self.entities.push(id);
        self.positions.insert(id, Point::new(0.0, 0.0));
        self.sizes.insert(id, Size::new(0.0, 0.0));
        id
    }

    pub fn clear_cache(&mut self) {
        //clear entities vector
        self.entities = Vec::new();
        self.positions = HashMap::new();
        self.sizes = HashMap::new();
    }

    //set the measure_text function
    pub fn set_measure_text_fn(&mut self, measure_text: fn(&str, &TextOptions) -> (f64, f64)) {
        println!("Setting measure text function");
        self.measure_text = Option::Some(measure_text);
    }

    //get the position of an entity
    pub fn get_position(&self, entity_id: EntityID) -> (f64, f64) {
        let pos = self.positions.get(&entity_id).unwrap();
        (pos.x, pos.y)
    }

    pub fn set_position(&mut self, entity_id: EntityID, x: f64, y: f64) {
        println!("Setting position of entity {} to ({}, {})", entity_id, x, y);
        let pos = self.positions.get_mut(&entity_id).unwrap();
        pos.x = x;
        pos.y = y;
    }

    //get the size of an entity
    pub fn get_size(&self, entity_id: EntityID) -> (f64, f64) {
        let size = self.sizes.get(&entity_id).unwrap();
        (size.w, size.h)
    }

    pub fn set_size(&mut self, entity_id: EntityID, width: f64, height: f64) {
        let size = self.sizes.get_mut(&entity_id).unwrap();
        size.w = width;
        size.h = height;
    }

    /**
     * Architecture note:
     * the new_element methods should only create the necessary elements
     * without calculating the position and size.
     * That will be done in the layout layer.
     */

    // Wraps an element in a box
    pub fn new_box(&mut self, child: DiagramTreeNode, options: BoxOptions) -> DiagramTreeNode {
        let box_id = self.new_entity(EntityType::BoxShape);

        let sbox = ShapeBox::new(box_id, child.entity_id, options);
        self.boxes.insert(box_id, sbox);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::BoxShape,
            entity_id: box_id,
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
        let text_id = self.new_entity(EntityType::TextShape);
        //create the lines
        let text_lines = textwrap::wrap(&text, options.line_width);
        let lines: Vec<EntityID> = text_lines
            .iter()
            .map(|line| {
                let line_id = self.new_entity(EntityType::TextLine);
                let text_line = TextLine {
                    entity: line_id,
                    text: line.to_string(),
                };
                self.textlines.insert(line_id, text_line);
                line_id
            })
            .collect();

        let text = ShapeText::new(text_id, text, options, &lines);
        self.texts.insert(text_id, text);
        DiagramTreeNode::new(EntityType::TextShape, text_id)
    }

    pub fn new_line(&mut self, options: LineOptions) -> DiagramTreeNode {
        let line_id = self.new_entity(EntityType::LineShape);
        let line = ShapeLine::new(line_id, options);
        self.lines.insert(line_id, line);
        println!("Creating new line with id {}", line_id);
        DiagramTreeNode::new(EntityType::LineShape, line_id)
    }

    pub fn new_elipse(
        &mut self,
        center: (f64, f64),
        radius: (f64, f64),
        options: EllipseOptions,
    ) -> DiagramTreeNode {
        let ellipse_id = self.new_entity(EntityType::EllipseShape);
        let ellipse = ShapeEllipse::new(ellipse_id, center, radius, options);
        self.ellipses.insert(ellipse_id, ellipse);
        DiagramTreeNode::new(EntityType::EllipseShape, ellipse_id)
    }

    pub fn new_image(&mut self, image_data: &str, preferred_size:(f64, f64)) -> DiagramTreeNode {
        let image_id = self.new_entity(EntityType::ImageShape);
        let image = ShapeImage::new(image_id, image_data.to_string(), preferred_size);
        self.images.insert(image_id, image);
        DiagramTreeNode::new(EntityType::ImageShape, image_id)
    }
    
    pub fn new_image_from_file(&mut self, file_path: &str, preferred_size:(f64, f64)) -> DiagramTreeNode {
        let image_id = self.new_entity(EntityType::ImageShape);
        let image = ShapeImage::from_file(image_id, file_path.to_string(), preferred_size);
        self.images.insert(image_id, image);
        DiagramTreeNode::new(EntityType::ImageShape, image_id)
    }

    // Creates a new Group.
    pub fn new_group(&mut self, children: Vec<DiagramTreeNode>) -> DiagramTreeNode {
        let group_id = self.new_entity(EntityType::GroupShape);
        let mut sgroup = ShapeGroup {
            entity: group_id,
            elements: Vec::new(),
        };
        let mut node = DiagramTreeNode {
            entity_type: EntityType::GroupShape,
            entity_id: group_id,
            children: Vec::new(),
        };

        //set children
        for child in children {
            sgroup.elements.push(child.entity_id);
            node.add_child(child)
        }

        self.groups.insert(group_id, sgroup);

        node
    }

    pub fn new_table(
        &mut self,
        cells: Vec<DiagramTreeNode>,
        cols: usize,
        options: TableOptions,
    ) -> DiagramTreeNode {
        let mut cell_ids = Vec::new();
        for cell in &cells {
            cell_ids.push(cell.entity_id);
        }
        //create entities for the col and row lines
        let mut col_lines = Vec::new();
        for i in 0..cols {
            let line_id = self.new_entity(EntityType::LineShape);
            let line = ShapeLine::new(line_id, LineOptions::new());
            self.lines.insert(line_id, line);
            col_lines.push(line_id);
        }
        let num_rows = cells.len() / cols;
        let mut row_lines = Vec::new();
        for i in 0..num_rows + 1 {
            let line_id = self.new_entity(EntityType::LineShape);
            let line = ShapeLine::new(line_id, LineOptions::new());
            self.lines.insert(line_id, line);
            row_lines.push(line_id);
        }

        //Add a rectangle for the header row
        let header_id = self.new_entity(EntityType::BoxShape);
        let rect_options = RectOptions{
            fill_color:"red".to_string(),
            ..Default::default()
        };
        let header_rect = ShapeRect::new(header_id, rect_options);
        self.rectangles.insert(header_id, header_rect);


        let table_id = self.new_entity(EntityType::TableShape);
        let table = Table::new(
            table_id,
            cell_ids,
            col_lines.clone(),
            row_lines.clone(),
            cols,
            header_id,
            options.clone(),
        );

        self.tables.insert(table_id, table);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::TableShape,
            entity_id: table_id,
            children: Vec::new(),
        };
        for child in cells {
            node.add_child(child)
        }

        //add the lines
        for line in col_lines {
            node.add_child(DiagramTreeNode::new(EntityType::LineShape, line));
        }
        for line in row_lines {
            node.add_child(DiagramTreeNode::new(EntityType::LineShape, line));
        }
        node.add_child(DiagramTreeNode::new(EntityType::RectShape, header_id));

        node
    }

    pub fn new_polyline(&mut self, points: Vec<(f64, f64)>, options: LineOptions) -> DiagramTreeNode {
        let polyline_id = self.new_entity(EntityType::PolyLine);
        let polyline = PolyLine::new(polyline_id, points, options);
        self.polylines.insert(polyline_id, polyline);
        DiagramTreeNode::new(EntityType::PolyLine, polyline_id)
    }
}

// element list accessors
impl DiagramBuilder {
    pub fn get_text(&self, id: EntityID) -> &ShapeText {
        &self.texts[&id]
    }

    pub fn get_group(&self, id: EntityID) -> &ShapeGroup {
        &self.groups[&id]
    }

    pub fn get_horizontal_stack(&self, id: EntityID) -> &HorizontalStack {
        &self.horizontal_stacks[&id]
    }

    pub fn get_vertical_stack(&self, id: EntityID) -> &VerticalStack {
        &self.vertical_stacks[&id]
    }

    pub fn get_ellipse(&self, id: EntityID) -> &ShapeEllipse {
        &self.ellipses[&id]
    }

    pub fn get_line(&self, id: EntityID) -> &ShapeLine {
        &self.lines[&id]
    }

    pub fn get_text_line(&self, id: EntityID) -> &TextLine {
        &self.textlines[&id]
    }

    pub fn get_arrow(&self, id: EntityID) -> &ShapeArrow {
        &self.arrows[&id]
    }

    pub fn get_table(&self, id: EntityID) -> &Table {
        &self.tables[&id]
    }

    pub fn get_image(&self, id: EntityID) -> &ShapeImage {
        &self.images[&id]
    }

    pub fn get_box(&self, id: EntityID) -> &ShapeBox {
        &self.boxes[&id]
    }

    pub fn get_polyline(&self, id: EntityID) -> &PolyLine {
        &self.polylines[&id]
    }
}

//test
#[cfg(test)]
mod tests {
    use crate::get_entity_type_from_id;

    use super::*;

    #[test]
    fn test_session() {
        let mut session = DiagramBuilder::new();

        session.set_measure_text_fn(|text, text_options| {
            let textW: f64 = text.len() as f64 * text_options.font_size as f64;

            (textW, text_options.font_size.into())
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
}
