use nalgebra::Point3;

pub struct Light {
    pub center: Point3<f64>,
    pub intensity: f64,
}

impl Light {
    pub fn new(center: Point3<f64>, intensity: f64) -> Self {
        Light { center, intensity }
    }
}
