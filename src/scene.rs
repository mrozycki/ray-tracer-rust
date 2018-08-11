use color::Color;
use light::Light;
use shapes::Shape;

pub struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<Shape>>,
    background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            shapes: Vec::new(),
            background: Color::gray(0),
        }
    }

    pub fn add_shape(&mut self, shape: Box<Shape>) -> &mut Self {
        self.shapes.push(shape);
        self
    }

    pub fn add_light(&mut self, light: Light) -> &mut Self {
        self.lights.push(light);
        self
    }

    pub fn shapes(&self) -> &Vec<Box<Shape>> {
        &self.shapes
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn background(&self) -> Color {
        self.background.clone()
    }
}
