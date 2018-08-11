use camera::Camera;
use canvas::Canvas;
use color::Color;
use geometry::*;
use light::Light;
use progress_bar::ProgressBar;
use rayon::prelude::*;
use scene::Scene;
use shapes::Shape;
use std::cmp;

pub struct Render<'a> {
    scene: &'a Scene,
    camera: &'a Camera,
    width: usize,
    height: usize,
    pixel_radius: f64,
}

impl<'a> Render<'a> {
    pub fn new(scene: &'a Scene, camera: &'a Camera, width: usize, height: usize) -> Self {
        Render { 
            scene,
            camera, 
            width, 
            height,
            pixel_radius: 1.0 / (cmp::max(width, height) as f64),
        }
    }

    fn cast(&self, ray: Line3d) -> Color {
        self.nearest_intersection(ray).into_iter()
            .map(|(shape, point)| self.color_at(shape, point))
            .next()
            .unwrap_or_else(|| self.scene.background())
    }

    fn nearest_intersection(&self, ray: Line3d) -> Option<(&Shape, Vector3d)> {
        self.scene.shapes().iter()
            .flat_map(|shape| shape.intersect(ray))
            .filter(|&(_, position)| ray.project(position) > 0.0)
            .min_by(|&(_, a), &(_, b)| ray.project(a).partial_cmp(&ray.project(b)).unwrap())
    }

    fn color_at(&self, shape: &Shape, point: Vector3d) -> Color {
        shape
            .color_at(point)
            .dim(self.illumination_at(shape, point))
    }

    fn illumination_at(&self, shape: &Shape, point: Vector3d) -> f64 {
        self.scene.lights().iter()
            .filter(|light| self.path_clear(point, light.center))
            .map(|light| self.illumination_from_light(shape, point, light))
            .map(|illumination| illumination.powi(2))
            .sum::<f64>().sqrt()
    }

    fn path_clear(&self, point_on_shape: Vector3d, other_point: Vector3d) -> bool {
        self.scene.shapes().iter()
            .filter(|obstacle| obstacle.occludes(point_on_shape, other_point, self.pixel_radius))
            .peekable().peek().is_none()
    }

    fn illumination_from_light(&self, shape: &Shape, position: Vector3d, light: &Light) -> f64 {
        let normal = shape.normal_at(position);

        let unit_to_light = position.unit_to(light.center);
        let diffuse = normal.dot(unit_to_light) * shape.diffuse_coefficient() * light.intensity;

        let unit_to_camera = position.unit_to(self.camera.center());
        let reflection = unit_to_light.reflect(normal);
        let specular = -reflection.dot(unit_to_camera).min(0.0).powi(5)
            * shape.specular_coefficient()
            * light.intensity;

        diffuse + specular + shape.ambient_light()
    }

    pub fn into_canvas(self) -> Canvas {
        let progress_bar = ProgressBar::new("Rendering", self.width * self.height);
        let color_points : Vec<_> = self.camera.rays(self.width, self.height).into_par_iter()
            .inspect(|_| progress_bar.step().print())
            .map(|(x, y, ray)| (x, y, self.cast(ray)))
            .collect();

        let mut canvas = Canvas::new(self.width, self.height, self.scene.background());
        for (x, y, color) in &color_points {
            if let Some(pixel) = canvas.get_mut(*x, *y) {
                *pixel = color.clone();
            }
        }

        canvas
    }
}
