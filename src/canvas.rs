use color::Color;
use progress_bar::ProgressBar;
use std::io;
use std::vec::Vec;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize(width * height, color);

        Canvas { width, height, pixels }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn save_pbm(&self, out: &mut io::Write) -> io::Result<()> {
        try!(out.write_fmt(format_args!("P6 {} {} 255\n", self.width, self.height)));

        let mut progress_bar = ProgressBar::new("Saving file", self.width * self.height);
        for color in &self.pixels {
            try!(out.write_all(&[color.r, color.g, color.b]));
            progress_bar.step().print();
        }

        Ok(())
    }
}
