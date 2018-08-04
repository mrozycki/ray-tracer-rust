use camera::Camera;
use canvas::Canvas;
use color::Color;
use core::cmp::Ordering;
use geometry::*;
use light::Light;
use progress_bar::ProgressBar;
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

    fn find_intersection(&self, ray : Line3d) -> Option<(&Shape, Vector3d)> {
        self.shapes.iter()
            .flat_map(|shape| shape.intersect(ray).into_iter())
            .filter(|&(_, position)| ray.project(position) > 0.0)
            .min_by(|a, b| closer_intersection(a, b, ray.o))
    }

    fn calculate_illumination(&self, shape: &Shape, point: Vector3d, eye: Vector3d) -> f64 {
        self.lights.iter()
            .filter(|light| {
                self.shapes.iter()
                    .filter(|occluding_shape| occluding_shape.uuid() != shape.uuid())
                    .all(|shape| !shape.occludes(light.center, point))
            })
            .map(|light| calculate_illumination(point, light, shape, eye))
            .sum()
    }

    pub fn render(&self, camera: &Camera, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height, self.background.clone());
        let mut progress_bar = ProgressBar::new("Rendering", width * height);

        for (x, y, ray) in camera.rays(width, height) {
            if let (Some(pixel), Some((shape, point))) = (canvas.get_mut(x, y), self.find_intersection(ray)) {
                *pixel = shape.color_at(point)
                    .dim(self.calculate_illumination(shape, point, camera.center()));
            }
            progress_bar.step().print();
        }

        canvas
    }
}

fn closer_intersection(&(_, a): &(&Shape, Vector3d), &(_, b): &(&Shape, Vector3d), origin: Vector3d) -> Ordering {
    if origin.distance(a) >= origin.distance(b) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn calculate_illumination(position: Vector3d, light: &Light, shape: &Shape, viewer: Vector3d) -> f64 {
    let unit_to_light = light.center.sub(position).unit();
    let normal = shape.normal_at(position);
    let diffuse = normal.dot(unit_to_light) * shape.diffuse_coefficient() * light.intensity;

    let unit_to_viewer = viewer.sub(position).unit();
    let reflection = unit_to_light.reflect(normal);
    let specular = if reflection.dot(unit_to_viewer) > 0.0 {
        0.0
    } else {
        -reflection.dot(unit_to_viewer).powi(5) * shape.specular_coefficient() * light.intensity
    };

    diffuse + specular + shape.ambient_light()
}
