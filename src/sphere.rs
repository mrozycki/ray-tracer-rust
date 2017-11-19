use color::Color;
use geometry::{Line3d, Vector3d};
use shape::Shape;

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(2);
    let delta = b*b - 4.0*a*c;
    if delta >= 0f64 {
        result.push((-b-delta.sqrt())/(2.0*a));
        result.push((-b+delta.sqrt())/(2.0*a));
    }

    result
}

pub struct Sphere {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
}

impl Sphere {
    pub fn new(color: Color) -> Self {
        Sphere { color: color, ambient: 0.2, diffuse: 0.4, specular: 0.4 }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Line3d) -> Vec<Vector3d> {
        let a = 1.0;
        let b = 2.0 * ray.l.dot(ray.o);
        let c = ray.o.norm().powi(2) - 1.0;

        let ds = solve_quadratic(a, b, c);
        let mut result = Vec::with_capacity(2);
        for d in ds {
            let intersection = ray.at(d);
            result.push(intersection);
        }

        result
    }

    fn color_at(&self, _: Vector3d) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, position: Vector3d) -> Vector3d {
        position.unit()
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
