use crate::components::*;
use anyhow::Result;
use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::{AddEditVariableError, Constraint, Solver, Variable, WeightedRelation::*};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SimpleConstraint {
    AlignLeft(String, String),
    RightOf(String, String),
    BottomOf(String, String),
    TopOf(String, String),
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
            SimpleConstraint::AlignLeft(id1, id2) => {
                let vars1 = self.variables.get(&id1).expect("Entity not found");
                let vars2 = self.variables.get(&id2).expect("Entity not found");
                self.solver
                    .add_constraint(vars1.x | EQ(REQUIRED) | vars2.x)
                    .map_err(|e| anyhow::anyhow!("Failed to add align left constraint: {:?}", e))?;
            }
            SimpleConstraint::RightOf(id1, id2 ) => {
                let vars1 = self.variables.get(&id1).expect("Entity not found");
                let vars2 = self.variables.get(&id2).expect("Entity not found");
                self.solver
                    .add_constraint(vars1.x | EQ(REQUIRED) |  vars2.x + vars2.width)
                    .map_err(|e| anyhow::anyhow!("Failed to add rightOf constraint: {:?}", e))?;
            }
            SimpleConstraint::BottomOf(id1, id2 ) => {
                let vars1 = self.variables.get(&id1).expect("Entity not found");
                let vars2 = self.variables.get(&id2).expect("Entity not found");
                self.solver
                    .add_constraint(vars1.y | EQ(REQUIRED) |  vars2.y + vars2.height)
                    .map_err(|e| anyhow::anyhow!("Failed to add bottomOf constraint: {:?}", e))?;
            }
            SimpleConstraint::TopOf(id1, id2 ) => {
                let vars1 = self.variables.get(&id1).expect("Entity not found");
                let vars2 = self.variables.get(&id2).expect("Entity not found");
                self.solver
                    .add_constraint(vars1.y | EQ(REQUIRED) |  vars2.y - vars2.height)
                    .map_err(|e| anyhow::anyhow!("Failed to add bottomOf constraint: {:?}", e))?;
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
    ) -> Result<(), cassowary::SuggestValueError> {
        println!("Suggest_size called {} {} {}", id, width, height);
        if let Some(vars) = self.variables.get(id) {
             let _ = self.solver
                .add_constraint(vars.width | EQ(STRONG) | (width as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested width constraint: {:?}", e));
            let _ = self.solver
                .add_constraint(vars.height | EQ(STRONG) | (height as f64))
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
             let _ = self.solver
                .add_constraint(vars.x | EQ(MEDIUM) | (x as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested x constraint: {:?}", e));
            let _ = self.solver
                .add_constraint(vars.y | EQ(MEDIUM) | (y as f64))
                .map_err(|e| anyhow::anyhow!("Failed to add suggested y constraint: {:?}", e));
        }
        Ok(())
    }
}
