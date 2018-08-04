use geometry::Vector3d;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Line3d {
    pub l: Vector3d,
    pub o: Vector3d,
}

impl Line3d {
    pub fn new(l: Vector3d, o: Vector3d) -> Line3d {
        Line3d { l: l.unit(), o }
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
