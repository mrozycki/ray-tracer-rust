#[macro_use]
extern crate clap;
extern crate core;
extern crate rand;
#[macro_use]
extern crate itertools;
extern crate png;
extern crate rayon;

mod camera;
mod canvas;
mod color;
mod geometry;
mod light;
mod progress_bar;
mod scene;
mod shapes;

use camera::Camera;
use clap::App;
use geometry::Vector3d;
use light::Light;
use progress_bar::ProgressBar;
use scene::Scene;
use shapes::{CheckerBoard, Sphere};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut scene = Scene::new();
    scene
        .add_light(Light::new(Vector3d::new(-5.0, 5.0, 7.0), 1.0))
        .add_light(Light::new(Vector3d::new(-5.0, -5.0, 3.0), 0.8))
        .add_shape(Box::new(CheckerBoard::new(0.0)));

    const NUMBER_OF_SPHERES: usize = 200;
    let progress_bar = ProgressBar::new("Generating scene", NUMBER_OF_SPHERES);
    let mut spheres = Vec::new();
    while spheres.len() < NUMBER_OF_SPHERES {
        let new_sphere = Sphere::random();
        if spheres.iter().any(|sphere| new_sphere.collides_with(sphere)) {
            continue;
        }
        spheres.push(new_sphere);
        progress_bar.step().print();
    }

    for sphere in spheres {
        scene.add_shape(Box::new(sphere));
    }

    let camera = Camera::new(Vector3d::new(-5.0, 0.0, 2.0), Vector3d::new(1.0, 0.0, 0.0));

    let canvas_width = matches.value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or(800);
    let canvas_height = matches.value_of("height")
        .and_then(|h| h.parse::<usize>().ok())
        .unwrap_or(canvas_width);
    let canvas = scene.render(&camera, canvas_width, canvas_height);

    let filename = matches.value_of("output").unwrap_or("out.png");
    canvas.save_png(filename);
}
