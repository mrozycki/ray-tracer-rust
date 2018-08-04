extern crate rand;
use color::Color;
use geometry::{Line3d, Vector3d};
use rand::distributions::{IndependentSample, Range};
use shape::Shape;
use uuid::Uuid;

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
    center: Vector3d,
    radius: f64,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    uuid: Uuid,
}

impl Sphere {
    pub fn new(center: Vector3d, radius: f64, color: Color) -> Self {
        Sphere {
            color,
            center,
            radius,
            ambient: 0.0,
            diffuse: 0.5,
            specular: 0.5,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let radius_range = Range::new(0.1, 0.5);
        let radius = radius_range.ind_sample(&mut rng);
        let position_range_x = Range::new(-2.0, 30.0);
        let position_range_y = Range::new(-10.0, 10.0);
        let (x, y) = (
            position_range_x.ind_sample(&mut rng),
            position_range_y.ind_sample(&mut rng),
        );

        Sphere::new(Vector3d::new(x, y, radius), radius, Color::random())
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Line3d) -> Vec<(&Shape, Vector3d)> {
        let a = 1.0;
        let b = 2.0 * ray.l.dot(ray.o.sub(self.center));
        let c = ray.o.sub(self.center).norm().powi(2) - self.radius.powi(2);

        solve_quadratic(a, b, c).into_iter()
            .map(|d| (self as &Shape, ray.at(d)))
            .collect()
    }

    fn color_at(&self, _: Vector3d) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, position: Vector3d) -> Vector3d {
        position.sub(self.center).unit()
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

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}
