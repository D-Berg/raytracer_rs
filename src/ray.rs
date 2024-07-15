
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

        let t = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);


        if t > 0.0 {

            let surface_normal = (self.at(t) - Vec3::new(0.0, 0.0, -1.0))
                .normalize();

            let clr: Color = 0.5 * (surface_normal + 1.0);

            return clr;

        }

        let unit_direction = self.direction.normalize();

        let a: f64 = 0.5 * (unit_direction.y()  + 1.0);

        (1.0 - a) * Color::ones() + a * Color::new(0.5, 0.7, 1.0)

    }

    // temp code
    fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {

        let oc = center - &self.origin;

        let a = self.direction.dot(&self.direction);

        let b = -2.0 * self.direction.dot(&oc);
        
        let c = oc.dot(&oc) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }

        
    }
}
