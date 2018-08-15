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
mod scene_generator;
mod shapes;

use camera::Camera;
use clap::App;
use nalgebra::Point3;
use render::Render;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let scene = scene_generator::spheres_demo(200);

    let mut camera = Camera::new(Point3::new(-5.0, 0.0, 2.0));
    camera.rotate(1.0, 0.0, 0.0);

    let antialias = matches.value_of("antialias")
        .and_then(|a| a.parse::<usize>().ok())
        .unwrap_or(1);
    let canvas_width = matches.value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or(800);
    let canvas_height = matches.value_of("height")
        .and_then(|h| h.parse::<usize>().ok())
        .unwrap_or(canvas_width);
    let filename = matches.value_of("output").unwrap_or("out.png");

    let canvas = Render::new(&scene, &camera, canvas_width * antialias, canvas_height * antialias)
        .into_canvas()
        .downsample(antialias);
    canvas.save_png(filename);
}
