use crate::components::*;
use anyhow::{bail, Context, Result};
use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::{AddEditVariableError, Constraint, Solver, Variable, WeightedRelation::*};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum SimpleConstraint {
    // ===== BASIC ALIGNMENT CONSTRAINTS (Updated to support lists) =====
    /// Align the left edges of entities. First entity is the reference, others align to it.
    AlignLeft(Vec<EntityID>),
    /// Align the right edges of entities. First entity is the reference, others align to it.
    AlignRight(Vec<EntityID>),
    /// Align the top edges of entities. First entity is the reference, others align to it.
    AlignTop(Vec<EntityID>),
    /// Align the bottom edges of entities. First entity is the reference, others align to it.
    AlignBottom(Vec<EntityID>),
    /// Align the horizontal centers of entities. First entity is the reference, others align to it.
    AlignCenterHorizontal(Vec<EntityID>),
    /// Align the vertical centers of entities. First entity is the reference, others align to it.
    AlignCenterVertical(Vec<EntityID>),

    // ===== DIRECTIONAL POSITIONING CONSTRAINTS (Keep as pairs for now) =====
    /// First entity is to the right of the second entity
    RightOf(EntityID, EntityID),
    /// First entity is to the left of the second entity
    LeftOf(EntityID, EntityID),
    /// First entity is above the second entity
    Above(EntityID, EntityID),
    /// First entity is below the second entity
    Below(EntityID, EntityID),

    // ===== SPACING CONSTRAINTS =====
    /// Horizontal spacing between two entities
    HorizontalSpacing(EntityID, EntityID, Float),
    /// Vertical spacing between two entities
    VerticalSpacing(EntityID, EntityID, Float),
    /// Fixed distance between centers of two entities
    FixedDistance(EntityID, EntityID, Float),

    // ===== SIZE CONSTRAINTS =====
    /// All entities should have the same width. First is reference.
    SameWidth(Vec<EntityID>),
    /// All entities should have the same height. First is reference.
    SameHeight(Vec<EntityID>),
    /// All entities should have at least the same height (or more) than first.
    AtLeastSameHeight(Vec<EntityID>),
    MinHeight(EntityID, Float),
    /// All entities should have the same size. First is reference.
    SameSize(Vec<EntityID>),

    ProportionalWidth(EntityID, EntityID, Float),
    /// First entity's height is proportional to second entity's height by the given ratio
    ProportionalHeight(EntityID, EntityID, Float),
    /// Maintain a specific aspect ratio (width/height) for an entity
    AspectRatio(EntityID, Float),

    /// Fix an entity's width to a specific value
    FixedWidth(EntityID, Float),
    /// Fix an entity's height to a specific value
    FixedHeight(EntityID, Float),
    /// Fix an entity's size to specific values
    FixedSize(EntityID, Float, Float),
    /// Fix an entity's position to specific coordinates
    FixedPosition(EntityID, Float, Float),
    FixedX(EntityID, Float),

    // ===== STACK CONSTRAINTS =====
    /// Arrange entities in a horizontal line with optional spacing
    StackHorizontal(Vec<EntityID>, Option<Float>), // spacing between elements
    /// Arrange entities in a vertical line with optional spacing
    StackVertical(Vec<EntityID>, Option<Float>), // spacing between elements
}

pub struct ConstraintSystem {
    solver: Solver,
    variables: HashMap<String, EntityVars>,
}

