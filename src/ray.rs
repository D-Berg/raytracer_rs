
use crate::vec3::{Color, Point3, Vec3};

/// P(t) = A + t*b
/// P is a 3D position along a line.
/// A is the origin, b is the rays direction
#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {

    pub fn new(origin: Point3, direction: Vec3) -> Self {

        Ray {
            origin,
            direction
        }

    }

    
    pub fn at(&self, t: f64) -> Point3 {

        return &self.origin + &(&self.direction * t);

    }

    pub fn color(&self) -> Color {

        let a: f64 = 0.5 * (self.direction.y()  + 1.0);

        (1.0 - a) * Color::ones() + a * Color::new(0.5, 0.7, 1.0)

    }
}
