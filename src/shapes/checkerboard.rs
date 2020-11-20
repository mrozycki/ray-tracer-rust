use color::Color;
use geometry::Line3d;
use nalgebra::{Point3, Vector3, Unit};
use shapes::Shape;

pub struct CheckerBoard {
    height: f64,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    reflectiveness: f64,
}

impl CheckerBoard {
    pub fn new(height: f64) -> Self {
        CheckerBoard {
            height,
            ambient: 0.0,
            diffuse: 0.6,
            specular: 0.4,
            reflectiveness: 0.8,
        }
    }
}

impl Shape for CheckerBoard {
    fn intersect(&self, ray: &Line3d) -> Vec<(&dyn Shape, Point3<f64>)> {
        if ray.direction().z == 0.0 {
            return Vec::new();
        }

        let d = (self.height - ray.origin().z) / ray.direction().z;
        vec![(self as &dyn Shape, ray.at(d))]
    }

    fn color_at(&self, position: &Point3<f64>) -> Color {
        if ((position.x.floor() + position.y.floor()).abs() as i64) % 2 == 1 {
            Color::gray(100)
        } else {
            Color::gray(128)
        }
    }

    fn normal_at(&self, _: &Point3<f64>) -> Unit<Vector3<f64>> {
        Vector3::z_axis()
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

    fn reflectiveness(&self) -> f64 {
        self.reflectiveness
    }
}
