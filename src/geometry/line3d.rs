use nalgebra::{Point3, Vector3, Unit};
use geometry::utils::*;

#[derive(Clone, Copy)]
pub struct Line3d {
    origin: Point3<f64>,
    direction: Unit<Vector3<f64>>
}

impl Line3d {
    pub fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Line3d {
        Line3d { origin, direction }
    }

    pub fn between(a: &Point3<f64>, b: &Point3<f64>) -> Line3d {
        Line3d { origin: a.clone(), direction: a.unit_to(b) }
    }

    pub fn at(&self, d: f64) -> Point3<f64> {
        self.origin + self.direction.unwrap() * d
    }

    pub fn project(&self, p: Point3<f64>) -> f64 {
        (p - self.origin).dot(&self.direction)
    }

    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction.unwrap()
    }
}
