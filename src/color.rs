use std::fmt;
use rand::Rng;
extern crate rand;


#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn gray(brightness: u8) -> Color {
        Color { r: brightness, g: brightness, b: brightness }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color { r: rng.gen::<u8>(), g: rng.gen::<u8>(), b: rng.gen::<u8>() }
    }

    pub fn dim(&self, illumination: f64) -> Self {
        if illumination <= 0.0 {
            Color::gray(0)
        } else if illumination >= 1.0 {
            self.clone()
        } else {
            Color::rgb(
                ((self.r as f64)*illumination) as u8,
                ((self.g as f64)*illumination) as u8,
                ((self.b as f64)*illumination) as u8
            )
        }
    }
}
