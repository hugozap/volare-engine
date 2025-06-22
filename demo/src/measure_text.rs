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
    let dpi = 96.0; // Increased from 96.0 for better accuracy
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
   //(100.0,100.0)
}


// Method 1: Advance width minus last character's trailing space
pub fn measure_text_tight_advance(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    let mut total_width = 0.0f32;
    let mut prev_glyph_id = None;

    // Process all characters except the last one with full advance width
    for (i, &c) in chars.iter().enumerate() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        if let Some(prev_id) = prev_glyph_id {
            total_width += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        if i == chars.len() - 1 {
            // For the last character, use actual glyph width instead of advance width
            let scaled_glyph = base_glyph.scaled(scale);
            if let Some(bb) = scaled_glyph.positioned(point(0.0, 0.0)).pixel_bounding_box() {
                total_width += bb.width() as f32;
            } else {
                // Fallback for whitespace characters - create a fresh glyph for metrics
                let scaled_glyph_for_metrics = font.glyph(c).scaled(scale);
                total_width += scaled_glyph_for_metrics.h_metrics().advance_width;
            }
        } else {
            // Get advance width before using the glyph
            total_width += base_glyph.scaled(scale).h_metrics().advance_width;
        }
        
        prev_glyph_id = Some(glyph_id);
    }

    let height = v_metrics.ascent - v_metrics.descent;
    
    println!("TIGHT ADVANCE: '{}' -> {:.2}x{:.2}", text, total_width, height);
    (total_width as f64, height as f64)
}

pub fn debug_text_measurement(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    // Method 1: Advance width approach (your current method)
    let chars: Vec<char> = text.chars().collect();
    let mut total_advance_width = 0.0f32;
    let mut prev_glyph_id = None;

    println!("=== ADVANCE WIDTH DEBUG ===");
    for (i, &c) in chars.iter().enumerate() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        let kerning = if let Some(prev_id) = prev_glyph_id {
            font.pair_kerning(scale, prev_id, glyph_id)
        } else {
            0.0
        };
        
        total_advance_width += kerning;
        
        if i == chars.len() - 1 {
            // Last character - use pixel width
            let scaled_glyph = base_glyph.scaled(scale);
            if let Some(bb) = scaled_glyph.positioned(point(0.0, 0.0)).pixel_bounding_box() {
                let char_width = bb.width() as f32;
                total_advance_width += char_width;
                println!("  Char '{}' (LAST): kerning={:.2}, pixel_width={:.2}", c, kerning, char_width);
            } else {
                let char_advance = font.glyph(c).scaled(scale).h_metrics().advance_width;
                total_advance_width += char_advance;
                println!("  Char '{}' (LAST, no pixels): kerning={:.2}, advance={:.2}", c, kerning, char_advance);
            }
        } else {
            let char_advance = base_glyph.scaled(scale).h_metrics().advance_width;
            total_advance_width += char_advance;
            println!("  Char '{}': kerning={:.2}, advance={:.2}", c, kerning, char_advance);
        }
        
        prev_glyph_id = Some(glyph_id);
    }
    
    println!("Total advance width: {:.2}", total_advance_width);

    // Method 2: Pure pixel bounds approach
    println!("\n=== PIXEL BOUNDS DEBUG ===");
    let mut positioned_glyphs = Vec::new();
    let mut caret = point(0.0, 0.0);
    let mut prev_glyph_id = None;

    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        if let Some(prev_id) = prev_glyph_id {
            caret.x += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        let positioned_glyph = base_glyph.scaled(scale).positioned(caret);
        let advance_width = positioned_glyph.unpositioned().h_metrics().advance_width;
        
        if let Some(bb) = positioned_glyph.pixel_bounding_box() {
            println!("  Char '{}' at x={:.2}: bounds=({}, {}) to ({}, {}), width={}", 
                     c, caret.x, bb.min.x, bb.min.y, bb.max.x, bb.max.y, bb.width());
        } else {
            println!("  Char '{}' at x={:.2}: no pixel bounds, advance={:.2}", c, caret.x, advance_width);
        }
        
        positioned_glyphs.push(positioned_glyph);
        caret.x += advance_width;
        prev_glyph_id = Some(glyph_id);
    }

    // Calculate exact visual bounds
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;

    for glyph in &positioned_glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x as f32);
            max_x = max_x.max(bb.max.x as f32);
        }
    }

    let pixel_bounds_width = if min_x.is_finite() && max_x.is_finite() {
        max_x - min_x
    } else {
        total_advance_width
    };

    println!("Pixel bounds: min_x={:.2}, max_x={:.2}, width={:.2}", min_x, max_x, pixel_bounds_width);
    println!("Left padding from bounds: {:.2}", min_x);
    println!("Right padding potential: {:.2}", total_advance_width - max_x);
    
    // Method 3: Tightest possible measurement
    let tightest_width = pixel_bounds_width;
    
    println!("\n=== COMPARISON ===");
    println!("Advance method: {:.2}", total_advance_width);
    println!("Pixel bounds: {:.2}", pixel_bounds_width);
    println!("Difference: {:.2}", total_advance_width - pixel_bounds_width);

    let height = v_metrics.ascent - v_metrics.descent;
    
    // Return the pixel bounds width (tightest)
    (tightest_width as f64, height as f64)
}


