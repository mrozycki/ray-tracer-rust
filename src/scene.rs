use camera::Camera;
use canvas::Canvas;
use color::Color;
use geometry::*;
use light::Light;
use progress_bar::ProgressBar;
use rayon::prelude::*;
use shapes::Shape;
use std::cmp;

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

    fn cast(&self, ray: Line3d) -> Option<(&Shape, Vector3d)> {
        self.shapes.iter()
            .flat_map(|shape| shape.intersect(ray))
            .filter(|&(_, position)| ray.project(position) > 0.0)
            .min_by(|&(_, a), &(_, b)| ray.project(a).partial_cmp(&ray.project(b)).unwrap())
    }

    fn path_clear(&self, point_on_shape: Vector3d, radius: f64, other_point: Vector3d) -> bool {
        self.shapes.iter()
            .filter(|obstacle| obstacle.occludes(point_on_shape, other_point, radius))
            .peekable().peek().is_none()
    }

    fn illumination_at(&self, shape: &Shape, point: Vector3d, radius: f64, eye: Vector3d) -> f64 {
        self.lights.iter()
            .filter(|light| self.path_clear(point, radius, light.center))
            .map(|light| Self::illumination_from_light(shape, point, light, eye))
            .map(|illumination| illumination.powi(2))
            .sum::<f64>().sqrt()
    }

    fn illumination_from_light(shape: &Shape, position: Vector3d, light: &Light, eye: Vector3d) -> f64 {
        let normal = shape.normal_at(position);

        let unit_to_light = position.unit_to(light.center);
        let diffuse = normal.dot(unit_to_light) * shape.diffuse_coefficient() * light.intensity;

        let unit_to_eye = position.unit_to(eye);
        let reflection = unit_to_light.reflect(normal);
        let specular = -reflection.dot(unit_to_eye).min(0.0).powi(5)
            * shape.specular_coefficient()
            * light.intensity;

        diffuse + specular + shape.ambient_light()
    }

    fn shape_color_at(&self, shape: &Shape, point: Vector3d, radius: f64, eye: Vector3d) -> Color {
        shape
            .color_at(point)
            .dim(self.illumination_at(shape, point, radius, eye))
    }

    pub fn render(&self, camera: &Camera, width: usize, height: usize) -> Canvas {
        let progress_bar = ProgressBar::new("Rendering", width * height);
        let radius = 1.0 / (cmp::max(width, height) as f64);
        let color_points : Vec<_> = camera.rays(width, height).into_par_iter()
            .inspect(|_| progress_bar.step().print())
            .flat_map(|(x, y, ray)| self.cast(ray).map(|intersection| (x, y, intersection)))
            .map(|(x, y, (shape, point))| (x, y, self.shape_color_at(shape, point, radius, camera.center())))
            .collect();

        let mut canvas = Canvas::new(width, height, self.background.clone());
        for (x, y, color) in &color_points {
            if let Some(pixel) = canvas.get_mut(*x, *y) {
                *pixel = color.clone();
            }
        }

        canvas
    }
}
