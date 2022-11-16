use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::random::{random_double, random_range_double, random_range_vector3, random_vector3};
use crate::sphere::Sphere;
use nalgebra::vector;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(vector![0.5, 0.5, 0.5]));
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
