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

    // Use a higher DPI for more accurate measurements
    let dpi = 120.0; // Increased from 96.0 for better accuracy
    let scale_factor = dpi / 72.0;
    let scale = Scale::uniform(options.font_size * scale_factor);
    let v_metrics = font.v_metrics(scale);
    
    // Collect all glyphs with proper kerning
    let mut glyphs = Vec::new();
    let mut caret = point(0.0, v_metrics.ascent);
    let mut prev_glyph_id = None;
    
    // For each character in the text, calculate position with kerning
    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        // Add kerning if we have a previous glyph
        if let Some(prev_id) = prev_glyph_id {
            caret.x += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        // Get metrics for advance width (clone needed because scaled() consumes the glyph)
        let advance_width = font.glyph(c).scaled(scale).h_metrics().advance_width;
        
        // Position glyph and save to our collection
        let glyph = font.glyph(c).scaled(scale).positioned(caret);
        glyphs.push(glyph);
        
        // Advance caret and track the previous glyph for kerning
        caret.x += advance_width;
        prev_glyph_id = Some(glyph_id);
    }

    // Calculate width from the final caret position
    // This accounts for all advances including the final glyph
    let width = caret.x;
    
    // Calculate exact pixel bounds width as a fallback/sanity check
    let glyph_width = if !glyphs.is_empty() {
        // Find the furthest x extent from the pixel bounding boxes
        let min_x = glyphs.iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.min.x)
            .min()
            .unwrap_or(0);
            
        let max_x = glyphs.iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.x)
            .max()
            .unwrap_or(0);
            
        (max_x - min_x) as f32 + 2.0 // Add a small buffer
    } else {
        0.0
    };
    
    // Use the larger of the two width measurements to ensure we don't underestimate
    let final_width = width.max(glyph_width);
    
    // Calculate height with a slight increase to account for descenders
    let height = (v_metrics.ascent - v_metrics.descent) + 2.0;
    
    // Add a small safety factor (5%) to ensure we have enough space
    let width_with_margin = final_width * 1.05;
    
    println!("{}: {}x{} (original measurement: {})", text, width_with_margin, height, width);
    
    // Convert to f64 for return
    (width_with_margin.into(), height.into())
}
