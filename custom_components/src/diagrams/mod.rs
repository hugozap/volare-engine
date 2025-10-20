mod ichikawa;
pub use ichikawa::create_ishikawa;
use volare_engine_layout::DiagramBuilder;

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    println!("📄 Diagram components registered");
}
