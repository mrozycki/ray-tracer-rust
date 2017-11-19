use std::fmt;

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

    pub fn dim(&self, illumination: f64) -> Self {
        if illumination <= 0.0 {
            return Color::gray(0);
        }

        Color {
            r: ((self.r as f64)*illumination) as u8,
            g: ((self.g as f64)*illumination) as u8,
            b: ((self.b as f64)*illumination) as u8
        }
    }
}
