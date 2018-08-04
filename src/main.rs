extern crate core;
extern crate rand;
#[macro_use] extern crate itertools;
extern crate uuid;

mod camera;
mod canvas;
mod checkerboard;
mod color;
mod geometry;
mod light;
mod scene;
mod shape;
mod sphere;

use camera::Camera;
use color::Color;
use checkerboard::CheckerBoard;
use light::Light;
use scene::Scene;
use sphere::Sphere;
use geometry::Vector3d;


fn main() {
    let canvas_size = (800, 480);
    let mut scene = Scene::new();
    scene
        .add_light(Light::new(Vector3d::new(-5.0, 5.0, 7.0), 1.0))
        .add_light(Light::new(Vector3d::new(-5.0, -5.0, 3.0), 0.8))
        .add_shape(Box::new(CheckerBoard::new(0.0)));

    for _ in 0..200 {
        scene.add_shape(Box::new(Sphere::random()));
    }

    let mut camera = Camera::new();
    camera.center(Vector3d::new(-5.0, 0.0, 2.0))
        .direction(Vector3d::new(1.0, 0.0, 0.0))
        .canvas_size(canvas_size);

    let canvas = scene.render(&camera);

    let mut out_file = std::fs::File::create("out.ppm")
        .expect("Failed to open file");
    canvas.save_ppm(&mut out_file);
}
