use std::fmt;

#[derive(Clone, Copy)]
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3d { x: x, y: y, z: z }
    }

    pub fn norm(&self) -> f64 {
        ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt()
    }

    pub fn unit(&self) -> Self {
        let normal = self.norm();
        Vector3d { x: self.x/normal, y: self.y/normal, z: self.z/normal }
    }

    pub fn dot(&self, ref other: Vector3d) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, ref other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }

    pub fn scale(&self, scale: f64) -> Self {
        Vector3d { x: self.x*scale, y: self.y*scale, z: self.z * scale }
    }

    pub fn add(&self, ref other: Vector3d) -> Self {
        Vector3d { x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }

    pub fn sub(&self, ref other: Vector3d) -> Self {
        self.add(other.scale(-1.0))
    }

    pub fn distance(&self, ref other: Vector3d) -> f64 {
        self.sub(*other).norm()
    }
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy)]
pub struct Line3d {
    pub l: Vector3d,
    pub o: Vector3d,
}

impl Line3d {
    pub fn new(l: Vector3d, o: Vector3d) -> Line3d {
        Line3d { l: l.unit(), o: o }
    }

    pub fn at(&self, d: f64) -> Vector3d {
        self.o.add(self.l.scale(d))
    }
}

impl fmt::Display for Line3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.o, self.l)
    }
}
