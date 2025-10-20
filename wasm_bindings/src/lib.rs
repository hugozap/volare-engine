
use wasm_bindgen::prelude::*;
use volare_engine_layout::*;
use svg_renderer::SVGRenderer;
use custom_components::*;

// Simple text measurement function
fn simple_measure_text(text: &str, options: &TextOptions) -> (Float, Float) {
    let char_width = options.font_size * 0.6;
    let width = text.len() as Float * char_width;
    let height = options.font_size;
    (width, height)
}

// JavaScript callback for text measurement
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "measureTextCallback")]
    pub type MeasureTextCallback;

    #[wasm_bindgen(method, js_name = "call")]
    fn call(this: &MeasureTextCallback, text: &str, font_size: f32, font_family: &str, line_width: usize) -> Vec<f32>;
}

// Global storage for the JavaScript callback
static mut MEASURE_TEXT_CALLBACK: Option<MeasureTextCallback> = None;

fn measure_text_wrapper(text: &str, options: &TextOptions) -> (Float, Float) {
    unsafe {
        if let Some(ref callback) = MEASURE_TEXT_CALLBACK {
            let result = callback.call(
                text,
                options.font_size,
                &options.font_family,
                options.line_width
            );
            
            if result.len() >= 2 {
                (result[0], result[1])
            } else {
                // Fallback to simple measurement
                simple_measure_text(text, options)
            }
        } else {
            // Fallback to simple measurement
            simple_measure_text(text, options)
        }
    }
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
        builder.set_measure_text_fn(measure_text_wrapper);
        custom_components::register_all_components(&mut builder);
        println!("Volare Engine initialized with custom components!");
        VolareEngine { builder }
    }

        /// Set the JavaScript function to be used for text measurement
    /// The callback should take (text: string, fontSize: number, fontFamily: string, lineWidth: number)
    /// and return [width: number, height: number]
    #[wasm_bindgen(js_name = "setMeasureTextFunction")]
    pub fn set_measure_text_function(&mut self, callback: MeasureTextCallback) {
        unsafe {
            MEASURE_TEXT_CALLBACK = Some(callback);
        }
    }

    /// Create SVG from JSON Lines string
    #[wasm_bindgen]
    pub fn create_diagram(&mut self, jsonl: &str) -> Result<String, JsValue> {
        let mut parser = parser::JsonLinesParser::new();
        
        let root_id = parser.parse_string(jsonl)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let diagram = parser.build(&root_id, &mut self.builder)
            .map_err(|e| JsValue::from_str(&format!("Build error: {}", e)))?;

        layout::layout_diagram(&mut self.builder, &diagram);

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

#[wasm_bindgen]
pub fn generate_transformations_jsonl_prompt(input:&str, current_jsonl:&str) -> String {
    volare_engine_layout::generate_transformations_jsonl_prompt(input, current_jsonl)
}