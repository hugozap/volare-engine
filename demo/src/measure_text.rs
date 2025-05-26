use rusttype::{point, Font, Scale, PositionedGlyph};
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
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
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
            
        (max_x - min_x) as f32 // Precise width without buffer
    } else {
        0.0
    };
    
    // Use the larger of the two width measurements to ensure we don't underestimate
    let final_width = width.max(glyph_width);
    
    // Calculate exact height without any buffer
    let height = v_metrics.ascent - v_metrics.descent;
    
    // Instead of adding an asymmetric margin, we'll provide the exact text dimensions
    // with no additional buffer unless explicitly needed
    // This ensures text fits precisely without extra margins
    let exact_width = final_width; // No buffer - exact measurement
    
    // Same for height - exact measurement
    // We don't add buffer to height either for more precise layout
    
    println!("{}: {}x{} (original measurement: {})", text, exact_width, height, width);
    
    // Convert to f64 for return
    (exact_width.into(), height.into())
}


pub fn measure_text_svg(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    // Standard SVG DPI
    let scale_factor = 96.0 / 72.0;
    let scale = Scale::uniform(options.font_size * scale_factor);
    
    // Handle empty text case
    if text.is_empty() {
        return (0.0, options.font_size as f64);
    }
    
    // Layout all glyphs with proper kerning
    let mut positioned_glyphs = Vec::new();
    let mut caret = point(0.0, 0.0);  // Position at baseline
    let mut prev_glyph_id = None;

    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        // Apply kerning
        if let Some(prev_id) = prev_glyph_id {
            caret.x += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        // Position the glyph
        let positioned_glyph = base_glyph.scaled(scale).positioned(caret);
        
        // Advance the cursor
        caret.x += positioned_glyph.unpositioned().h_metrics().advance_width;
        
        positioned_glyphs.push(positioned_glyph);
        prev_glyph_id = Some(glyph_id);
    }
    
    // Calculate width from actual glyph bounds
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut has_pixel_bounds = false;
    
    for glyph in &positioned_glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x as f32);
            max_x = max_x.max(bb.max.x as f32);
            min_y = min_y.min(bb.min.y as f32);
            max_y = max_y.max(bb.max.y as f32);
            has_pixel_bounds = true;
        }
    }
    
    // For space-only text
    if !has_pixel_bounds {
        min_x = 0.0;
        max_x = positioned_glyphs.iter()
            .map(|g| g.unpositioned().h_metrics().advance_width)
            .sum::<f32>();
        
        // Default height for space-only text
        min_y = -options.font_size * scale_factor * 0.7;  // Approximate for ascender
        max_y = options.font_size * scale_factor * 0.3;   // Approximate for descender
    }
    
    // Get accurate width
    let width = max_x - min_x;
    
    // BROWSER-MATCHED HEIGHT CALCULATION
    // In browsers, the text height is typically very close to the line-height
    // which defaults to around 1.2 times the font size
    let browser_line_height_factor = 1.2;
    let browser_height = options.font_size * scale_factor * browser_line_height_factor;
    
    // For precise bbox fitting without extra space, we can also calculate
    // the actual rendered height from glyph bounds
    let actual_rendered_height = if has_pixel_bounds {
        max_y - min_y
    } else {
        options.font_size * scale_factor
    };
    
    // We'll use the actual rendered height rather than browser_height
    // This gives a tighter fit around the text
    let height = actual_rendered_height;
    
    // Adjust width for overhangs
    let width_adjustment = if has_extreme_overhang(text) {
        0.95  // Reduce slightly for overhanging chars
    } else {
        1.0
    };
    
    let final_width = width * width_adjustment;
    
    // For debugging
    println!("Text: '{}' - Width: {:.2} × Height: {:.2}", text, final_width, height);
    println!("  Bounds: X({:.2} to {:.2}), Y({:.2} to {:.2})", min_x, max_x, min_y, max_y);
    
    (final_width as f64, height as f64)
}

// Helper function to detect text with characters that typically have overhangs
fn has_extreme_overhang(text: &str) -> bool {
    text.chars().any(|c| matches!(c, 'f' | 'j' | 'y' | 'g' | 'p' | 'q' | 'T' | 'W' | 'A' | 'V'))
}

