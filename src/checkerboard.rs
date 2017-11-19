use color::Color;
use geometry::{Line3d, Vector3d};
use shape::Shape;

pub struct CheckerBoard {
    height: f64,
    ambient: f64,
    diffuse: f64,
    specular: f64,
}

impl CheckerBoard {
    pub fn new(height: f64) -> Self {
        CheckerBoard { height: height, ambient: 0.2, diffuse: 0.8, specular: 0.0 }
    }
}

impl Shape for CheckerBoard {
    fn intersect(&self, ray: Line3d) -> Vec<(&Shape, Vector3d)> {
        if ray.l.z() == 0.0 {
            return Vec::new();
        }
        let d = (self.height-ray.o.z())/ray.l.z();

        vec!((self as &Shape, ray.at(d)))
    }

    fn color_at(&self, position: Vector3d) -> Color {
        if ((position.x().floor() + position.y().floor()).abs() as i64) % 2 == 1 {
            Color::gray(0)
        } else {
            Color::gray(255)
        }
    }

    fn normal_at(&self, _: Vector3d) -> Vector3d {
        Vector3d::new(0.0, 0.0, 1.0)
    }

    fn ambient_light(&self) -> f64 {
        self.ambient
    }

    fn diffuse_coefficient(&self) -> f64 {
        self.diffuse
    }

    fn specular_coefficient(&self) -> f64 {
        self.specular
    }
}