// Ultra-tight measurement that accounts for actual glyph positioning
pub fn measure_text_ultra_tight(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    // Use rusttype's layout function which handles everything correctly
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
        max_x - min_x  // Pure visual width, no padding
    } else {
        // Fallback for whitespace
        glyphs.iter().map(|g| g.unpositioned().h_metrics().advance_width).sum()
    };

    let height = v_metrics.ascent - v_metrics.descent;
    
    println!("ULTRA TIGHT: '{}' -> {:.2}x{:.2}", text, width, height);
    (width as f64, height as f64)
}



// Method 2: Pure pixel bounds (most accurate)
pub fn measure_text_pure_bounds(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    let mut positioned_glyphs = Vec::new();
    let mut caret = point(0.0, 0.0);
    let mut prev_glyph_id = None;

    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        if let Some(prev_id) = prev_glyph_id {
            caret.x += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        let positioned_glyph = base_glyph.scaled(scale).positioned(caret);
        
        // Get advance width for caret positioning BEFORE moving the glyph
        let advance_width = positioned_glyph.unpositioned().h_metrics().advance_width;
        
        positioned_glyphs.push(positioned_glyph);
        
        // Advance caret for next glyph positioning
        caret.x += advance_width;
        prev_glyph_id = Some(glyph_id);
    }

    // Calculate exact visual bounds
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut has_visible_bounds = false;

    for glyph in &positioned_glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x as f32);
            max_x = max_x.max(bb.max.x as f32);
            has_visible_bounds = true;
        }
    }

    let width = if has_visible_bounds {
        max_x - min_x  // Exact visual width
    } else {
        // Fallback for whitespace-only text
        positioned_glyphs.iter()
            .map(|g| g.unpositioned().h_metrics().advance_width)
            .sum::<f32>()
    };

    let height = v_metrics.ascent - v_metrics.descent;
    
    println!("PURE BOUNDS: '{}' -> {:.2}x{:.2}", text, width, height);
    (width as f64, height as f64)
}



// Method 1: No DPI scaling (font_size is already in pixels)
pub fn measure_text_no_scaling(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    // Use font size directly as pixels
    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    let mut total_width = 0.0f32;
    let mut prev_glyph_id = None;

    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        if let Some(prev_id) = prev_glyph_id {
            total_width += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        total_width += base_glyph.scaled(scale).h_metrics().advance_width;
        prev_glyph_id = Some(glyph_id);
    }

    let height = v_metrics.ascent - v_metrics.descent;
    
    println!("NO SCALING: '{}' -> {:.2}x{:.2}", text, total_width, height);
    (total_width as f64, height as f64)
}


// Alternative approach using advance width (more predictable for layout)
pub fn measure_text_advance(text: &str, options: &TextOptions) -> (f64, f64) {
    let font_data = include_bytes!("../assets/AnonymiceProNerdFont-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let dpi = 96.0;
    let scale_factor = dpi / 72.0;
    let scale = Scale::uniform(options.font_size * scale_factor);
    let v_metrics = font.v_metrics(scale);

    if text.is_empty() {
        return (0.0, (v_metrics.ascent - v_metrics.descent) as f64);
    }

    let mut total_width = 0.0f32;
    let mut prev_glyph_id = None;

    for c in text.chars() {
        let base_glyph = font.glyph(c);
        let glyph_id = base_glyph.id();
        
        // Add kerning
        if let Some(prev_id) = prev_glyph_id {
            total_width += font.pair_kerning(scale, prev_id, glyph_id);
        }
        
        // Add advance width
        total_width += base_glyph.scaled(scale).h_metrics().advance_width;
        prev_glyph_id = Some(glyph_id);
    }

    let height = v_metrics.ascent - v_metrics.descent;
    
    println!("Text: '{}' -> {:.2}x{:.2} (advance method)", text, total_width, height);
    (total_width as f64, height as f64)
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

