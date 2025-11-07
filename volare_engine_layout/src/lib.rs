
pub use crate::diagram_builder::{DiagramBuilder};
pub use crate::components::*;
pub use crate::constraints::*;
pub use crate::layout::*;
pub use crate::utils::*;
pub use crate::renderer_base::*;
pub use crate::theme::*;

pub mod diagram_builder;
pub mod utils;
pub mod components;
pub mod constraints;
pub mod layout;
pub mod renderer_base;
pub mod parser;
pub mod transform;
pub mod theme;

/// Generates a complete LLM prompt for converting natural language to JSONL operations
/// 
/// # Arguments
/// * `user_input` - The natural language request from the user
/// * `current_jsonl` - The current JSONL document (empty string for new documents)
/// 
/// # Returns
/// A formatted string containing the complete prompt ready to send to an LLM
/// 
/// # Example
/// ```
// let user_request = "Create a document with a title and paragraph";
// let current_doc = ""; // empty for new document
// let prompt = generate_jsonl_prompt(user_request, current_doc);
/// // Send prompt to LLM...
/// ```
pub fn generate_transformations_jsonl_prompt(user_input: &str, current_jsonl: &str) -> String {
    // Embed the documentation files at compile time
    const LLM_TRANSFORM_OPS_SPEC: &str = include_str!("../docs/LLM Transform Operations SPEC.md");
    const COMPONENTS_DOC: &str = include_str!("../docs/JSONL Document components.md");
    const DIAGRAM_SPEC: &str = include_str!("../docs/JSONL Diagram Specification.md");
    
    // Combine the specifications
    let complete_spec = format!(
        "{}\n\n## Document Components\n\n{}\n\n## Diagram Elements\n\n{}",
        COMPONENTS_DOC,
        DIAGRAM_SPEC,
        LLM_TRANSFORM_OPS_SPEC,
    );
    
    // Build the complete prompt by injecting the spec and user inputs
    let prompt = format!(
r#"{}

## Current Task

**User Request:**
{}

**Current Document:**
{}

Please generate the JSONL operations to fulfill this request. Output ONLY the JSONL operations, one per line, with no additional formatting or explanation.
"#,
        complete_spec,
        user_input,
        if current_jsonl.trim().is_empty() {
            "(empty)"
        } else {
            current_jsonl
        }
    );
    
    prompt
}

#[cfg(test)]
mod tests;


