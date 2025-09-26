//components/src/lib.rs
// Main library file for Volare Engine custom components

// Re-export the main engine for convenience
pub use volare_engine_layout::*;

// Component modules
pub mod document;
// You can add more component domains here as you create them:
// pub mod business_process;
// pub mod technical_diagrams;
// pub mod educational;
// pub mod data_viz;

// Convenience function to register ALL component sets at once
/// Register all available component sets with a DiagramBuilder
/// This is a convenience function that registers components from all domains
pub fn register_all_components(builder: &mut DiagramBuilder) {
   
    document::register_document_components(builder);
    println!("All component libraries registered successfully!");
}
