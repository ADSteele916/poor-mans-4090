use clap::Parser;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use nalgebra::Vector3;
use rayon::prelude::*;
use std::path::PathBuf;

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

    let image_width: u32 = 255;
    let image_height: u32 = 255;

    let buffer: Vec<u8> = (0..image_height)
        .flat_map(|j| (0..image_width).map(move |i| (i, j)))
        .collect::<Vec<(_, _)>>()
        .into_par_iter()
        .progress()
        .flat_map(|(x, y)| {
            let r = (x as f64) / (image_width - 1) as f64;
            let g = (image_height - y) as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let colour = Vector3::new(r, g, b);

            vector_to_rgb(&colour)
        })
        .collect();
    let img = RgbImage::from_raw(image_width, image_height, buffer).unwrap();

    img.save(args.path).unwrap();
}
