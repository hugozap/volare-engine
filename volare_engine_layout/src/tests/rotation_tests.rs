#[cfg(test)]
mod simple_rotation_test {
    use crate::transform::Transform;
    use crate::{Fill, RectOptions, SizeBehavior};
    use crate::{DiagramBuilder, layout::layout_tree_node};

    #[test]
    fn test_90_degree_rotation_bounding_box() {
        println!("🧪 Testing 90° rotation bounding box...");
        
        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as f32 * 8.0, 16.0));
        
        // Create a 100x50 rectangle (wide rectangle)
        let rect_opts = RectOptions {
            width_behavior: SizeBehavior::Fixed(100.0),
            height_behavior: SizeBehavior::Fixed(50.0),
            fill_color: Fill::Color("blue".to_string()),
            stroke_color: "darkblue".to_string(),
            stroke_width: 1.0,
            border_radius: 0.0,
        };
        
        let rect = builder.new_rectangle("test_rect".to_string(), rect_opts);
        
        // Apply 90° rotation
        let rotation_transform = Transform::rotation(90.0);
        builder.set_transform("test_rect".to_string(), rotation_transform);
        
        // Layout the rectangle
        layout_tree_node(&mut builder, &rect);
        
        // Get the effective bounds
        let bounds = builder.get_effective_bounds("test_rect".to_string());
        
        println!("📏 Original size: 100x50 (wide rectangle)");
        println!("📐 Rotation: 90°");
        println!("📦 Effective bounds: w={:.1}, h={:.1}", bounds.width, bounds.height);
        
        // After 90° rotation: width and height should swap
        // Original: 100 wide, 50 tall → Rotated: 50 wide, 100 tall
        assert_eq!(bounds.width as i32, 50, "90° rotated width should be 50, got {:.1}", bounds.width);
        assert_eq!(bounds.height as i32, 100, "90° rotated height should be 100, got {:.1}", bounds.height);
        
        println!("✅ 90° rotation test passed! Dimensions swapped correctly.");
    }
}


#[cfg(test)]
mod debug_rotation_positioning {
    use super::*;
    use crate::transform::Transform;
    use crate::{Fill, HorizontalAlignment, RectOptions, SizeBehavior, VerticalAlignment};
    use crate::{DiagramBuilder, layout::layout_tree_node};

    #[test]
    fn test_debug_rotation_positioning() {
        println!("🧪 Debugging rotation positioning...");
        
        let mut builder = DiagramBuilder::new();
        builder.set_measure_text_fn(|text, _| (text.len() as f32 * 8.0, 16.0));
        
        // Create the same setup as your JSONL test
        let normal_rect = builder.new_rectangle("normal_rect".to_string(), RectOptions {
            width_behavior: SizeBehavior::Fixed(60.0),
            height_behavior: SizeBehavior::Fixed(40.0),
            fill_color: Fill::Color("red".to_string()),
            stroke_color: "darkred".to_string(),
            stroke_width: 2.0,
            border_radius: 0.0,
        });
        
        let rotated_rect = builder.new_rectangle("rotated_rect".to_string(), RectOptions {
            width_behavior: SizeBehavior::Fixed(60.0),
            height_behavior: SizeBehavior::Fixed(40.0),
            fill_color: Fill::Color("blue".to_string()),
            stroke_color: "darkblue".to_string(),
            stroke_width: 2.0,
            border_radius: 0.0,
        });
        
        let normal_rect2 = builder.new_rectangle("normal_rect2".to_string(), RectOptions {
            width_behavior: SizeBehavior::Fixed(60.0),
            height_behavior: SizeBehavior::Fixed(40.0),
            fill_color: Fill::Color("green".to_string()),
            stroke_color: "darkgreen".to_string(),
            stroke_width: 2.0,
            border_radius: 0.0,
        });
        
        // Apply rotation to middle rectangle
        let rotation_transform = Transform::rotation(45.0);
        builder.set_transform("rotated_rect".to_string(), rotation_transform);
        
        println!("🔍 BEFORE LAYOUT:");
        println!("  Normal rect bounds: {:?}", builder.get_effective_bounds("normal_rect".to_string()));
        println!("  Rotated rect bounds: {:?}", builder.get_effective_bounds("rotated_rect".to_string()));
        println!("  Normal rect2 bounds: {:?}", builder.get_effective_bounds("normal_rect2".to_string()));
        
        // Create horizontal stack
        let hstack = builder.new_hstack("root".to_string(), 
                                       vec![normal_rect, rotated_rect, normal_rect2], 
                                       VerticalAlignment::Center);
        
        // Layout the stack
        layout_tree_node(&mut builder, &hstack);
        
        println!("🔍 AFTER LAYOUT:");
        println!("  Normal rect position: {:?}", builder.get_position("normal_rect".to_string()));
        println!("  Normal rect bounds: {:?}", builder.get_effective_bounds("normal_rect".to_string()));
        
        println!("  Rotated rect position: {:?}", builder.get_position("rotated_rect".to_string()));
        println!("  Rotated rect bounds: {:?}", builder.get_effective_bounds("rotated_rect".to_string()));
        println!("  Rotated rect transform: {:?}", builder.get_transform("rotated_rect".to_string()));
        
        println!("  Normal rect2 position: {:?}", builder.get_position("normal_rect2".to_string()));
        println!("  Normal rect2 bounds: {:?}", builder.get_effective_bounds("normal_rect2".to_string()));
        
        println!("  Stack total size: {:?}", builder.get_size("root".to_string()));
        
        // Check if there's overlap
        let rect1_bounds = builder.get_effective_bounds("normal_rect".to_string());
        let rect2_bounds = builder.get_effective_bounds("rotated_rect".to_string());
        let rect3_bounds = builder.get_effective_bounds("normal_rect2".to_string());
        
        println!("🔍 OVERLAP CHECK:");
        println!("  Rect1 occupies: x={:.1} to x={:.1}", rect1_bounds.x, rect1_bounds.x + rect1_bounds.width);
        println!("  Rect2 occupies: x={:.1} to x={:.1}", rect2_bounds.x, rect2_bounds.x + rect2_bounds.width);
        println!("  Rect3 occupies: x={:.1} to x={:.1}", rect3_bounds.x, rect3_bounds.x + rect3_bounds.width);
        
        // Check for actual overlap
        let rect1_end = rect1_bounds.x + rect1_bounds.width;
        let rect2_start = rect2_bounds.x;
        let rect2_end = rect2_bounds.x + rect2_bounds.width;
        let rect3_start = rect3_bounds.x;
        
        if rect2_start < rect1_end {
            println!("❌ OVERLAP DETECTED: Rect2 starts at {:.1} but Rect1 ends at {:.1}", rect2_start, rect1_end);
        } else {
            println!("✅ No overlap between Rect1 and Rect2");
        }
        
        if rect3_start < rect2_end {
            println!("❌ OVERLAP DETECTED: Rect3 starts at {:.1} but Rect2 ends at {:.1}", rect3_start, rect2_end);
        } else {
            println!("✅ No overlap between Rect2 and Rect3");
        }
    }
}