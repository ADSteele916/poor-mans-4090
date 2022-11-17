use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::random::{random_double, random_in_unit_sphere, random_unit_vector};
use crate::ray::Ray;
use crate::texture::{SolidColour, Texture};
use nalgebra::{vector, Vector3};

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = n.dot(&-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)>;

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        vector![0.0, 0.0, 0.0]
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(colour: Vector3<f64>) -> Self {
        Self {
            albedo: Arc::new(SolidColour::new(colour)),
        }
    }

    pub fn new_from_texture(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.norm() < 1e-7 {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo.value(rec.u(), rec.v(), &rec.point());
        let scattered = Ray::new(rec.point(), scatter_direction, r_in.time);
        Some((attenuation, scattered))
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
        let scattered = Ray::new(
            rec.point(),
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        if scattered.direction.dot(&rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let refraction_ratio = if rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = rec.normal().dot(&-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &rec.normal())
            } else {
                refract(&unit_direction, &rec.normal(), refraction_ratio)
            };

        let scattered = Ray::new(rec.point(), direction, r_in.time);
        Some((vector![1.0, 1.0, 1.0], scattered))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(colour: Vector3<f64>) -> Self {
        Self {
            emit: Arc::new(SolidColour::new(colour)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(colour: Vector3<f64>) -> Self {
        Self {
            albedo: Arc::new(SolidColour::new(colour)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        Some((
            self.albedo.value(rec.u(), rec.v(), &rec.point()),
            Ray::new(rec.point(), random_in_unit_sphere(), r_in.time),
        ))
    }
}
