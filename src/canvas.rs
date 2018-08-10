use color::Color;
use png::{BitDepth, ColorType, Encoder, HasParameters};
use progress_bar::ProgressBar;
use std::fs;
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

    pub fn save_png(&self, filename: &str) {
        let file = fs::File::create(filename).expect("Could not open file");
        let ref mut w = io::BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);

        encoder.set(ColorType::RGB).set(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let progress_bar = ProgressBar::new("Saving file", self.width * self.height);
        let data: Vec<u8> = self.pixels.iter()
            .inspect(|_| progress_bar.step().print())
            .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b])
            .collect();

        writer.write_image_data(data.as_slice()).unwrap();
    }
}
