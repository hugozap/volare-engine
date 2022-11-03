
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

// impl bottom getter
impl BoundingBox {
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }
}

//test boundingbox

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundingbox() {
        let bb = BoundingBox {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0
        };

        assert_eq!(bb.bottom(), 10.0);
        assert_eq!(bb.right(), 10.0);
    }
}