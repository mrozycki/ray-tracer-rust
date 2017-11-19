use geometry::{Vector3d, Line3d};

pub struct Camera {
    pub center: Vector3d,
    pub direction: Vector3d,
    pub up: Vector3d,
    pub width: f64,
    pub height: f64,
    pub canvas_width: usize,
    pub canvas_height: usize,
}

impl Camera {
    pub fn ray(&self, x: usize, y: usize) -> Line3d {
        let up_unit = self.up.unit();
        let left_unit = self.up.cross(self.direction).unit();

        let canvas_corner = self.center
            .add(self.direction)
            .add(up_unit.scale(self.height/2.0))
            .add(left_unit.scale(self.width/2.0));

        let pixel_right = left_unit.scale(-self.width/(self.canvas_width as f64));
        let pixel_down = up_unit.scale(-self.height/(self.canvas_height as f64));

        let pixel_corner = canvas_corner
            .add(pixel_right.scale(x as f64))
            .add(pixel_down.scale(y as f64));

        let pixel_center = pixel_corner.add(pixel_right.scale(0.5)).add(pixel_down.scale(0.5));

        Line3d::new(pixel_center.sub(self.center), pixel_center)
    }
}
