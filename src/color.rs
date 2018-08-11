use rand::Rng;
use std::fmt;
extern crate rand;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn gray(brightness: u8) -> Color {
        Color {
            r: brightness,
            g: brightness,
            b: brightness,
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<u8>(),
            g: rng.gen::<u8>(),
            b: rng.gen::<u8>(),
        }
    }

    pub fn dim(&self, illumination: f64) -> Self {
        if illumination <= 0.0 {
            Color::gray(0)
        } else if illumination >= 1.0 {
            self.clone()
        } else {
            Color::rgb(
                (f64::from(self.r) * illumination) as u8,
                (f64::from(self.g) * illumination) as u8,
                (f64::from(self.b) * illumination) as u8,
            )
        }
    }

    fn blend_component(a: u8, b: u8, ratio: f64) -> u8 {
        ((a as f64).powi(2) + (b as f64).powi(2)*ratio).sqrt().min(255.0) as u8
    }

    pub fn blend(&self, other: &Color, ratio: f64) -> Color {
        let ratio = ratio.max(0.0).min(1.0);
        Color {
            r: Self::blend_component(self.r, other.r, ratio),
            g: Self::blend_component(self.g, other.g, ratio),
            b: Self::blend_component(self.b, other.b, ratio),
        }
    }
}
