use color::Color;
use light::Light;
use shapes::Shape;

pub struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<dyn Shape>>,
    background: Color,
}

impl Scene {
    pub fn new(background: Color) -> Self {
        Scene {
            lights: Vec::new(),
            shapes: Vec::new(),
            background,
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) -> &mut Self {
        self.shapes.push(shape);
        self
    }

    pub fn add_light(&mut self, light: Light) -> &mut Self {
        self.lights.push(light);
        self
    }

    pub fn shapes(&self) -> &Vec<Box<dyn Shape>> {
        &self.shapes
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn background(&self) -> Color {
        self.background.clone()
    }
}
