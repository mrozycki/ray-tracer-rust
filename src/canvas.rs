use color::Color;
use progress_bar::ProgressBar;
use std::vec::Vec;
use std::io;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize(width*height, color);

        Canvas { width, height, pixels }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn save_ppm(&self, out : &mut io::Write) {
        out.write_all(b"P3\n").expect("Failed");
        out.write_fmt(format_args!("{} {}\n", self.width, self.height)).expect("Failed");
        out.write_all(b"255\n").expect("Failed");

        let mut progress_bar = ProgressBar::new("Saving file", self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(color) = self.get(x, y) {
                    out.write_fmt(format_args!("{} {} {} ", color.r, color.g, color.b)).expect("Failed");
                }
                progress_bar.step().print();
            }
            out.write_all(b"\n").expect("Failed");
        }
    }
}
