use geometry::Vector3d;

#[derive(Clone, Copy)]
pub struct Line3d {
    origin: Vector3d,
    direction: Vector3d,
}

impl Line3d {
    pub fn new(origin: Vector3d, direction: Vector3d) -> Line3d {
        Line3d { origin, direction: direction.unit() }
    }

    pub fn between(a: Vector3d, b: Vector3d) -> Line3d {
        Line3d { origin: a, direction: a.unit_to(b) }
    }

    pub fn at(&self, d: f64) -> Vector3d {
        self.origin.add(self.direction.scale(d))
    }

    pub fn project(&self, v: Vector3d) -> f64 {
        v.sub(self.origin).dot(self.direction)
    }

    pub fn origin(&self) -> Vector3d {
        self.origin
    }

    pub fn direction(&self) -> Vector3d {
        self.direction
    }
}
