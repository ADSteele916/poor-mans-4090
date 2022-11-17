use std::sync::Arc;

use nalgebra::{vector, Vector3};

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::random::random_double;
use crate::ray::Ray;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, colour: Vector3<f64>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(colour)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const DEBUG: bool = false;
        let debugging = DEBUG && random_double() < 0.00001;

        let rec1 = match self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        let rec2 = match self.boundary.hit(r, rec1.t() + 0.0001, f64::INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        if debugging {
            eprintln!("t_min={}, t_max={}", rec1.t(), rec2.t());
        }

        let mut t_min = t_min.max(rec1.t());
        let t_max = t_max.min(rec2.t());

        if t_min >= t_max {
            return None;
        }

        t_min = t_min.max(0.0);

        let ray_length = r.direction.norm();
        let distance_inside_boundary = (t_max - t_min) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t_min + hit_distance / ray_length;
        let point = r.at(t);

        if debugging {
            eprint!(
                "hit_distance = {}\nrec.t = {}\nrec.point = {}\n",
                hit_distance, t, point
            );
        }

        Some(HitRecord::new(
            point,
            vector![1.0, 0.0, 0.0],
            self.phase_function.clone(),
            t,
            0.0,
            0.0,
            r,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
