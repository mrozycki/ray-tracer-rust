use geometry::{Line3d, Vector3d};

pub struct Camera {
    center: Vector3d,
    direction: Vector3d,
    up_unit: Vector3d,
}

impl Camera {
    pub fn new(center: Vector3d, direction: Vector3d) -> Camera {
        Camera {
            center,
            direction,
            up_unit: Vector3d::new(0.0, 0.0, 1.0),
        }
    }

    pub fn center(&self) -> Vector3d {
        self.center
    }

    pub fn rays(&self, width: usize, height: usize) -> Vec<(usize, usize, Line3d)> {
        let left_unit = self.up_unit.cross(self.direction).unit();
        let aspect_ratio = (width as f64) / (height as f64);

        let canvas_corner = self.center
            .add(self.direction)
            .add(self.up_unit.scale(0.5))
            .add(left_unit.scale(aspect_ratio / 2.0));

        let pixel_right = left_unit.scale(-aspect_ratio / (width as f64));
        let pixel_down = self.up_unit.scale(-1.0 / (height as f64));

        let mut rays = Vec::new();
        for (x, y) in iproduct!(0..width, 0..height) {
            let pixel_center = canvas_corner
                .add(pixel_right.scale(x as f64 + 0.5))
                .add(pixel_down.scale(y as f64 + 0.5));

            rays.push((x, y, Line3d::new(pixel_center, self.center.unit_to(pixel_center))));
        }

        rays
    }
}
