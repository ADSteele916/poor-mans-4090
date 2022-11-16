use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{vector, Vector3};
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn get_sphere_uv(p: &Vector3<f64>) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (root > t_max) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (root > t_max) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let (u, v) = Self::get_sphere_uv(&normal);
        Some(HitRecord::new(p, normal, self.material.clone(), t, u, v, r))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - vector![self.radius, self.radius, self.radius],
            self.center + vector![self.radius, self.radius, self.radius],
        ))
    }
}
