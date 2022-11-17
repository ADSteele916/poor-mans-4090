use nalgebra::{vector, Vector3};

use crate::ray::Ray;

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub minimum: Vector3<f64>,
    pub maximum: Vector3<f64>,
}

impl Aabb {
    pub fn new(minimum: Vector3<f64>, maximum: Vector3<f64>) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0)
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = vector![
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z)
        ];
        let big = vector![
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z)
        ];
        Self::new(small, big)
    }
}
