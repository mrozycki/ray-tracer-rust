use std::fs;
use std::io;
use std::vec::Vec;

use crate::color::Color;
use crate::progress_bar::ProgressBar;

use png::{BitDepth, ColorType, Encoder};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize(width * height, color);

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn save_png(&self, filename: &str) {
        let file = fs::File::create(filename).expect("Could not open file");
        let ref mut w = io::BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);

        encoder.set_color(ColorType::RGB);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let progress_bar = ProgressBar::new("Saving file", self.width * self.height);
        let data: Vec<u8> = self
            .pixels
            .iter()
            .inspect(|_| progress_bar.step().print())
            .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b])
            .collect();

        writer.write_image_data(data.as_slice()).unwrap();
    }

    pub fn downsample(self, factor: usize) -> Self {
        let new_width = self.width / factor;
        let new_height = self.height / factor;
        let mut downsampled = Self::new(new_width, new_height, Color::gray(0));

        for (out_x, out_y) in iproduct!(0..new_width, 0..new_height) {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;

            for (in_x, in_y) in iproduct!(0..factor, 0..factor) {
                if let Some(color) = self.get(out_x * factor + in_x, out_y * factor + in_y) {
                    r += (color.r as f64).powi(2);
                    g += (color.g as f64).powi(2);
                    b += (color.b as f64).powi(2);
                }
            }

            if let Some(pixel) = downsampled.get_mut(out_x, out_y) {
                *pixel = Color {
                    r: (r / (factor as f64).powi(2)).sqrt() as u8,
                    g: (g / (factor as f64).powi(2)).sqrt() as u8,
                    b: (b / (factor as f64).powi(2)).sqrt() as u8,
                };
            }
        }

        downsampled
    }
}
