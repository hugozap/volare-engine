
//use EntityID
use crate::components::*;

/* A table contains a list of rows, each row has a cell 
* which is a group that contains other elements.

Tables are defined with an array of cells and the number of columns
*/
pub struct Table {
    pub entity: EntityID,
    pub cols: usize, 
    pub cells: Vec<EntityID>,
    pub lines: Vec<EntityID>,
    pub table_options: TableOptions,
}

//new


impl Clone for Table {
    fn clone(&self) -> Self {
        Table {
            entity: self.entity,
            cols: self.cols,
            cells: self.cells.clone(),
            lines: self.lines.clone(),
            table_options: self.table_options.clone(),
        }
    }
}

//constructor that receives only the table options
impl Table {
    pub fn new(entity: EntityID,cells: Vec<EntityID>,  cols: usize, table_options: TableOptions) -> Table {
        Table {
            entity,
            cols,
            cells,
            lines: Vec::new(),
            table_options,
        }
    }
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


pub struct TableOptions {
    pub fill_color: String,
    pub border_color: String,
    pub border_width: usize,
    pub cell_padding: usize,
}

//defaults
impl Default for TableOptions {
    fn default() -> Self {
        TableOptions {
            fill_color: String::from("white"),
            border_color: String::from("black"),
            border_width: 1,
            cell_padding: 20,
        }
    }
}

impl Clone for TableOptions {
    fn clone(&self) -> Self {
        TableOptions {
            fill_color: self.fill_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width,
            cell_padding: self.cell_padding,
        }
    }
}
