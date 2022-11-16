use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{vector, Vector3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    point: Vector3<f64>,
    normal: Vector3<f64>,
    material: Arc<dyn Material>,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Vector3<f64>,
        normal: Vector3<f64>,
        material: Arc<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
    ) -> Self {
        let front_face = r.direction.dot(&normal) < 0.0;
        Self {
            point,
            normal: if front_face { normal } else { -normal },
            material,
            t,
            u,
            v,
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

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vector3<f64>,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vector3<f64>) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        self.ptr.hit(&moved_r, t_min, t_max).map(|rec| {
            HitRecord::new(
                rec.point() + self.offset,
                rec.normal(),
                rec.material().clone(),
                rec.t(),
                rec.u(),
                rec.v(),
                &moved_r,
            )
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr
            .bounding_box(time0, time1)
            .map(|bbox| Aabb::new(bbox.minimum + self.offset, bbox.maximum + self.offset))
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let new_box = if let Some(bbox) = ptr.bounding_box(0.0, 1.0) {
            let mut min = vector![f64::INFINITY, f64::INFINITY, f64::INFINITY];
            let mut max = vector![f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = (i as f64) * bbox.maximum.x + (1.0 - (i as f64)) * bbox.minimum.x;
                        let y = (j as f64) * bbox.maximum.y + (1.0 - (j as f64)) * bbox.minimum.y;
                        let z = (k as f64) * bbox.maximum.z + (1.0 - (k as f64)) * bbox.minimum.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = vector![newx, y, newz];

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            Some(Aabb::new(min, max))
        } else {
            None
        };

        Self {
            ptr,
            sin_theta,
            cos_theta,
            bbox: new_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut point = rec.point();
            let mut normal = rec.normal();

            point[0] = self.cos_theta * rec.point()[0] + self.sin_theta * rec.point()[2];
            point[2] = -self.sin_theta * rec.point()[0] + self.cos_theta * rec.point()[2];

            normal[0] = self.cos_theta * rec.normal()[0] + self.sin_theta * rec.normal()[2];
            normal[2] = -self.sin_theta * rec.normal()[0] + self.cos_theta * rec.normal()[2];

            Some(HitRecord::new(
                point,
                normal,
                rec.material().clone(),
                rec.t(),
                rec.u(),
                rec.v(),
                &rotated_r,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
