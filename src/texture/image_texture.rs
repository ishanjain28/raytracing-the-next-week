use image::{error::ImageError, io::Reader as ImageReader};

use crate::{types::Vec3, Texture};

#[derive(Clone)]
pub struct ImageTexture {
    image: Vec<u8>,
    // (width, height)
    dimensions: (u32, u32),
    bytes_per_scanline: u32,
    bytes_per_pixel: u32,
}

impl ImageTexture {
    #[allow(dead_code)]
    pub fn from_filename(filename: &str) -> Result<Self, ImageError> {
        let img = ImageReader::open(filename)?.decode()?;
        let img = img.to_rgb8();

        let (width, _) = img.dimensions();

        let bytes_per_pixel = 3;

        Ok(Self {
            image: img.to_vec(),
            dimensions: img.dimensions(),
            bytes_per_scanline: bytes_per_pixel * width,
            bytes_per_pixel,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        let (width, height) = self.dimensions;

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * width as f64) as u32;
        let j = (v * height as f64) as u32;

        let i = i.clamp(0, width - 1);
        let j = j.clamp(0, height - 1);

        let color_scale = 1.0 / 255.0;

        let pixel = (j * self.bytes_per_scanline + i * self.bytes_per_pixel) as usize;

        Vec3::new(
            color_scale * (self.image[pixel] as f64),
            color_scale * (self.image[pixel + 1] as f64),
            color_scale * (self.image[pixel + 2] as f64),
        )
    }
}
