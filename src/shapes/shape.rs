use color::Color;
use geometry::{Line3d, Vector3d};

pub trait Shape : Send + Sync {
    fn intersect(&self, ray: Line3d) -> Vec<(&Shape, Vector3d)>;
    fn color_at(&self, position: Vector3d) -> Color;
    fn normal_at(&self, position: Vector3d) -> Vector3d;

    fn ambient_light(&self) -> f64;
    fn diffuse_coefficient(&self) -> f64;
    fn specular_coefficient(&self) -> f64;
    fn reflectiveness(&self) -> f64;

    fn occludes(&self, a: Vector3d, b: Vector3d, radius: f64) -> bool {
        self.intersect(Line3d::new(b.sub(a).unit(), a)).into_iter()
            .filter(|&(_, position)| position.sub(a).norm() > radius)
            .filter(|&(_, position)| position.sub(b).norm() > radius)
            .filter(|&(_, position)| position.sub(a).dot(position.sub(b)) < 0.0)
            .count() > 0
    }

    fn reflect(&self, ray: Line3d, point: Vector3d) -> Line3d {
        Line3d {
            o: point,
            l: ray.l.reflect(self.normal_at(point)),
        }
    }
}
