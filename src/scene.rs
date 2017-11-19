use core::cmp::Ordering;
use shape::Shape;
use light::Light;
use canvas::Canvas;
use color::Color;
use camera::Camera;
use geometry::*;

pub struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<Shape>>,
    background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene { lights: Vec::new(), shapes: Vec::new(), background: Color::gray(128) }
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

        for (x, y) in iproduct!(0..width, 0..height) {
            let ray = camera.ray(x, y);
            let intersection = self.shapes.iter()
                .flat_map(|shape| shape.intersect(ray).into_iter())
                .min_by(|a, b| closer_intersection(a, b, ray.o));

            if let Some((ref shape, point)) = intersection {
                let normal = shape.normal_at(point);
                let ambient = shape.ambient_light();
                let diffuse = calculate_diffuse(point, normal, &self.lights, shape.diffuse_coefficient());
                let specular = calculate_specular(point, normal, &self.lights, ray.o, shape.specular_coefficient());
                let total_illumination = calculate_total_illumination(ambient, diffuse, specular);

                if let Some(pixel_color) = canvas.get_mut(x, y) {
                    *pixel_color = shape.color_at(point).dim(total_illumination);
                }
            }
        }

        canvas
    }
}

fn closer_intersection(&(_,a): &(&Shape, Vector3d), &(_,b): &(&Shape, Vector3d), origin: Vector3d) -> Ordering {
    if origin.distance(a) <= origin.distance(b) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn calculate_diffuse(position: Vector3d, normal: Vector3d, lights: &Vec<Light>, diffuse_coeff: f64) -> f64{
    lights.iter().map(|light: &Light| -> f64 {
        let light_normal = light.center.sub(position).unit();
        let diffuse = normal.dot(light_normal) * diffuse_coeff * light.intensity;
        diffuse
    }).fold(0.0, |p, a| p+a)
}

fn calculate_specular(position: Vector3d, normal: Vector3d, lights: &Vec<Light>, viewer: Vector3d, spectral_coeff: f64) -> f64 {
    lights.iter().map(|light: &Light| -> f64 {
        let unit_to_viewer = viewer.sub(position).unit();
        let direction = light.center.sub(position).unit();
        let reflection = direction.reflect(normal);
        let specular = reflection.dot(unit_to_viewer).powi(10) * spectral_coeff * light.intensity;

        if normal.dot(direction) < 0.0 { 0.0 } else { specular }
    }).fold(0.0, |p, a| p+a)
}

fn calculate_total_illumination(ambient: f64, diffuse: f64, spectral: f64) -> f64 {
    let raw = ambient + diffuse + spectral;
    if raw <= 0.0 {
        return 0.0;
    } else if raw >= 1.0 {
        return 1.0;
    }

    raw
}
