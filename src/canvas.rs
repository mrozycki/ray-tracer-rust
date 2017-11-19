use color::Color;
use std::vec::Vec;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        let mut pixels = Vec::with_capacity(width*height);
        pixels.resize(width*height, color);

        Canvas { width: width, height: height, pixels: pixels }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn print_ppm(self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(color) = self.get(x, y) {
                    print!("{} {} {} ", color.r, color.g, color.b);
                }
            }
            println!();
        }
    }
}
