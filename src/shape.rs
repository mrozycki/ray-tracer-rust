use geometry::{Line3d, Vector3d};
use color::Color;

pub trait Shape {
    fn intersect(&self, ray: Line3d) -> Vec<Vector3d>;
    fn color_at(&self, position: Vector3d) -> Color;
    fn normal_at(&self, position: Vector3d) -> Vector3d;

    fn ambient_light(&self) -> f64;
    fn diffuse_coefficient(&self) -> f64;
    fn specular_coefficient(&self) -> f64;
}
