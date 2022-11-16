use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{vector, Vector3};
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vector3<f64>,
    pub center1: Vector3<f64>,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vector3<f64>,
        center1: Vector3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vector3<f64> {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
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
        let normal = (p - self.center(r.time)) / self.radius;
        Some(HitRecord::new(
            p,
            normal,
            self.material.clone(),
            t,
            0.0,
            0.0,
            r,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box1 = Aabb::new(
            self.center(time0) - vector![self.radius, self.radius, self.radius],
            self.center(time0) + vector![self.radius, self.radius, self.radius],
        );
        let box2 = Aabb::new(
            self.center(time1) - vector![self.radius, self.radius, self.radius],
            self.center(time1) + vector![self.radius, self.radius, self.radius],
        );
        Some(Aabb::surrounding_box(&box1, &box2))
    }
}
