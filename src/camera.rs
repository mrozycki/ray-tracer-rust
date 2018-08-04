use geometry::{Line3d, Vector3d};

pub struct Camera {
    center: Vector3d,
    direction: Vector3d,
    up_unit: Vector3d,
    width: f64,
    height: f64,
    canvas_width: usize,
    canvas_height: usize,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            center: Vector3d::new(0.0, 0.0, 0.0),
            direction: Vector3d::new(1.0, 0.0, 0.0),
            up_unit: Vector3d::new(0.0, 0.0, 1.0),
            width: 2.0,
            height: 2.0,
            canvas_width: 800,
            canvas_height: 480,
        }
    }

    pub fn center(&mut self, center: Vector3d) -> &mut Self {
        self.center = center;
        self
    }

    pub fn direction(&mut self, direction: Vector3d) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn canvas_size(&mut self, (canvas_width, canvas_height): (usize, usize)) -> &mut Self {
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        self.height = self.width * (canvas_height as f64) / (canvas_width as f64);
        self
    }

    pub fn ray(&self, x: usize, y: usize) -> Line3d {
        let left_unit = self.up_unit.cross(self.direction).unit();

        let canvas_corner = self.center
            .add(self.direction)
            .add(self.up_unit.scale(self.height / 2.0))
            .add(left_unit.scale(self.width / 2.0));

        let pixel_right = left_unit.scale(-self.width / (self.canvas_width as f64));
        let pixel_down = self.up_unit
            .scale(-self.height / (self.canvas_height as f64));

        let pixel_corner = canvas_corner
            .add(pixel_right.scale(x as f64))
            .add(pixel_down.scale(y as f64));

        let pixel_center = pixel_corner
            .add(pixel_right.scale(0.5))
            .add(pixel_down.scale(0.5));

        Line3d::new(pixel_center.sub(self.center), pixel_center)
    }

    pub fn sees(&self, position: Vector3d) -> bool {
        self.direction.dot(position.sub(self.center)) > 0.0
    }

    pub fn get_canvas_size(&self) -> (usize, usize) {
        (self.canvas_width, self.canvas_height)
    }

    pub fn get_center(&self) -> Vector3d {
        self.center
    }
}
