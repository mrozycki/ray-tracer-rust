use crate::color::Color;
use crate::geometry::{utils::*, Line3d};
use crate::shapes::Material;

use nalgebra::{Point3, Unit, Vector3};

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Line3d) -> Vec<(&dyn Shape, Point3<f64>)>;
    fn color_at(&self, position: &Point3<f64>) -> Color;
    fn normal_at(&self, position: &Point3<f64>) -> Unit<Vector3<f64>>;
    fn material(&self) -> &Material;

    fn occludes(&self, a: &Point3<f64>, b: &Point3<f64>, radius: f64) -> bool {
        self.intersect(&Line3d::between(a, b))
            .into_iter()
            .filter(|&(_, position)| a.distance_to(&position) > radius)
            .filter(|&(_, position)| b.distance_to(&position) > radius)
            .filter(|&(_, position)| (position - a).dot(&(position - b)) < 0.0)
            .count()
            > 0
    }

    fn reflect(&self, ray: &Line3d, point: &Point3<f64>) -> Line3d {
        Line3d::new(
            point.clone(),
            Unit::new_normalize(ray.direction().reflect(&self.normal_at(point))),
        )
    }
}
