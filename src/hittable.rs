use crate::ray::Ray;
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    point: Vector3<f64>,
    normal: Vector3<f64>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3<f64>, normal: Vector3<f64>, t: f64, r: &Ray) -> Self {
        let front_face = r.direction.dot(&normal) < 0.0;
        Self {
            point,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &Vector3<f64> {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
