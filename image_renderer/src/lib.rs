use std::io::Write;

use volare_engine_layout::{diagram_builder::DiagramTreeNode, Renderer, DiagramBuilder, RendererError};

/**
 * This is the PNG renderer. It will render the diagram to a PNG stream.
 */

pub struct PNGRenderer;

impl<W: Write> Renderer<W> for PNGRenderer {
    fn render(
        &self,
        session: &DiagramBuilder,
        diagram_node: &DiagramTreeNode,
        stream: &mut W,
    ) -> Result<(), RendererError> {
        // Implement the PNG rendering logic here
        Ok(())
    }
}

