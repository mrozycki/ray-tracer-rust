#[macro_use]
extern crate clap;
extern crate core;
extern crate rand;
#[macro_use]
extern crate itertools;
extern crate nalgebra;
extern crate png;
extern crate rayon;

mod camera;
mod canvas;
mod color;
mod geometry;
mod light;
mod progress_bar;
mod render;
mod scene;
mod shapes;

use camera::Camera;
use clap::App;
use light::Light;
use nalgebra::{Point3, Vector3};
use progress_bar::ProgressBar;
use render::Render;
use scene::Scene;
use shapes::{CheckerBoard, Sphere};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut scene = Scene::new();
    scene
        .add_light(Light::new(Point3::new(-5.0, 5.0, 7.0), 1.0))
        .add_light(Light::new(Point3::new(-5.0, -5.0, 3.0), 0.8))
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

    let camera = Camera::new(Point3::new(-5.0, 0.0, 2.0), Vector3::new(1.0, 0.0, 0.0));

    let canvas_width = matches.value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or(800);
    let canvas_height = matches.value_of("height")
        .and_then(|h| h.parse::<usize>().ok())
        .unwrap_or(canvas_width);
    let canvas = Render::new(&scene, &camera, canvas_width, canvas_height).into_canvas();

    let filename = matches.value_of("output").unwrap_or("out.png");
    canvas.save_png(filename);
}
