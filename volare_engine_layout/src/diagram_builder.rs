use std::{collections::HashMap, sync::Arc};

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
    pub measure_text: Option<fn(&str, &TextOptions) -> (Float, Float)>,
    pub entities: Vec<EntityID>,
    pub positions: HashMap<EntityID, Point>,
    pub sizes: HashMap<EntityID, Size>,
    pub entityTypes: HashMap<EntityID, EntityType>,

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
    free_containers: HashMap<EntityID, FreeContainer>,
    pub custom_components: CustomComponentRegistry,
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
    pub fn new(entity_type: EntityType, id: EntityID) -> DiagramTreeNode {
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
            entityTypes: HashMap::<EntityID, EntityType>::new(),
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
            free_containers: HashMap::new(),
            custom_components: CustomComponentRegistry::new(),
        }
    }

    /// Register a custom component with the builder
    pub fn register_custom_component<F>(&mut self, component_type: &str, factory: F)
    where
        F: Fn(
                &serde_json::Map<String, serde_json::Value>,
                &mut DiagramBuilder,
            ) -> Result<crate::diagram_builder::DiagramTreeNode, String>
            + Send
            + Sync
            + 'static,
    {
        self.custom_components.register(component_type, factory);
    }

    /// Check if a custom component is registered
    pub fn has_custom_component(&self, component_type: &str) -> bool {
        self.custom_components.has_component(component_type)
    }

    pub fn get_custom_component_types(&self) -> Vec<&String> {
        self.custom_components.get_registered_types()
    }

    pub fn create_custom_component(
        &mut self,
        component_type: &str,
        options: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<DiagramTreeNode, String> {
        if !self.custom_components.has_component(component_type) {
            return Err(format!(
                "Custom component '{}' not registered",
                component_type
            ));
        }

        let factory = { self.custom_components.get(component_type).unwrap().clone() };

        factory(options, self)
    }

    /* Create a new entity of a given type
     * Returns the id of the new entity
     * We have another array with the positions of the entities
     * in the same index. So they are fast to access
     */
    pub fn new_entity(&mut self,id : EntityID, entity_type: EntityType) -> EntityID {
        println!("Creating new entity with id {}", id);
        self.entities.push(id.clone());
        self.positions.insert(id.clone(), Point::new(0.0, 0.0));
        self.sizes.insert(id.clone(), Size::new(0.0, 0.0));
        self.entityTypes.insert(id.clone(), entity_type.clone());
        id
    }

    pub fn clear_cache(&mut self) {
        //clear entities vector
        self.entities = Vec::new();
        self.positions = HashMap::new();
        self.sizes = HashMap::new();
    }

    //set the measure_text function
    pub fn set_measure_text_fn(&mut self, measure_text: fn(&str, &TextOptions) -> (Float, Float)) {
        println!("Setting measure text function");
        self.measure_text = Option::Some(measure_text);
    }

    //get the position of an entity
    pub fn get_position(&self, entity_id: EntityID) -> (Float, Float) {
        let pos = self.positions.get(&entity_id).unwrap();
        (pos.x, pos.y)
    }

    pub fn set_position(&mut self, entity_id: EntityID, x: Float, y: Float) {
        println!("Setting position of entity {} to ({}, {})", entity_id, x, y);
        let pos = self.positions.get_mut(&entity_id).unwrap();
        pos.x = x;
        pos.y = y;
    }

    //get the size of an entity
    pub fn get_size(&self, entity_id: EntityID) -> (Float, Float) {
        let size = self.sizes.get(&entity_id).unwrap();
        (size.w, size.h)
    }

    pub fn set_size(&mut self, entity_id: EntityID, width: Float, height: Float) {
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
    pub fn new_box(&mut self,id: EntityID, child: DiagramTreeNode, options: BoxOptions) -> DiagramTreeNode {
        let box_id = self.new_entity(id.clone(), EntityType::BoxShape);

        let sbox = ShapeBox::new(box_id.clone(), child.entity_id.clone(), options);
        self.boxes.insert(box_id.clone(), sbox);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::BoxShape,
            entity_id: box_id.clone(),
            children: Vec::new(),
        };
        node.children.push(Box::new(child.clone()));
        node
    }

    // Creates a new Vertical stack.
    pub fn new_vstack(
        &mut self,
        id: EntityID,
        children: Vec<DiagramTreeNode>,
        horizontal_alignment: HorizontalAlignment,
    ) -> DiagramTreeNode {
        let stack_id = self.new_entity(id.clone(), EntityType::VerticalStackShape);
        let mut vstack = VerticalStack {
            entity: stack_id.clone(),
            elements: Vec::new(),
            horizontal_alignment,
        };
        let mut node = DiagramTreeNode {
            entity_type: EntityType::VerticalStackShape,
            entity_id: stack_id.clone(),
            children: Vec::new(),
        };

        //set children
        for child in children {
            vstack.elements.push(child.entity_id.clone());
            node.add_child(child)
        }

        self.vertical_stacks.insert(stack_id.clone(), vstack);

        node
    }

    // Creates a new Vertical stack.
    pub fn new_hstack(
        &mut self,
        id : EntityID,
        children: Vec<DiagramTreeNode>,
        vertical_alignment: VerticalAlignment,
    ) -> DiagramTreeNode {
        let stack_id = self.new_entity(id.clone(), EntityType::HorizontalStackShape);
        let mut hstack = HorizontalStack {
            entity: stack_id.clone(),
            elements: Vec::new(),
            vertical_alignment,
        };
        let mut node = DiagramTreeNode {
            entity_type: EntityType::HorizontalStackShape,
            entity_id: stack_id.clone(),
            children: Vec::new(),
        };

        //set children
        for child in children {
            hstack.elements.push(child.entity_id.clone());
            node.add_child(child)
        }

        self.horizontal_stacks.insert(stack_id, hstack);

        node
    }

    pub fn new_rectangle(&mut self, id: EntityID, options: RectOptions) -> DiagramTreeNode {
        let rect_id = self.new_entity(id.clone(),EntityType::RectShape);
        let rect = ShapeRect::new(rect_id.clone(), options);
        self.rectangles.insert(rect_id.clone(), rect);
        DiagramTreeNode::new(EntityType::RectShape, rect_id.clone())
    }

    // Creates a new Text element
    // text: the text to display
    // options: the options for the text
    // ```rust
    // let text = session.new_text("Hello World", TextOptions::new());
    // ```
    pub fn new_text(&mut self,id : EntityID, text: &str, options: TextOptions) -> DiagramTreeNode {
        let text_id = self.new_entity(id, EntityType::TextShape);
        //create the lines
        let text_lines = textwrap::wrap(&text, options.line_width);
        let lines: Vec<EntityID> = text_lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let line_id = format!("text-{}-line-{}", text_id.clone(), i); // Generate a new ID for each line using the index
                self.new_entity(line_id.clone(), EntityType::TextLine);
                let text_line = TextLine {
                    entity: line_id.clone(),
                    text: line.to_string(),
                };
                self.textlines.insert(line_id.clone(), text_line.clone());
                line_id
            })
            .collect();

        let text = ShapeText::new(text_id.clone(), text, options, &lines);
        self.texts.insert(text_id.clone(), text);
        DiagramTreeNode::new(EntityType::TextShape, text_id)
    }

    pub fn new_line(
        &mut self,
        id : EntityID,
        start: (Float, Float),
        end: (Float, Float),
        options: LineOptions,
    ) -> DiagramTreeNode {
        let line_id = self.new_entity(id, EntityType::LineShape);
        let line = ShapeLine::new(line_id.clone(), start, end, options);
        self.lines.insert(line_id.clone(), line);
        println!("Creating new line with id {}", line_id.clone());
        DiagramTreeNode::new(EntityType::LineShape, line_id)
    }

    pub fn new_elipse(
        &mut self,
        id: EntityID,
        center: (Float, Float),
        radius: (Float, Float),
        options: EllipseOptions,
    ) -> DiagramTreeNode {
        let ellipse_id = self.new_entity(id, EntityType::EllipseShape);
        let ellipse = ShapeEllipse::new(ellipse_id.clone(), center, radius, options);
        self.ellipses.insert(ellipse_id.clone(), ellipse);
        DiagramTreeNode::new(EntityType::EllipseShape, ellipse_id.clone())
    }

    pub fn new_image(
        &mut self,
        id: EntityID,
        image_data: &str,
        size: (SizeBehavior, SizeBehavior),
    ) -> DiagramTreeNode {
        let image_id = self.new_entity(id, EntityType::ImageShape);
        let image = ShapeImage::new(image_id.clone(), image_data.to_string(), size);
        self.images.insert(image_id.clone(), image);
        DiagramTreeNode::new(EntityType::ImageShape, image_id.clone())
    }

    pub fn new_image_from_file(
        &mut self,
        id: EntityID,
        file_path: &str,
        size: (SizeBehavior, SizeBehavior),
    ) -> DiagramTreeNode {
        let image_id = self.new_entity(id, EntityType::ImageShape);
        let image = ShapeImage::from_file(image_id.clone(), file_path.to_string(), size);
        self.images.insert(image_id.clone(), image.clone());
        DiagramTreeNode::new(EntityType::ImageShape, image_id.clone())
    }

    // Creates a new Group.
    pub fn new_group(&mut self,id : EntityID, children: Vec<DiagramTreeNode>) -> DiagramTreeNode {
        let group_id = self.new_entity(id, EntityType::GroupShape);
        let mut sgroup = ShapeGroup {
            entity: group_id.clone(),
            elements: Vec::new(),
        };
        let mut node = DiagramTreeNode {
            entity_type: EntityType::GroupShape,
            entity_id: group_id.clone(),
            children: Vec::new(),
        };

        //set children
        for child in children {
            sgroup.elements.push(child.entity_id.clone());
            node.add_child(child.clone())
        }

        self.groups.insert(group_id, sgroup);

        node
    }

    pub fn new_table(
        &mut self,
        id: EntityID,
        cells: Vec<DiagramTreeNode>,
        cols: usize,
        options: TableOptions,
    ) -> DiagramTreeNode {
        let mut cell_ids = Vec::new();
        for cell in &cells {
            cell_ids.push(cell.entity_id.clone());
        }
        //create entities for the col and row lines
        let mut col_lines = Vec::new();
        for i in 0..cols {
            let line_id = format!("{}-col-line-{}", id.clone(), i);
            self.new_entity(line_id.clone(), EntityType::LineShape);
            let line = ShapeLine::new(line_id.clone(), (0.0, 0.0), (0.0, 0.0), LineOptions::new());
            self.lines.insert(line_id.clone(), line);
            col_lines.push(line_id.clone());
        }
        let num_rows = cells.len() / cols;
        let mut row_lines = Vec::new();
        for i in 0..num_rows + 1 {
            let line_id = format!("{}-row-line-{}", id.clone(), i);
            self.new_entity(line_id.clone(), EntityType::LineShape);
            let line = ShapeLine::new(line_id.clone(), (0.0, 0.0), (0.0, 0.0), LineOptions::new());
            self.lines.insert(line_id.clone(), line);
            row_lines.push(line_id.clone());
        }

        //Add a rectangle for the header row
        let header_id = format!("{}-header", id);
        self.new_entity(header_id.clone(), EntityType::RectShape);
        // Create the rectangle for the header row
        let header = self.new_rectangle(header_id, RectOptions {
            fill_color: Fill::Color(options.header_fill_color.clone()),
            stroke_color: String::from("black"),
            stroke_width: 1.0,
            ..Default::default()
        });

        let table_id = format!("{}-table", id.clone());
        self.new_entity(table_id.clone(), EntityType::TableShape);
        let table = Table::new(
            table_id.clone(),
            cell_ids,
            col_lines.clone(),
            row_lines.clone(),
            cols,
            header.entity_id.clone(),
            options.clone(),
        );

        self.tables.insert(table_id.clone(), table);
        let mut node = DiagramTreeNode {
            entity_type: EntityType::TableShape,
            entity_id: table_id.clone(),
            children: Vec::new(),
        };

        // Add the header before the cells, otherwise it can cover the cells
        node.add_child(DiagramTreeNode::new(
            EntityType::RectShape,
            header.entity_id.clone(),
        ));

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

        node
    }

    pub fn new_polyline(
        &mut self,
        id: EntityID,
        points: Vec<(Float, Float)>,
        options: LineOptions,
    ) -> DiagramTreeNode {
        self.new_entity(id.clone(), EntityType::PolyLine);
        let polyline = PolyLine::new(id.clone(), points, options);
        self.polylines.insert(id.clone(), polyline);
        DiagramTreeNode::new(EntityType::PolyLine, id.clone())
    }

    /// Creates a new FreeContainer with all children at once
    pub fn new_free_container(
        &mut self,
        id: EntityID,
        children_with_positions: Vec<(DiagramTreeNode, (Float, Float))>,
    ) -> DiagramTreeNode {
        let container_id = self.new_entity(id.clone(), EntityType::FreeContainer);  

        // Create the free container
        let mut container = FreeContainer::new(container_id.clone());

        // Create the node for the tree
        let mut node = DiagramTreeNode {
            entity_type: EntityType::FreeContainer,
            entity_id: container_id.clone(),
            children: Vec::new(),
        };

        // Add all children with their positions
        for (child, position) in children_with_positions {
            container.add_child(child.entity_id.clone(), position);
            node.add_child(child);
        }

        // Store the container
        self.free_containers.insert(container_id.clone(), container);

        node
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

    pub fn get_rectangle(&self, id: EntityID) -> &ShapeRect {
        &self.rectangles[&id]
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

    pub fn get_free_container(&self, id: EntityID) -> &FreeContainer {
        &self.free_containers[&id]
    }

    pub fn get_free_container_mut(&mut self, id: EntityID) -> &mut FreeContainer {
        self.free_containers.get_mut(&id).unwrap()
    }

    pub fn get_custom_component(
        &self,
        component_type: &str,
    ) -> Option<
        &Arc<
            dyn Fn(
                    &serde_json::Map<String, serde_json::Value>,
                    &mut DiagramBuilder,
                ) -> Result<DiagramTreeNode, String>
                + Send
                + Sync,
        >,
    > {
        self.custom_components.get(component_type)
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
            let textW: Float = text.len() as Float * text_options.font_size as Float;

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

//Component registration tests
#[cfg(test)]
mod component_registration_tests {
    use serde_json::{json, Map, Value};

    use super::*;

    /// Custom Component 1: Badge
    /// Creates a rounded pill-shaped element with text
    fn create_badge_component(
        attrs: &Map<String, Value>,
        builder: &mut DiagramBuilder,
    ) -> Result<DiagramTreeNode, String> {
        println!("🏷️  Creating badge component with attrs: {:?}", attrs);

        // Extract attributes
        let text = get_string_attr(attrs, "text", "Badge");
        let background = get_string_attr(attrs, "background", "blue");
        let color = get_string_attr(attrs, "color", "white");
        let font_size = get_float_attr(attrs, "font_size", 12.0);
        let padding = get_float_attr(attrs, "padding", 8.0);

        // Create text element
        let text_options = TextOptions {
            font_family: "Arial".to_string(),
            font_size,
            text_color: color,
            line_width: 200,
            line_spacing: 0.0,
        };
        let text_node = builder.new_text("text".to_string(), &text, text_options);

        // Wrap in rounded box
        let box_options = BoxOptions {
            fill_color: Fill::Color(background),
            stroke_color: "transparent".to_string(),
            stroke_width: 0.0,
            padding,
            border_radius: font_size,               // Make it pill-shaped
            width_behavior: SizeBehavior::Content,  // Auto width based on text
            height_behavior: SizeBehavior::Content, // Auto height based on text
        };
        let badge = builder.new_box("container".to_string(), text_node, box_options);

        println!("✅ Badge '{}' created successfully", text);
        Ok(badge)
    }

    // Helper function to extract attributes (since we can't access CustomComponentRegistry helpers directly)
    fn get_string_attr(attrs: &Map<String, Value>, key: &str, default: &str) -> String {
        attrs
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    }

    fn get_float_attr(attrs: &Map<String, Value>, key: &str, default: f64) -> Float {
        attrs.get(key).and_then(|v| v.as_f64()).unwrap_or(default) as Float
    }

    fn get_bool_attr(attrs: &Map<String, Value>, key: &str, default: bool) -> bool {
        attrs.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    fn get_int_attr(attrs: &Map<String, Value>, key: &str, default: i64) -> i64 {
        attrs.get(key).and_then(|v| v.as_i64()).unwrap_or(default)
    }

    #[test]
    fn test_badge_component() {
        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as Float * 8.0, 16.0));
        builder.register_custom_component("badge", create_badge_component);

        let attrs = json!({
            "text": "Test",
            "background": "blue"
        })
        .as_object()
        .unwrap()
        .clone();

        let result = builder.create_custom_component("badge", &attrs);
        assert!(result.is_ok());
        let badge_node = result.unwrap();
        assert_eq!(badge_node.entity_type, EntityType::BoxShape);
        assert!(builder.has_custom_component("badge"));
        let badge = builder.get_box(badge_node.entity_id);
        assert_eq!(badge.box_options.fill_color, Fill::Color("blue".to_string()));
    }

    #[test]
    fn test_all_components_registration() {
        let mut builder = DiagramBuilder::new();
        builder.register_custom_component("badge", create_badge_component);

        let types = builder.get_custom_component_types();
        assert_eq!(types.len(), 1);
        assert!(builder.has_custom_component("badge"));
    }
}
