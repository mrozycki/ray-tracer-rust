use crate::color::Color;
use crate::light::Light;
use crate::progress_bar::ProgressBar;
use crate::scene::Scene;
use crate::shapes::{CheckerBoard, Sphere};

use nalgebra::Point3;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

pub fn random_sphere() -> Sphere {
    let mut rng = thread_rng();
    let radius_range = Uniform::new(0.1, 0.5);
    let radius = radius_range.sample(&mut rng);
    let position_range_x = Uniform::new(-30.0, 30.0);
    let position_range_y = Uniform::new(-30.0, 30.0);
    let (x, y) = (
        position_range_x.sample(&mut rng),
        position_range_y.sample(&mut rng),
    );

    Sphere::new(Point3::new(x, y, radius), radius, Color::random())
}

pub fn spheres_demo(number_of_spheres: usize) -> Scene {
    let mut scene = Scene::new(Color::gray(0));
    scene
        .add_light(Light::new(Point3::new(-5.0, 5.0, 7.0), 1.0))
        .add_light(Light::new(Point3::new(-5.0, -5.0, 3.0), 0.8))
        .add_shape(Box::new(CheckerBoard::new(0.0)));

    let progress_bar = ProgressBar::new("Generating scene", number_of_spheres);
    let mut spheres = Vec::new();
    while spheres.len() < number_of_spheres {
        let new_sphere = random_sphere();
        if spheres
            .iter()
            .any(|sphere| new_sphere.collides_with(sphere))
        {
            continue;
        }
        spheres.push(new_sphere);
        progress_bar.step().print();
    }

    for sphere in spheres {
        scene.add_shape(Box::new(sphere));
    }

    scene
}
