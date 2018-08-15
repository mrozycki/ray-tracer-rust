extern crate rand;
use color::Color;
use geometry::{Line3d, utils::*};
use nalgebra::{Point3, Vector3, Unit};
use shapes::Shape;

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(2);
    let delta = b * b - 4.0 * a * c;
    if delta >= 0f64 {
        result.push((-b - delta.sqrt()) / (2.0 * a));
        result.push((-b + delta.sqrt()) / (2.0 * a));
    }

    result
}

pub struct Sphere {
    color: Color,
    center: Point3<f64>,
    radius: f64,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    reflectiveness: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, color: Color) -> Self {
        Sphere {
            color,
            center,
            radius,
            ambient: 0.0,
            diffuse: 0.5,
            specular: 0.5,
            reflectiveness: 0.0,
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        (self.center - other.center).norm() <= self.radius + other.radius
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Line3d) -> Vec<(&Shape, Point3<f64>)> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(&(ray.origin() - self.center));
        let c = ray.origin().distance_to(&self.center).powi(2) - self.radius.powi(2);

        solve_quadratic(a, b, c).into_iter()
            .map(|d| (self as &Shape, ray.at(d)))
            .collect()
    }

    fn color_at(&self, _: &Point3<f64>) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, position: &Point3<f64>) -> Unit<Vector3<f64>> {
        Unit::new_normalize(position - self.center)
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
