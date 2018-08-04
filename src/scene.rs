use core::cmp::Ordering;
use shape::Shape;
use light::Light;
use canvas::Canvas;
use color::Color;
use camera::Camera;
use geometry::*;
use progress_bar::ProgressBar;

pub struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<Shape>>,
    background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene { lights: Vec::new(), shapes: Vec::new(), background: Color::gray(0) }
    }

    pub fn add_shape<'a>(&'a mut self, shape: Box<Shape>) -> &'a mut Self {
        self.shapes.push(shape);
        self
    }

    pub fn add_light<'a>(&'a mut self, light: Light) -> &'a mut Self {
        self.lights.push(light);
        self
    }

    pub fn render(&self, camera: &Camera) -> Canvas {
        let (width, height) = camera.get_canvas_size();
        let mut canvas = Canvas::new(width, height, self.background.clone());

        let mut progress_bar = ProgressBar::new("Rendering", width * height);
        for (x, y) in iproduct!(0..width, 0..height) {
            let ray = camera.ray(x, y);
            let intersection = self.shapes.iter()
                .flat_map(|shape| shape.intersect(ray).into_iter())
                .filter(|&(_, position)| camera.sees(position))
                .min_by(|a, b| closer_intersection(a, b, ray.o));

            if let Some((shape, point)) = intersection {
                let total_illumination = self.lights.iter()
                    .filter(|light| self.shapes.iter()
                        .filter(|occluding_shape| occluding_shape.uuid() != shape.uuid())
                        .all(|shape| !shape.occludes(light.center, point)))
                    .map(|light| calculate_illumination(point, light, shape, camera.get_center()))
                    .sum();

                if let Some(pixel_color) = canvas.get_mut(x, y) {
                    *pixel_color = shape.color_at(point).dim(total_illumination);
                }
            }
            progress_bar.step().print();
        }

        canvas
    }
}

fn closer_intersection(&(_,a): &(&Shape, Vector3d), &(_,b): &(&Shape, Vector3d), origin: Vector3d) -> Ordering {
    if origin.distance(a) >= origin.distance(b) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
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
