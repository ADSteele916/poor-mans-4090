mod aabb;
mod aabox;
mod aarect;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod random;
mod ray;
mod scenes;
mod sphere;
mod texture;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::random::random_double;
use crate::ray::Ray;
use crate::scenes::random_scene;
use clap::Parser;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use nalgebra::{vector, Vector3};
use rayon::prelude::*;
use scenes::{cornell_box, earth, simple_light, two_perlin_spheres, two_spheres};
use std::path::PathBuf;

fn ray_colour(
    r: &Ray,
    background: &Vector3<f64>,
    world: &HittableList,
    depth: i32,
) -> Vector3<f64> {
    if depth <= 0 {
        return vector![0.0, 0.0, 0.0];
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.material().emitted(rec.u(), rec.v(), &rec.point());
        if let Some((attenuation, scatttered)) = rec.material().scatter(r, &rec) {
            emitted
                + attenuation.component_mul(&ray_colour(&scatttered, background, world, depth - 1))
        } else {
            emitted
        }
    } else {
        *background
    }
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
    #[arg(default_value_t = 0)]
    scene: usize,
    #[arg(default_value = "output.png")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Image

    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 100;

    // World

    let world;

    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.0;
    let background;

    match args.scene {
        1 => {
            world = random_scene();
            background = vector![0.70, 0.80, 1.00];
            lookfrom = vector![13.0, 2.0, 3.0];
            lookat = vector![0.0, 0.0, 0.0];
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = vector![0.70, 0.80, 1.00];
            lookfrom = vector![13.0, 2.0, 3.0];
            lookat = vector![0.0, 0.0, 0.0];
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = vector![0.70, 0.80, 1.00];
            lookfrom = vector![13.0, 2.0, 3.0];
            lookat = vector![0.0, 0.0, 0.0];
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = vector![0.70, 0.80, 1.00];
            lookfrom = vector![13.0, 2.0, 3.0];
            lookat = vector![0.0, 0.0, 0.0];
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = vector![0.0, 0.0, 0.0];
            lookfrom = vector![26.0, 3.0, 6.0];
            lookat = vector![0.0, 2.0, 0.0];
            vfov = 20.0;
        }
        _ => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = vector![0.0, 0.0, 0.0];
            lookfrom = vector![278.0, 278.0, -800.0];
            lookat = vector![278.0, 278.0, 0.0];
            vfov = 40.0;
        }
    }

    // Camera

    let vup = vector![0.0, 1.0, 0.0];
    let dist_to_focus = 10.0;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render

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
                pixel_colour += ray_colour(&r, &background, &world, max_depth);
            }
            vector_to_rgb(&pixel_colour, samples_per_pixel)
        })
        .collect();
    let img = RgbImage::from_raw(image_width, image_height, buffer).unwrap();

    img.save(args.path).unwrap();
}
