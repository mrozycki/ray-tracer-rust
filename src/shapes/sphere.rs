use crate::color::Color;
use crate::geometry::{utils::*, Line3d};
use crate::math::Polynomial;
use crate::shapes::{Material, Shape};

use nalgebra::{Point3, Unit, Vector3};

pub struct Sphere {
    color: Color,
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, color: Color) -> Self {
        Sphere {
            color,
            center,
            radius,
            material: Material {
                ambient_light: 0.0,
                diffuse_coefficient: 0.5,
                specular_coefficient: 0.5,
                reflectiveness: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
            },
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        (self.center - other.center).norm() <= self.radius + other.radius
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Line3d) -> Vec<(&dyn Shape, Point3<f64>)> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(&(ray.origin() - self.center));
        let c = ray.origin().distance_to(&self.center).powi(2) - self.radius.powi(2);

        Polynomial::new(vec![c, b, a])
            .into_solutions()
            .into_iter()
            .map(|d| (self as &dyn Shape, ray.at(d)))
            .collect()
    }

    fn color_at(&self, _: &Point3<f64>) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, position: &Point3<f64>) -> Unit<Vector3<f64>> {
        Unit::new_normalize(position - self.center)
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
