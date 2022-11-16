use std::{path::PathBuf, sync::Arc};

use image::io::Reader as ImageReader;
use image::RgbImage;
use nalgebra::{vector, Vector3};

use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64>;
}

pub struct SolidColour {
    colour_value: Vector3<f64>,
}

impl SolidColour {
    pub fn new(colour_value: Vector3<f64>) -> Self {
        Self { colour_value }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        self.colour_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Vector3<f64>, c2: Vector3<f64>) -> Self {
        Self {
            even: Arc::new(SolidColour::new(c1)),
            odd: Arc::new(SolidColour::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        vector![1.0, 1.0, 1.0] * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p)).sin())
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image_path: PathBuf) -> Self {
        Self {
            image: ImageReader::open(image_path)
                .unwrap()
                .decode()
                .unwrap()
                .into_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = ((u * (self.image.width() as f64)) as u32).min(self.image.width() - 1);
        let j = ((v * (self.image.height() as f64)) as u32).min(self.image.height() - 1);

        let colour_scale = 1.0 / 255.0;

        let pixel = self.image.get_pixel(i, j).0;

        colour_scale * vector![pixel[0] as f64, pixel[1] as f64, pixel[2] as f64]
    }
}
