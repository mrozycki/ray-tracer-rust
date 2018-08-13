use camera::Camera;
use canvas::Canvas;
use color::Color;
use geometry::utils::*;
use geometry::Line3d;
use light::Light;
use nalgebra::Point3;
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

    fn cast(&self, ray: &Line3d, depth: usize) -> Color {
        if depth == 0 {
            return self.scene.background();
        }

        let nearest_intersection = self.nearest_intersection(ray);
        if nearest_intersection.is_none() {
            return self.scene.background();
        }

        let (shape, point) = nearest_intersection.unwrap();
        let color = self.color_at(shape, &point, self.camera.center());

        if depth == 1 || shape.reflectiveness() <= 0.0 {
            return color;
        }

        let reflected_ray = shape.reflect(ray, &point);
        let deep_color = self.cast(&reflected_ray, depth - 1);
        color.blend(&deep_color, 0.5)
    }

    fn nearest_intersection(&self, ray: &Line3d) -> Option<(&Shape, Point3<f64>)> {
        self.scene.shapes().iter()
            .flat_map(|shape| shape.intersect(ray))
            .filter(|&(_, position)| ray.project(position) > 0.0)
            .filter(|&(_, position)| position.distance_to(&ray.origin()) > self.pixel_radius)
            .min_by(|&(_, a), &(_, b)| ray.project(a).partial_cmp(&ray.project(b)).unwrap())
    }

    fn color_at(&self, shape: &Shape, point: &Point3<f64>, eye: &Point3<f64>) -> Color {
        shape
            .color_at(point)
            .dim(self.illumination_at(shape, point, eye))
    }

    fn illumination_at(&self, shape: &Shape, point: &Point3<f64>, eye: &Point3<f64>) -> f64 {
        self.scene.lights().iter()
            .filter(|light| self.path_clear(point, &light.center))
            .map(|light| Self::illumination_from_light(shape, point, light, eye))
            .map(|illumination| illumination.powi(2))
            .sum::<f64>().sqrt()
            + shape.ambient_light()
    }

    fn path_clear(&self, point_on_shape: &Point3<f64>, other_point: &Point3<f64>) -> bool {
        self.scene.shapes().iter()
            .filter(|obstacle| obstacle.occludes(point_on_shape, other_point, self.pixel_radius))
            .peekable().peek().is_none()
    }

    fn illumination_from_light(shape: &Shape, position: &Point3<f64>, light: &Light, eye: &Point3<f64>) -> f64 {
        let normal = shape.normal_at(position);

        let unit_to_light = position.unit_to(&light.center);
        let diffuse = normal.dot(&unit_to_light) * shape.diffuse_coefficient() * light.intensity;

        let unit_to_eye = position.unit_to(eye);
        let reflection = unit_to_light.reflect(&normal);
        let specular = -reflection.dot(&unit_to_eye).min(0.0).powi(5)
            * shape.specular_coefficient()
            * light.intensity;

        diffuse + specular
    }

    pub fn into_canvas(self) -> Canvas {
        let progress_bar = ProgressBar::new("Rendering", self.width * self.height);
        let color_points : Vec<_> = self.camera.rays(self.width, self.height).into_par_iter()
            .inspect(|_| progress_bar.step().print())
            .map(|(x, y, ray)| (x, y, self.cast(&ray, 8)))
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
