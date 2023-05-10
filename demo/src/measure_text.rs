use rusttype::{point, Font, Scale};
use volare_engine_layout::TextOptions;

/* Implementation of the measure text function required by the session
using rusttype

Note about DPI

 Rusttype, the font size is expressed in "points" or
"postscript points."
One point is equal to 1/72 of an inch.
 The font size value you pass to the Scale::uniform function 
 will determine the size of the text when rendered at 72 DPI (dots per inch).

Browsers use 96 DPI by default so we need to scale the font size
 */

pub fn measure_text(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    //Web browsers use 96 dpi by default so we need to scale the font size
    //Rust type uses 72 dpi by default
    let dpi = 96.0;
    let scale_factor = dpi / 72.0;
    let scale = Scale::uniform(options.font_size * scale_factor);
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
