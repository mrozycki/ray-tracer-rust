use geometry::{Line3d, utils::*};
use nalgebra::{Point3, Vector3, Unit};

pub struct Camera {
    center: Point3<f64>,
    direction: Unit<Vector3<f64>>,
    up_unit: Unit<Vector3<f64>>,
}

impl Camera {
    pub fn new(center: Point3<f64>, direction: Vector3<f64>) -> Camera {
        Camera {
            center,
            direction: Unit::new_normalize(direction),
            up_unit: Vector3::z_axis(),
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.center
    }

    pub fn rays(&self, width: usize, height: usize) -> Vec<(usize, usize, Line3d)> {
        let left_unit = self.up_unit.cross(&self.direction);
        let aspect_ratio = (width as f64) / (height as f64);

        let canvas_corner = self.center
            + self.direction.unwrap()
            + 0.5 * self.up_unit.unwrap()
            + (aspect_ratio / 2.0) * left_unit;
        let pixel_right = left_unit * (-aspect_ratio / (width as f64));
        let pixel_down = self.up_unit.unwrap() * (-1.0 / (height as f64));

        let mut rays = Vec::new();
        for (x, y) in iproduct!(0..width, 0..height) {
            let pixel_center = canvas_corner
                + pixel_right * (x as f64 + 0.5)
                + pixel_down * (y as f64 + 0.5);

            rays.push((x, y, Line3d::new(pixel_center, self.center.unit_to(&pixel_center))));
        }

        rays
    }
}
