extern crate core;
#[macro_use] extern crate itertools;

mod camera;
mod canvas;
mod color;
mod geometry;
mod light;
mod scene;
mod shape;
mod sphere;

use camera::Camera;
use color::Color;
use light::Light;
use scene::Scene;
use sphere::Sphere;
use geometry::Vector3d;


fn main() {
    let canvas_size = (500, 500);
    let mut scene = Scene::new();
    scene.add_light(Light::new(Vector3d::new(5.0, 5.0, 5.0), 1.0))
        .add_shape(Box::new(Sphere::new(Color::rgb(255, 0, 0))));

    let mut camera = Camera::new();
    camera.center(Vector3d::new(-2.0, 0.0, 0.0))
        .direction(Vector3d::new(1.0, 0.0, 0.0))
        .up(Vector3d::new(0.0, 0.0, 1.0))
        .canvas_size(canvas_size);

    let canvas = scene.render(&camera);

    canvas.print_ppm();
}
