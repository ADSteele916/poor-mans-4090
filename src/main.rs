mod camera;
mod hittable;
mod hittable_list;
mod random;
mod ray;
mod sphere;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::random::{random_double, random_unit_vector};
use crate::ray::Ray;
use crate::sphere::Sphere;
use clap::Parser;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use nalgebra::{vector, Vector3};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

fn ray_colour(r: &Ray, world: &HittableList, depth: i32) -> Vector3<f64> {
    if depth <= 0 {
        return vector![0.0, 0.0, 0.0];
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.point() + rec.normal() + random_unit_vector();
        return 0.5
            * ray_colour(
                &Ray::new(*rec.point(), target - rec.point()),
                world,
                depth - 1,
            );
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
}

fn vector_to_rgb(colour: &Vector3<f64>, samples: u32) -> [u8; 3] {
    let scale = f64::powi(255.0, 2) / (samples as f64);
    [
        (colour.x * scale).sqrt().round() as u8,
        (colour.y * scale).sqrt().round() as u8,
        (colour.z * scale).sqrt().round() as u8,
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
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(vector![0.0, 0.0, -1.0], 0.5)));
    world.add(Arc::new(Sphere::new(vector![0.0, -100.5, -1.0], 100.0)));

    // Camera
    let cam = Camera::new();

    let buffer: Vec<u8> = (0..image_height)
        .flat_map(|j| (0..image_width).map(move |i| (i, j)))
        .collect::<Vec<(_, _)>>()
        .into_par_iter()
        .progress()
        .flat_map(|(x, y)| {
            let mut pixel_colour = vector![0.0, 0.0, 0.0];
            for _ in 0..samples_per_pixel {
                let u = ((x as f64) + random_double()) / (image_width - 1) as f64;
                let v = ((y as f64) - random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, max_depth);
            }
            vector_to_rgb(&pixel_colour, samples_per_pixel)
        })
        .collect();
    let img = RgbImage::from_raw(image_width, image_height, buffer).unwrap();

    img.save(args.path).unwrap();
}
