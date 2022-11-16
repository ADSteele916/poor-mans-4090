use std::sync::Arc;

use nalgebra::Vector3;

use crate::aabb::Aabb;
use crate::aarect::{XYRect, XZRect, YZRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;

pub struct AaBox {
    box_min: Vector3<f64>,
    box_max: Vector3<f64>,
    sides: HittableList,
}

impl AaBox {
    pub fn new(p0: Vector3<f64>, p1: Vector3<f64>, ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::default();

        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for AaBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
