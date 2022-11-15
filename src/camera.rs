use crate::ray::Ray;
use nalgebra::{vector, Vector3};

pub struct Camera {
    origin: Vector3<f64>,
    upper_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vector![0.0, 0.0, 0.0];
        let horizontal = vector![viewport_width, 0.0, 0.0];
        let vertical = vector![0.0, viewport_height, 0.0];
        let upper_left_corner =
            origin - horizontal / 2.0 + vertical / 2.0 - vector![0.0, 0.0, focal_length];

        Self {
            origin,
            upper_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.upper_left_corner + u * self.horizontal - v * self.vertical - self.origin,
        )
    }
}
