use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t();
                rec = Some(hit);
            }
        }

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = Aabb::default();
        let mut first_box = true;

        for object in &self.objects {
            if let Some(bbox) = object.bounding_box(time0, time1) {
                if first_box {
                    temp_box = bbox;
                } else {
                    temp_box = Aabb::surrounding_box(&temp_box, &bbox);
                    first_box = false;
                }
            } else {
                return None;
            }
        }

        Some(temp_box)
    }
}
