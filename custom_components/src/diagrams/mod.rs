mod ichikawa;
mod activity;
pub use ichikawa::create_ishikawa;
pub use activity::create_activity_diagram;
use volare_engine_layout::DiagramBuilder;

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    builder.register_custom_component("activity_diagram", create_activity_diagram);
    println!("📄 Diagram components registered");
}
