use std::fmt;

#[derive(Clone, Copy)]
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3d { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn unit(&self) -> Self {
        self.scale(1.0 / self.norm())
    }

    pub fn dot(&self, other: Vector3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(&self, scale: f64) -> Self {
        Vector3d {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    pub fn add(&self, other: Vector3d) -> Self {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: Vector3d) -> Self {
        self.add(other.scale(-1.0))
    }

    pub fn reflect(&self, normal: Vector3d) -> Vector3d {
        self.sub(normal.scale(2.0 * self.dot(normal)))
    }

    pub fn unit_to(self, point: Vector3d) -> Vector3d {
        point.sub(self).unit()
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
