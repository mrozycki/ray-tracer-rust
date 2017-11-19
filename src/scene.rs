use shape::Shape;
use light::Light;

struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<Shape>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { lights: Vec::new(), shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<Shape>) {
        self.shapes.push(shape);
    }

    pub fn get_shapes(&self) -> &Vec<Box<Shape>> {
        &self.shapes
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }
}
