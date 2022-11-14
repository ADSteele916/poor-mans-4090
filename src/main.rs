mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use clap::Parser;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use nalgebra::{vector, Vector3};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

fn ray_colour(r: &Ray, world: &HittableList) -> Vector3<f64> {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal() + vector![1.0, 1.0, 1.0]);
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
}

fn vector_to_rgb(colour: &Vector3<f64>) -> [u8; 3] {
    [
        (colour.x * 255.0) as u8,
        (colour.y * 255.0) as u8,
        (colour.z * 255.0) as u8,
    ]
}

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(default_value = "output.png")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    // World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(vector![0.0, 0.0, -1.0], 0.5)));
    world.add(Arc::new(Sphere::new(vector![0.0, -100.5, 0.0], 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vector![0.0, 0.0, 0.0];
    let horizontal = vector![viewport_width, 0.0, 0.0];
    let vertical = vector![0.0, viewport_height, 0.0];
    let upper_left_corner =
        origin - horizontal / 2.0 + vertical / 2.0 - vector![0.0, 0.0, focal_length];

    let buffer: Vec<u8> = (0..image_height)
        .flat_map(|j| (0..image_width).map(move |i| (i, j)))
        .collect::<Vec<(_, _)>>()
        .into_par_iter()
        .progress()
        .flat_map(|(x, y)| {
            let u = (x as f64) / (image_width - 1) as f64;
            let v = (y as f64) / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                upper_left_corner + u * horizontal - v * vertical - origin,
            );
            let pixel_colour = ray_colour(&r, &world);
            vector_to_rgb(&pixel_colour)
        })
        .collect();
    let img = RgbImage::from_raw(image_width, image_height, buffer).unwrap();

    img.save(args.path).unwrap();
}
