use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    point: Vector3<f64>,
    normal: Vector3<f64>,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Vector3<f64>,
        normal: Vector3<f64>,
        material: Arc<dyn Material>,
        t: f64,
        r: &Ray,
    ) -> Self {
        let front_face = r.direction.dot(&normal) < 0.0;
        Self {
            point,
            normal: if front_face { normal } else { -normal },
            material,
            t,
            front_face,
        }
    }

    pub fn point(&self) -> Vector3<f64> {
        self.point
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }

    pub fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
