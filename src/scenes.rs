use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::random::{random_double, random_range_double, random_range_vector3, random_vector3};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use nalgebra::vector;
use std::path::PathBuf;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::new(
        vector![0.2, 0.3, 0.1],
        vector![0.9, 0.9, 0.9],
    ));
    let material_ground = Arc::new(Lambertian::new_from_texture(checker));
    world.add(Arc::new(Sphere::new(
        vector![0.0, -1000.0, -1.0],
        1000.0,
        material_ground,
    )));
    let material_glass = Arc::new(Dielectric::new(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = vector![
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double()
            ];

            if (center - vector![4.0, 0.2, 0.0]).norm() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vector3().component_mul(&random_vector3());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + vector![0.0, random_range_double(0.0, 0.5), 0.0];
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_range_vector3(0.5, 1.0);
                    let fuzz = random_range_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    world.add(Arc::new(Sphere::new(center, 0.2, material_glass.clone())));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        vector![0.0, 1.0, 0.0],
        1.0,
        material_glass,
    )));

    let material_brown = Arc::new(Lambertian::new(vector![0.4, 0.2, 0.1]));
    world.add(Arc::new(Sphere::new(
        vector![-4.0, 1.0, 0.0],
        1.0,
        material_brown,
    )));

    let material_mirror = Arc::new(Metal::new(vector![0.7, 0.6, 0.5], 0.0));
    world.add(Arc::new(Sphere::new(
        vector![4.0, 1.0, 0.0],
        1.0,
        material_mirror,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();

    let checker = Arc::new(CheckerTexture::new(
        vector![0.2, 0.3, 0.1],
        vector![0.9, 0.9, 0.9],
    ));

    objects.add(Arc::new(Sphere::new(
        vector![0.0, -10.0, 0.0],
        10.0,
        Arc::new(Lambertian::new_from_texture(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        vector![0.0, 10.0, 0.0],
        10.0,
        Arc::new(Lambertian::new_from_texture(checker)),
    )));

    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let permat = Arc::new(Lambertian::new_from_texture(pertext));
    objects.add(Arc::new(Sphere::new(
        vector![0.0, -1000.0, 0.0],
        1000.0,
        permat.clone(),
    )));
    objects.add(Arc::new(Sphere::new(vector![0.0, 2.0, 0.0], 2.0, permat)));

    objects
}

pub fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new(PathBuf::from("earthmap.jpg")));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(vector![0.0, 0.0, 0.0], 2.0, earth_surface));

    HittableList::new(globe)
}
