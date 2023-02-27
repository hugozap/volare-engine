use rusttype::{point, Font, Scale};
use volare_engine_layout::TextOptions;

/* Implementation of the measure text function required by the session
using rusttype */

pub fn measure_text(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();
    let width = glyphs
        .last()
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0);
    let height = v_metrics.ascent - v_metrics.descent;
    println!("{}: {}x{}", text, width, height);
    (width.into(), height.into())
}
