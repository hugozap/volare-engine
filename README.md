# Volare Engine

> A token-efficient visual specification language for LLM-generated diagrams and documents

**Status**: Alpha - Core engine functional, tooling and specification in development.

## Why Volare?

Traditional visual formats (SVG, HTML, Canvas) are verbose and waste LLM tokens. Volare uses a compact JSONL specification designed for AI generation:

**Before** (100+ lines of SVG):
```xml
<svg width="400" height="300">
  <rect x="10" y="10" width="380" height="280" fill="#f0f0f0"/>
  <text x="200" y="30" font-size="24">Title</text>
  ...
</svg>
```

**After** (4 lines of JSONL):
```jsonl
{"id":"root","type":"vstack","children":["title","box"]}
{"id":"title","type":"text","content":"Title","font_size":24}
{"id":"box","type":"rect","width":380,"height":280,"background":"#f0f0f0"}
```

## Quick Start

```rust
use volare_engine_layout::*;
use svg_renderer::SVGRenderer;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jsonl = r#"
{"id":"root","type":"vstack","children":["hello"]}
{"id":"hello","type":"text","content":"Hello World"}
"#;

    let mut parser = JsonLinesParser::new();
    let root_id = parser.parse_string(jsonl)?;

    let mut builder = DiagramBuilder::new();
    let tree = parser.build(&root_id, &mut builder)?;

    // Render to SVG
    let mut output = File::create("output.svg")?;
    SVGRenderer.render(&builder, &tree, &mut output)?;
    
    Ok(())
}
```

## Core Concepts

### JSONL Format

- **One JSON object per line** - No arrays at the root level, strict JSONL
- **First line is root** - Single root element per diagram
- **Flat attributes** - No nested objects in attribute values
- **ID references** - Elements reference each other via string IDs

### Component Types

**Layout Containers**
- `vstack` - Stack children vertically
- `hstack` - Stack children horizontally
- `free_container` - Absolute positioning
- `constraint_container` - Constraint-based layouts
- `group` - Logical grouping

**Shapes**
- `rect` - Rectangles with optional border radius
- `ellipse` - Circles and ellipses
- `arc` - Circular arcs and pie slices
- `line` - Straight lines
- `polyline` - Connected line segments
- `box` - Wrapper with padding, borders, background

**Content**
- `text` - Text with automatic wrapping
- `image` - Images (base64 or file path)
- `table` - Tables with headers and cells

**Document Components**
- `document.text` - Styled paragraphs
- `document.section` - Titled sections
- `document.properties` - Key-value displays

**Specialized Diagrams** (Custom Components)
- `ishikawa` - Fishbone/cause-effect diagrams
- `calendar` - Calendar layouts
- *(more planned)*

### Layout System

Volare uses constraint-based layouts powered by the Cassowary solver:

```jsonl
{"id":"root","type":"constraint_container","children":["a","b"],"constraints":[{"type":"align_top","elements":["a","b"]},{"type":"horizontal_spacing","from":"a","to":"b","spacing":20}]}
{"id":"a","type":"rect","width":100,"height":50,"background":"blue"}
{"id":"b","type":"rect","width":100,"height":50,"background":"red"}
```

## Examples

### Simple Card Layout

```jsonl
{"id":"root","type":"box","children":["stack"],"padding":20,"background":"#ffffff","border_color":"#dddddd","border_width":1}
{"id":"stack","type":"vstack","children":["title","body"],"spacing":10}
{"id":"title","type":"text","content":"Card Title","font_size":18,"font_weight":700}
{"id":"body","type":"text","content":"This is the card body text.","font_size":14}
```

### Horizontal Layout

```jsonl
{"id":"root","type":"hstack","children":["left","right"],"spacing":20}
{"id":"left","type":"rect","width":100,"height":100,"background":"#3b82f6"}
{"id":"right","type":"rect","width":100,"height":100,"background":"#10b981"}
```

### Table

```jsonl
{"id":"root","type":"table","headers":["Name","Age","City"],"rows":[["Alice","30","NYC"],["Bob","25","SF"]]}
```

### Ishikawa Diagram

```jsonl
{"id":"root","type":"ishikawa","effect":"Low Sales","categories":[{"name":"People","items":["Lack of training","Poor motivation"]},{"name":"Process","items":["Complex checkout","Slow shipping"]}]}
```

## Rendering

Volare supports multiple output formats:

