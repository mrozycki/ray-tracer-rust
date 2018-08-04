use geometry::Vector3d;

pub struct Light {
    pub center: Vector3d,
    pub intensity: f64,
}

impl Light {
    pub fn new(center: Vector3d, intensity: f64) -> Self {
        Light { center, intensity }
    }
}
