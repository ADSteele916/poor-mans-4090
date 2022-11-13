use clap::Parser;
use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use std::path::PathBuf;

fn vector_to_rgb(colour: &Vector3<f64>) -> Rgb<u8> {
    Rgb([
        (colour.x * 255.0) as u8,
        (colour.y * 255.0) as u8,
        (colour.z * 255.0) as u8,
    ])
}

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(default_value = "output.png")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let image_width = 255;
    let image_height = 255;

    let mut img = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (image_height - j) as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let colour = Vector3::new(r, g, b);

            img.put_pixel(i, j, vector_to_rgb(&colour));
        }
    }
    eprintln!("\rDone!");

    img.save(args.path).unwrap();
}
