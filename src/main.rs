extern crate core;
extern crate rand;
#[macro_use]
extern crate itertools;
extern crate uuid;

mod camera;
mod canvas;
mod color;
mod geometry;
mod light;
mod progress_bar;
mod scene;
mod shapes;

use camera::Camera;
use geometry::Vector3d;
use light::Light;
use progress_bar::ProgressBar;
use scene::Scene;
use shapes::{CheckerBoard, Sphere};

fn main() {
    let mut scene = Scene::new();
    scene
        .add_light(Light::new(Vector3d::new(-5.0, 5.0, 7.0), 1.0))
        .add_light(Light::new(Vector3d::new(-5.0, -5.0, 3.0), 0.8))
        .add_shape(Box::new(CheckerBoard::new(0.0)));

    const NUMBER_OF_SPHERES: usize = 200;
    let mut progress_bar = ProgressBar::new("Generating scene", NUMBER_OF_SPHERES);
    for _ in 0..NUMBER_OF_SPHERES {
        scene.add_shape(Box::new(Sphere::random()));
        progress_bar.step().print();
    }

    let camera = Camera::new(Vector3d::new(-5.0, 0.0, 2.0), Vector3d::new(1.0, 0.0, 0.0));

    let canvas = scene.render(&camera, 800, 480);

    let mut out_file = std::fs::File::create("out.ppm").expect("Failed to open file");
    canvas.save_pbm(&mut out_file).expect("Failed to write to the file");
}
