use rusttype::{point, Font, Scale};
use volare_engine_layout::TextOptions;

/**
 * Measure text using SVG character advance method
 * This method calculates the width based on the advance width of each character,
 * taking into account kerning between characters.
 * It provides a more accurate width for SVG rendering, especially for variable-width fonts.
 */
pub fn measure_text_svg_character_advance(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);

    let mut total_width = 0.0;
    let mut prev_glyph_id = None;

    for ch in text.chars() {
        let glyph = font.glyph(ch).scaled(scale);

        // Use horizontal advance instead of bounding box
        let advance_width = glyph.h_metrics().advance_width;

        // Add kerning if available
        if let Some(prev_id) = prev_glyph_id {
            total_width += font.pair_kerning(scale, prev_id, glyph.id());
        }

        total_width += advance_width;
        prev_glyph_id = Some(glyph.id());
    }

    // Use font metrics for height instead of glyph bounds
    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent;

    (total_width as f64, height as f64)
}

// tight measurement that accounts for actual glyph positioning
// Used for PNG rendering
pub fn measure_text_ultra_tight(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    // Use rusttype's layout function which handles everything correctly
    // TODO: Needs unicode normalization for proper glyph handling, see comments of `layout` function
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();

    // Find the actual visual bounds
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;

    for glyph in &glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x as f32);
            max_x = max_x.max(bb.max.x as f32);
        }
    }

    let width = if min_x.is_finite() && max_x.is_finite() {
        max_x - min_x // Pure visual width, no padding
    } else {
        // Fallback for whitespace
        glyphs
            .iter()
            .map(|g| g.unpositioned().h_metrics().advance_width)
            .sum()
    };

    let height = v_metrics.ascent - v_metrics.descent;

    println!("ULTRA TIGHT: '{}' -> {:.2}x{:.2}", text, width, height);
    (width as f64, height as f64)
}
