#[macro_use]
extern crate clap;
extern crate core;
extern crate rand;
#[macro_use]
extern crate itertools;
extern crate nalgebra;
extern crate png;
extern crate rayon;

mod args;
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

use args::CommandLineArguments;
use camera::Camera;
use nalgebra::Point3;
use render::Render;

fn main() {
    let mut camera = Camera::new(Point3::new(-5.0, 0.0, 2.0));
    camera.rotate(1.0, 0.0, 0.0);

    let scene = scene_generator::spheres_demo(200);
    let args = CommandLineArguments::read();

    let canvas = Render::new(&scene, &camera, args.render_width, args.render_height)
        .into_canvas()
        .downsample(args.antialias_factor);

    canvas.save_png(&args.filename);
}
