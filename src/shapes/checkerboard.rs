use crate::color::Color;
use crate::geometry::Line3d;
use crate::shapes::{Material, Shape};

use nalgebra::{Point3, Unit, Vector3};

pub struct CheckerBoard {
    height: f64,
    material: Material,
}

impl CheckerBoard {
    pub fn new(height: f64) -> Self {
        CheckerBoard {
            height,
            material: Material {
                ambient_light: 0.0,
                diffuse_coefficient: 0.6,
                specular_coefficient: 0.4,
                reflectiveness: 0.8,
                transparency: 0.0,
                refractive_index: 1.0,
            },
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

    fn material(&self) -> &Material {
        &self.material
    }
}
