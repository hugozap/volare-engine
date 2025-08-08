use wasm_bindgen::prelude::*;
use volare_engine_layout::*;
use svg_renderer::SVGRenderer;

// Simple text measurement function
fn simple_measure_text(text: &str, options: &TextOptions) -> (Float, Float) {
    let char_width = options.font_size * 0.6;
    let width = text.len() as Float * char_width;
    let height = options.font_size;
    (width, height)
}

// Main WASM interface
#[wasm_bindgen]
pub struct VolareEngine {
    builder: DiagramBuilder,
}

#[wasm_bindgen]
impl VolareEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VolareEngine {
        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(simple_measure_text);
        
        VolareEngine { builder }
    }

    /// Create SVG from JSON Lines string
    #[wasm_bindgen]
    pub fn create_diagram(&mut self, jsonl: &str) -> Result<String, JsValue> {
        let mut parser = parser::JsonLinesParser::new();
        
        let root_id = parser.parse_string(jsonl)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let diagram = parser.build(&root_id, &mut self.builder)
            .map_err(|e| JsValue::from_str(&format!("Build error: {}", e)))?;

        layout::layout_tree_node(&mut self.builder, &diagram);

        let mut svg_output = Vec::new();
        let svg_renderer = SVGRenderer {};
        svg_renderer.render(&self.builder, &diagram, &mut svg_output)
            .map_err(|e| JsValue::from_str(&format!("Render error: {}", e)))?;

        String::from_utf8(svg_output)
            .map_err(|e| JsValue::from_str(&format!("UTF-8 error: {}", e)))
    }

    /// Clear internal state
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.builder.clear_cache();
    }
}

// Utility function
#[wasm_bindgen]
pub fn greet(name: &str) {
    web_sys::console::log_1(&format!("Hello, {}! Volare Engine ready.", name).into());
}