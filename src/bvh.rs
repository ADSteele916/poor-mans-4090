use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::random::random_int;
use crate::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::make_node(
            &mut list.objects().to_vec(),
            0,
            list.objects().len(),
            time0,
            time1,
        )
    }

    fn make_node(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Less => (objects[start].clone(), objects[start + 1].clone()),
                _ => (objects[start + 1].clone(), objects[start].clone()),
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            (
                Arc::new(Self::make_node(objects, start, mid, time0, time1)) as Arc<dyn Hittable>,
                Arc::new(Self::make_node(objects, mid, end, time0, time1)) as Arc<dyn Hittable>,
            )
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in bvh_node constructor.")
        }

        let bbox = Aabb::surrounding_box(&box_left.unwrap(), &box_right.unwrap());

        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        if let Some(left_hit) = self.left.hit(r, t_min, t_max) {
            if let Some(right_hit) = self.right.hit(r, t_min, left_hit.t()) {
                Some(right_hit)
            } else {
                Some(left_hit)
            }
        } else {
            self.right.hit(r, t_min, t_max)
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in bvh_node constructor.")
    }

    box_a.unwrap().minimum[axis]
        .partial_cmp(&box_b.unwrap().minimum[axis])
        .unwrap()
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
