use crate::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;
use crate::random::{random_in_unit_sphere, random_unit_vector};

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Vector3<f64>, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray,
               rec: &HitRecord,) -> Option<(Vector3<f64>, Ray)> {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.norm() < 1e-7 {
            scatter_direction = rec.normal();
        }

        let scattered = Ray::new(rec.point(), scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal());
        let scattered = Ray::new(rec.point(), reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction.dot(&rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
