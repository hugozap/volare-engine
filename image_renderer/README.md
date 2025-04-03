# Volare Image Renderer

This is a PNG renderer implementation for the Volare Visual Engine.

## Supported Shapes

Current implementation supports the following shapes:
- Rectangle (Box)
- Text
- Groups
- Vertical and Horizontal Stacks

## Usage

The renderer implements the `Renderer` trait from `volare_engine_layout`. You can use it the same way as the SVG renderer:

```rust
use image_renderer::PNGRenderer;
use volare_engine_layout::renderer_base::Renderer;

// Initialize diagram
let mut session = DiagramBuilder::new();
// ... build your diagram ...

// Render to PNG
let png_renderer = PNGRenderer {};
let mut file = File::create("diagram.png").unwrap();
png_renderer.render(&session, &diagram_node, &mut file).unwrap();
```

## Extending the Renderer

To support more shapes, implement additional rendering functions for the desired shape types and add them to the `match` statement in the `render_node` function.