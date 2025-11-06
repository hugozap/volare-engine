mod ichikawa;
mod calendar;
pub use ichikawa::create_ishikawa;
use volare_engine_layout::DiagramBuilder;

use crate::diagrams::calendar::create_calendar;

pub fn register_diagram_components(builder: &mut DiagramBuilder) {
    builder.register_custom_component("ishikawa", create_ishikawa);
    builder.register_custom_component("calendar", create_calendar);
    println!("📄 Diagram components registered");
}
