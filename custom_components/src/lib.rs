//components/src/lib.rs
// Main library file for Volare Engine custom components

// Re-export the main engine for convenience
pub use volare_engine_layout::*;

// Component modules
pub mod infographics;
// You can add more component domains here as you create them:
// pub mod business_process;
// pub mod technical_diagrams;
// pub mod educational;
// pub mod data_viz;

// Convenience function to register ALL component sets at once
/// Register all available component sets with a DiagramBuilder
/// This is a convenience function that registers components from all domains
pub fn register_all_components(builder: &mut DiagramBuilder) {
    infographics::register_infographic_components(builder);
    
    // Add future component registrations here:
    // business_process::register_business_process_components(builder);
    // technical_diagrams::register_technical_components(builder);
    // educational::register_educational_components(builder);
    
    println!("All component libraries registered successfully!");
}

// Re-export commonly used component registration functions for convenience
pub use infographics::register_infographic_components;
