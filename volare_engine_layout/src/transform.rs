// In src/transform.rs (new file)
use std::f32::consts::PI;

use crate::{BoundingBox, Float};

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    // 2D transform matrix [a, b, c, d, e, f]
    // | a  c  e |   | x |   | a*x + c*y + e |
    // | b  d  f | * | y | = | b*x + d*y + f |
    // | 0  0  1 |   | 1 |   |       1       |
    pub matrix: [Float; 6],
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], // [a, b, c, d, e, f]
        }
    }
    
    pub fn translation(tx: Float, ty: Float) -> Self {
        Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, tx, ty],
        }
    }
    
    pub fn rotation(angle_degrees: Float) -> Self {
        let angle_rad = angle_degrees * PI / 180.0;
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        Transform {
            matrix: [cos_a, sin_a, -sin_a, cos_a, 0.0, 0.0],
        }
    }
    
    pub fn scale(sx: Float, sy: Float) -> Self {
        Transform {
            matrix: [sx, 0.0, 0.0, sy, 0.0, 0.0],
        }
    }
    
    pub fn combine(&self, other: &Transform) -> Transform {
        // Matrix multiplication: self * other
        let [a1, b1, c1, d1, e1, f1] = self.matrix;
        let [a2, b2, c2, d2, e2, f2] = other.matrix;
        
        Transform {
            matrix: [
                a1 * a2 + c1 * b2,           // a
                b1 * a2 + d1 * b2,           // b
                a1 * c2 + c1 * d2,           // c
                b1 * c2 + d1 * d2,           // d
                a1 * e2 + c1 * f2 + e1,      // e
                b1 * e2 + d1 * f2 + f1,      // f
            ],
        }
    }
    
    pub fn transform_point(&self, x: Float, y: Float) -> (Float, Float) {
        let [a, b, c, d, e, f] = self.matrix;
        (
            a * x + c * y + e,
            b * x + d * y + f,
        )
    }
    
    pub fn transform_rect(&self, x: Float, y: Float, width: Float, height: Float) -> BoundingBox {
        // Transform all four corners and find axis-aligned bounding box
        let corners = [
            (x, y),
            (x + width, y),
            (x + width, y + height),
            (x, y + height),
        ];
        
        let transformed_corners: Vec<(Float, Float)> = corners
            .iter()
            .map(|(px, py)| self.transform_point(*px, *py))
            .collect();
        
        let min_x = transformed_corners.iter().map(|(x, _)| *x).fold(Float::INFINITY, f32::min);
        let max_x = transformed_corners.iter().map(|(x, _)| *x).fold(Float::NEG_INFINITY, f32::max);
        let min_y = transformed_corners.iter().map(|(_, y)| *y).fold(Float::INFINITY, f32::min);
        let max_y = transformed_corners.iter().map(|(_, y)| *y).fold(Float::NEG_INFINITY, f32::max);
        
        BoundingBox {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
    
     pub fn to_svg_string(&self) -> String {
        let [a, b, c, d, e, f] = self.matrix;
        
        // If it's just a translation, use the simpler translate syntax
        if a == 1.0 && b == 0.0 && c == 0.0 && d == 1.0 {
            if e == 0.0 && f == 0.0 {
                String::new() // Identity transform
            } else {
                format!("translate({} {})", e, f)
            }
        } else {
            // Full matrix transform
            format!("matrix({} {} {} {} {} {})", a, b, c, d, e, f)
        }
    }
}