struct EntityVars {
    x: Variable,
    y: Variable,
    width: Variable,
    height: Variable,
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            solver: Solver::new(),
            variables: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, id: String) -> anyhow::Result<()> {
        let vars = EntityVars {
            x: Variable::new(),
            y: Variable::new(),
            width: Variable::new(),
            height: Variable::new(),
        };

        // Ensure positive sizes
        self.solver
            .add_constraint(vars.width | GE(REQUIRED) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add width constraint: {:?}", e))?;
        self.solver
            .add_constraint(vars.height | GE(REQUIRED) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add height constraint: {:?}", e))?;

        // Mark editable variables
        self.solver
            .add_constraint(vars.width | GE(REQUIRED) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add width >= 0 constraint: {:?}", e))?;
        self.solver
            .add_constraint(vars.height | GE(REQUIRED) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add height >= 0 constraint: {:?}", e))?;
        self.solver
            .add_constraint(vars.x | EQ(WEAK) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add default x constraint: {:?}", e))?;
        self.solver
            .add_constraint(vars.y | EQ(WEAK) | 0.0)
            .map_err(|e| anyhow::anyhow!("Failed to add default y constraint: {:?}", e))?;

        self.variables.insert(id, vars);
        Ok(())
    }

    pub fn add_constraint(&mut self, constraint: SimpleConstraint) -> Result<()> {
        match constraint {
            SimpleConstraint::FixedX(entity, x) => {
                let vars = self.variables.get(&entity);
                if vars.is_none() {
                    bail!("entity not registered in constraint system {}", entity);
                }

                let vars = vars.unwrap();
                self.solver
                    .add_constraint(vars.x | EQ(REQUIRED) | x)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed position x constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::FixedPosition(entity, x, y) => {
                let vars = self.variables.get(&entity);
                if vars.is_none() {
                    bail!("entity not registered in constraint system {}", entity);
                }

                let vars = vars.unwrap();
                self.solver
                    .add_constraint(vars.x | EQ(REQUIRED) | x)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed position x constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint(vars.y | EQ(REQUIRED) | y)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed position y constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::AlignLeft(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("AlignLeft requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self.variables.get(reference);

                if ref_vars.is_none() {
                    bail!("entity not registered in constraint system {}", reference);
                }

                let ref_vars = ref_vars.unwrap();

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity);
                    if vars.is_none() {
                        bail!("entity not registered in constraint system {}", entity);
                    }
                    let vars = vars.unwrap();
                    self.solver
                        .add_constraint(vars.x | EQ(REQUIRED) | ref_vars.x)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add align left constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::AlignRight(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("AlignRight requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self.variables.get(reference).context(format!(
                    "entity not registered in constrain system {}",
                    reference
                ))?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(
                            (vars.x + vars.width) | EQ(REQUIRED) | (ref_vars.x + ref_vars.width),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add align right constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::AlignTop(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("AlignTop requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(vars.y | EQ(REQUIRED) | ref_vars.y)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add align top constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::AlignBottom(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("AlignBottom requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(
                            (vars.y + vars.height) | EQ(REQUIRED) | (ref_vars.y + ref_vars.height),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add align bottom constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::AlignCenterHorizontal(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!(
                        "AlignCenterHorizontal requires at least 2 entities"
                    ));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(
                            (vars.x + vars.width * 0.5)
                                | EQ(REQUIRED)
                                | (ref_vars.x + ref_vars.width * 0.5),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "Failed to add align center horizontal constraint: {:?}",
                                e
                            )
                        })?;
                }
            }

            SimpleConstraint::AlignCenterVertical(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!(
                        "AlignCenterVertical requires at least 2 entities"
                    ));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("Entity not found")?;
                    self.solver
                        .add_constraint(
                            (vars.y + vars.height * 0.5)
                                | EQ(REQUIRED)
                                | (ref_vars.y + ref_vars.height * 0.5),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "Failed to add align center vertical constraint: {:?}",
                                e
                            )
                        })?;
                }
            }

            SimpleConstraint::RightOf(id1, id2) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                self.solver
                    .add_constraint(vars1.x | GE(REQUIRED) | vars2.x + vars2.width)
                    .map_err(|e| anyhow::anyhow!("Failed to add rightOf constraint: {:?}", e))?;
            }

            SimpleConstraint::LeftOf(id1, id2) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                self.solver
                    .add_constraint(vars1.x | LE(REQUIRED) | vars2.x - vars1.width)
                    .map_err(|e| anyhow::anyhow!("Failed to add leftOf constraint: {:?}", e))?;
            }

            SimpleConstraint::Below(id1, id2) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                self.solver
                    .add_constraint(vars1.y | GE(REQUIRED) | vars2.y + vars2.height)
                    .map_err(|e| anyhow::anyhow!("Failed to add bottomOf constraint: {:?}", e))?;
            }
            SimpleConstraint::Above(id1, id2) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                // id1 está arriba de id2: id1.y + id1.height = id2.y
                self.solver
                    .add_constraint((vars1.y + vars1.height) | LE(REQUIRED) | vars2.y)
                    .map_err(|e| anyhow::anyhow!("Failed to add above constraint: {:?}", e))?;
            }
            SimpleConstraint::HorizontalSpacing(id1, id2, spacing) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                // Horizontal spacing: gap between right edge of first and left edge of second
                // x2 = x1 + width1 + spacing
                self.solver
                    .add_constraint(vars2.x | EQ(REQUIRED) | (vars1.x + vars1.width + spacing))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add horizontal spacing constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::VerticalSpacing(id1, id2, spacing) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                // Vertical spacing: gap between bottom edge of first and top edge of second
                // y2 = y1 + height1 + spacing
                self.solver
                    .add_constraint(vars2.y | EQ(REQUIRED) | (vars1.y + vars1.height + spacing))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add vertical spacing constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::FixedWidth(entity, width) => {
                let vars = self.variables.get(&entity).context("entity not found")?;
                self.solver
                    .add_constraint(vars.width | EQ(REQUIRED) | width)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed width constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::FixedHeight(entity, height) => {
                let vars = self.variables.get(&entity).context("entity not found")?;
                self.solver
                    .add_constraint(vars.height | EQ(REQUIRED) | height)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed height constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::FixedSize(entity, width, height) => {
                let vars = self.variables.get(&entity).context("entity not found")?;
                self.solver
                    .add_constraint(vars.width | EQ(REQUIRED) | width)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed size width constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint(vars.height | EQ(REQUIRED) | height)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add fixed size height constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::FixedDistance(id1, id2, distance) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;

                // Fixed distance between centers of two entities
                // Distance = sqrt((cx2 - cx1)² + (cy2 - cy1)²)
                // Since Cassowary is a linear solver, we can't directly implement sqrt.
                // We'll use the Manhattan distance as an approximation, or implement
                // a more sophisticated approach using auxiliary variables.

                // For now, let's implement using Manhattan distance (|dx| + |dy| = distance)
                // This is a reasonable approximation for many layout purposes.

                // Create auxiliary variables for the center positions
                let center1_x = Variable::new();
                let center1_y = Variable::new();
                let center2_x = Variable::new();
                let center2_y = Variable::new();

                // Define centers
                self.solver
                    .add_constraint(center1_x | EQ(REQUIRED) | (vars1.x + vars1.width * 0.5))
                    .map_err(|e| anyhow::anyhow!("Failed to add center1_x constraint: {:?}", e))?;
                self.solver
                    .add_constraint(center1_y | EQ(REQUIRED) | (vars1.y + vars1.height * 0.5))
                    .map_err(|e| anyhow::anyhow!("Failed to add center1_y constraint: {:?}", e))?;
                self.solver
                    .add_constraint(center2_x | EQ(REQUIRED) | (vars2.x + vars2.width * 0.5))
                    .map_err(|e| anyhow::anyhow!("Failed to add center2_x constraint: {:?}", e))?;
                self.solver
                    .add_constraint(center2_y | EQ(REQUIRED) | (vars2.y + vars2.height * 0.5))
                    .map_err(|e| anyhow::anyhow!("Failed to add center2_y constraint: {:?}", e))?;

                // For Manhattan distance approximation:
                // We'll enforce that the total horizontal and vertical distance equals the target distance
                // This works well for cases where entities are primarily aligned horizontally or vertically

                // Create auxiliary variables for absolute differences
                let dx_pos = Variable::new(); // max(center2_x - center1_x, 0)
                let dx_neg = Variable::new(); // max(center1_x - center2_x, 0)
                let dy_pos = Variable::new(); // max(center2_y - center1_y, 0)
                let dy_neg = Variable::new(); // max(center1_y - center2_y, 0)

                // Ensure non-negative auxiliary variables
                self.solver
                    .add_constraint(dx_pos | GE(REQUIRED) | 0.0)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dx_pos >= 0 constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint(dx_neg | GE(REQUIRED) | 0.0)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dx_neg >= 0 constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint(dy_pos | GE(REQUIRED) | 0.0)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dy_pos >= 0 constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint(dy_neg | GE(REQUIRED) | 0.0)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dy_neg >= 0 constraint: {:?}", e)
                    })?;

                // Define the absolute differences
                self.solver
                    .add_constraint((center2_x - center1_x) | EQ(REQUIRED) | (dx_pos - dx_neg))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dx difference constraint: {:?}", e)
                    })?;
                self.solver
                    .add_constraint((center2_y - center1_y) | EQ(REQUIRED) | (dy_pos - dy_neg))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add dy difference constraint: {:?}", e)
                    })?;

                // Manhattan distance constraint: |dx| + |dy| = distance
                self.solver
                    .add_constraint((dx_pos + dx_neg + dy_pos + dy_neg) | EQ(REQUIRED) | distance)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add manhattan distance constraint: {:?}", e)
                    })?;
            }
            SimpleConstraint::SameWidth(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("SameWidth requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(vars.width | EQ(REQUIRED) | ref_vars.width)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add same width constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::SameHeight(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("SameHeight requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(vars.height | EQ(REQUIRED) | ref_vars.height)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add same height constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::AtLeastSameHeight(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("SameHeight requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;

                    self.solver
                        .add_constraint(ref_vars.height | GE(REQUIRED) | vars.height)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add same height constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::MinHeight(id, h) => {
                let vars = self.variables.get(&id).context("entity not found")?;
                self.solver
                    .add_constraint(vars.height | GE(REQUIRED) | h)
                    .map_err(|e| anyhow::anyhow!("Failed to add min height constraint: {:?}", e))?;
            }

            SimpleConstraint::SameSize(entities) => {
                if entities.len() < 2 {
                    return Err(anyhow::anyhow!("SameSize requires at least 2 entities"));
                }
                let reference = &entities[0];
                let ref_vars = self
                    .variables
                    .get(reference)
                    .context("Reference entity not found")?;

                for entity in entities.iter().skip(1) {
                    let vars = self.variables.get(entity).context("entity not found")?;
                    self.solver
                        .add_constraint(vars.width | EQ(REQUIRED) | ref_vars.width)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add same size width constraint: {:?}", e)
                        })?;
                    self.solver
                        .add_constraint(vars.height | EQ(REQUIRED) | ref_vars.height)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add same size height constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::ProportionalWidth(id1, id2, ratio) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                // First entity's width = second entity's width * ratio
                // width1 = width2 * ratio
                self.solver
                    .add_constraint(vars1.width | EQ(REQUIRED) | (vars2.width * ratio))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add proportional width constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::ProportionalHeight(id1, id2, ratio) => {
                let vars1 = self.variables.get(&id1).context("entity not found")?;
                let vars2 = self.variables.get(&id2).context("entity not found")?;
                // First entity's height = second entity's height * ratio
                // height1 = height2 * ratio
                self.solver
                    .add_constraint(vars1.height | EQ(REQUIRED) | (vars2.height * ratio))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add proportional height constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::AspectRatio(id, ratio) => {
                let vars = self.variables.get(&id).context("entity not found")?;
                // Maintain aspect ratio: width / height = ratio
                // Rearranged: width = height * ratio
                self.solver
                    .add_constraint(vars.width | EQ(REQUIRED) | (vars.height * ratio))
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to add aspect ratio constraint: {:?}", e)
                    })?;
            }

            SimpleConstraint::StackHorizontal(entities, spacing) => {
                let spacing = spacing.unwrap_or(0.0);
                if entities.is_empty() {
                    return Err(anyhow::anyhow!(
                        "StackHorizontal requires at least 1 entity"
                    ));
                }

                // Stack entities horizontally with fixed spacing between them
                // entity[i+1].x = entity[i].x + entity[i].width + spacing
                for i in 1..entities.len() {
                    let prev_vars = self
                        .variables
                        .get(&entities[i - 1])
                        .context("Previous entity not found")?;
                    let curr_vars = self
                        .variables
                        .get(&entities[i])
                        .context("Current entity not found")?;

                    // Current entity starts where previous entity ends plus spacing
                    self.solver
                        .add_constraint(
                            curr_vars.x | EQ(REQUIRED) | (prev_vars.x + prev_vars.width + spacing),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add horizontal stack constraint: {:?}", e)
                        })?;
                }
            }

            SimpleConstraint::StackVertical(entities, spacing) => {
                let spacing = spacing.unwrap_or(0.0);
                if entities.is_empty() {
                    return Err(anyhow::anyhow!("StackVertical requires at least 1 entity"));
                }

                // Stack entities vertically with fixed spacing between them
                // entity[i+1].y = entity[i].y + entity[i].height + spacing
                for i in 1..entities.len() {
                    let prev_vars = self
                        .variables
                        .get(&entities[i - 1])
                        .context("Previous entity not found")?;
                    let curr_vars = self
                        .variables
                        .get(&entities[i])
                        .context("Current entity not found")?;

                    // Current entity starts where previous entity ends plus spacing
                    self.solver
                        .add_constraint(
                            curr_vars.y | EQ(REQUIRED) | (prev_vars.y + prev_vars.height + spacing),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to add vertical stack constraint: {:?}", e)
                        })?;
                }
            }
        }
        Ok(())
    }

    pub fn solve(&mut self) -> Result<HashMap<String, (f32, f32, f32, f32)>> {
        self.solver.fetch_changes();
        let mut results = HashMap::new();
        for (id, vars) in &self.variables {
            let x = self.solver.get_value(vars.x) as f32;
            let y = self.solver.get_value(vars.y) as f32;
            let width = self.solver.get_value(vars.width) as f32;
            let height = self.solver.get_value(vars.height) as f32;
            results.insert(id.clone(), (x, y, width, height));
        }
        Ok(results)
    }

    pub fn suggest_size(
        &mut self,
        id: &str,
        width: f32,
        height: f32,
        is_fixed_size: bool,
    ) -> Result<(), cassowary::SuggestValueError> {
        println!("Suggest_size called {} {} {}", id, width, height);

        // Solo permitir al solver cambiar width o height cuando tiene sentido (ej un rectangulo)
        // Un vstack no debe tener su width o height modificado
        let c_strength = if is_fixed_size { REQUIRED} else { STRONG };
        if let Some(vars) = self.variables.get(id) {
            let _ = self
                .solver
                .add_constraint(vars.width | EQ(c_strength) | (width as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested width constraint: {:?}", e));
            let _ = self
                .solver
                .add_constraint(vars.height | EQ(c_strength) | (height as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested height constraint: {:?}", e));
        }

        Ok(())
    }

    pub fn suggest_position(
        &mut self,
        id: &str,
        x: f32,
        y: f32,
    ) -> Result<(), cassowary::SuggestValueError> {
        if let Some(vars) = self.variables.get(id) {
            let _ = self
                .solver
                .add_constraint(vars.x | EQ(MEDIUM) | (x as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested x constraint: {:?}", e));
            let _ = self
                .solver
                .add_constraint(vars.y | EQ(MEDIUM) | (y as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested y constraint: {:?}", e));
        }
        Ok(())
    }
}
