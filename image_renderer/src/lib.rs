use std::io::Write;
use image::{ImageBuffer, Rgba};


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
        create_dummy_png(stream).map_err(|e| RendererError::new(&e.to_string()));
        Ok(())
    }
}


fn create_dummy_png<W: Write>(stream: &mut W) -> Result<(), image::ImageError> {
    // Create a dummy image (e.g., 100x100 pixels, red background)
    let width = 100;
    let height = 100;
    let red = Rgba([255, 0, 0, 255]);
    let mut imgbuf = ImageBuffer::from_fn(width, height, |_x, _y| red);

    // Write the PNG image to the stream
    let encoder = image::png::PngEncoder::new(stream);
    encoder.encode(
        imgbuf.as_ref(),
        imgbuf.width(),
        imgbuf.height(),
        image::ColorType::Rgba8,
    )?;

    Ok(())
}


