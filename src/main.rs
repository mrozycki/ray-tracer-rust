extern crate core;
mod scene;
mod color;
mod canvas;
mod sphere;
mod geometry;
mod camera;
mod light;
mod shape;
use core::cmp::Ordering;
use geometry::*;
use sphere::*;
use light::*;
use shape::*;

fn compare_intersections(ref a: geometry::Vector3d, ref b: geometry::Vector3d, ref origin: geometry::Vector3d) -> Ordering {
    if origin.distance(*a) <= origin.distance(*b) {
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
        let reflection = direction.sub(normal.scale(2.0*direction.dot(normal))).unit();
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

fn main() {
    let width = 500;
    let height = 500;

    let mut canvas = canvas::Canvas::new(width, height, color::Color{ r: 0, g: 0, b: 128 });
    let mut sphere : &Shape = &sphere::Sphere::new(color::Color{ r:255, g: 0, b: 0 });
    let camera = camera::Camera {
        center: geometry::Vector3d::new(-2.0, 0.0, 0.0),
        direction: geometry::Vector3d::new(1.0, 0.0, 0.0),
        up: geometry::Vector3d::new(0.0, 0.0, 1.0),
        width: 2.0,
        height: 2.0,
        canvas_width: width,
        canvas_height: height,
    };
    let light = light::Light::new(geometry::Vector3d::new(5.0, 5.0, 5.0), 1.0);
    let mut lights = Vec::new();
    lights.push(light);

    for x in 0..width {
        for y in 0..height {
            let ray = camera.ray(x, y);
            let mut intersections = sphere.intersect(ray);
            intersections.sort_by(|a, b| compare_intersections(*a, *b, ray.o));

            if let Some(&point) = intersections.get(0) {
                let normal = sphere.normal_at(point);
                let ambient = sphere.ambient_light();
                let diffuse = calculate_diffuse(point, normal, &lights, sphere.diffuse_coefficient());
                let specular = calculate_specular(point, normal, &lights, ray.o, sphere.specular_coefficient());
                let total_illumination = calculate_total_illumination(ambient, diffuse, specular);

                canvas.set(x, y, sphere.color_at(point).dim(total_illumination));
            }
        }
    }

    canvas.print_ppm();
}
