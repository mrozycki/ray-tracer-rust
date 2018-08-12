use nalgebra::{Point3, Vector3, Unit};

pub trait VectorUtils {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64>;
}

impl VectorUtils for Vector3<f64> {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self - 2.0 * normal * self.dot(&normal)
    }
}

pub trait PointUtils {
    fn unit_to(self, point: &Point3<f64>) -> Unit<Vector3<f64>>;
    fn distance_to(self, point: &Point3<f64>) -> f64;
}

impl PointUtils for Point3<f64> {
    fn unit_to(self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        Unit::new_normalize(point - self)
    }

    fn distance_to(self, point: &Point3<f64>) -> f64 {
        (self - point).norm()
    }
}