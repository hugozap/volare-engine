use std::{error::Error, fmt, io::Write};

use crate::{DiagramBuilder, diagram_builder::DiagramTreeNode};

#[derive(Debug)]
pub struct RendererError {
    message: String,
}

impl RendererError {
    pub fn new(message: &str) -> RendererError {
        RendererError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RendererError {}

pub trait Renderer<W: Write> {
    fn render(
        &self,
        session: &DiagramBuilder,
        diagram_node: &DiagramTreeNode,
        stream: &mut W,
    ) -> Result<(), RendererError>;
}
