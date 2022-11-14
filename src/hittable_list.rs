use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
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
                rec = Some(hit);
                closest_so_far = hit.t();
            }
        }

        rec
    }
}