**SVG** - Vector graphics for web and print
```rust
use svg_renderer::SVGRenderer;
use std::fs::File;

let mut output = File::create("diagram.svg")?;
SVGRenderer.render(&builder, &tree, &mut output)?;
```

**PNG** - Raster images with configurable scaling
```rust
use image_renderer::PNGRenderer;
use std::fs::File;

let mut output = File::create("diagram.png")?;
PNGRenderer.render(&builder, &tree, &mut output)?;
```

## Attribute Reference

### Common Attributes

**Layout**
- `width`, `height` - Fixed dimensions
- `padding` - Internal spacing
- `spacing` - Gap between children (stacks)
- `alignment` - Horizontal alignment: `left`, `center`, `right`

**Styling**
- `background` - Background color (hex or name)
- `border_color`, `border_width` - Border styling
- `border_radius` - Rounded corners
- `stroke_color`, `stroke_width` - Shape outlines
- `fill_color` - Shape fills

**Typography**
- `content` - Text content
- `font_size` - Font size in pixels
- `font_family` - Font family name
- `font_weight` - Font weight (400, 700, etc.)
- `text_color` - Text color
- `line_width` - Maximum line width for wrapping

**Containers**
- `children` - Array of child element IDs

**Size Behavior**
- `width_behavior`, `height_behavior` - `"content"` or `"fixed"`

## Custom Components

Create custom components by registering a factory function:

```rust
use volare_engine_layout::*;
use serde_json::{Map, Value};

fn create_my_component(
    id: &str,
    attributes: &Map<String, Value>,
    parser: &JsonLinesParser,
    builder: &mut DiagramBuilder,
) -> Result<DiagramTreeNode> {
    // Parse attributes
    let title = get_string_attr(attributes, &["title"], "Default Title");
    
    // Build component using DiagramBuilder API
    let text = builder.new_text(
        format!("{}_text", id),
        &title,
        TextOptions::default(),
    );
    
    Ok(builder.new_box(
        id.to_string(),
        text,
        BoxOptions::default(),
    ))
}

// Register the component
builder.register_custom_component("my_component", create_my_component);
```

Then use it in JSONL:
```jsonl
{"id":"root","type":"my_component","title":"Custom Component Title"}
```

## Integration

### Rust Library

Add to `Cargo.toml`:
```toml
[dependencies]
volare_engine_layout = { path = "path/to/volare_engine_layout" }
```

### WASM (Browser)

The engine compiles to WebAssembly for browser use:
```bash
wasm-pack build wasm_bindings --target web --out-dir ../pkg
```

### Planned: MCP Server

Model Context Protocol server for direct LLM integration (coming soon).

## Architecture

```
JSONL Input
    ↓
Parser (JsonLinesParser)
    ↓
Entity HashMap
    ↓
Tree Builder (recursive)
    ↓
Layout Engine (Cassowary constraints)
    ↓
Renderers (SVG, PNG, ...)
    ↓
Output
```

**Key Design Decisions:**
- **JSONL over JSON**: Single-pass parsing, no context needed between lines, easy for LLMs to generate
- **Constraint-based layouts**: Declarative relationships instead of manual coordinates
- **Two-phase parsing**: Load all entities first, then build tree (handles forward references)
- **Pluggable renderers**: Same specification → multiple output formats

## Roadmap

**Current Priorities**
- [ ] CLI tool for command-line rendering
- [ ] More high-level components (charts, diagrams, layouts)
- [ ] Icon system with embedded SVG paths
- [ ] Improved documentation and examples

**Future**
- [ ] MCP server for LLM integration
- [ ] PDF renderer
- [ ] Layout templates library
- [ ] Interactive editor (Tauri app)

## Status & Limitations

**What Works**
- Core layout engine with constraint solving
- SVG and PNG rendering
- Most basic components (shapes, text, containers, tables)
- Custom component system
- Ishikawa diagrams, calendars

**Known Limitations**
- CLI tool not yet available
- Some advanced features incomplete
- Documentation needs expansion
- Limited error messages

This is an alpha project under active development. APIs may change.

## Contributing

Contributions welcome! Areas needing help:
- Additional custom components
- More examples and documentation
- Test coverage
- Bug fixes and performance improvements

## License

MIT License

Copyright (c) 2025 Hugo Zapata

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Credits

Built with:
- [Cassowary](https://github.com/dylanede/cassowary-rs) - Constraint solving
- Rust image processing libraries

---

