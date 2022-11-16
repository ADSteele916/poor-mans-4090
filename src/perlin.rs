use nalgebra::{vector, Vector3};

use crate::random::{random_int, random_range_vector3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vector3<f64>; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = [vector![0.0, 0.0, 0.0]; POINT_COUNT];
        for vec in ranvec.iter_mut() {
            *vec = random_range_vector3(-1.0, 1.0).normalize();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];

        for (i, point) in p.iter_mut().enumerate() {
            *point = i
        }

        Self::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    pub fn turb(&self, p: &Vector3<f64>) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Vector3<f64>) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[vector![0.0, 0.0, 0.0]; 2]; 2]; 2];

        for (di, plane) in c.iter_mut().enumerate() {
            for (dj, row) in plane.iter_mut().enumerate() {
                for (dk, vec) in row.iter_mut().enumerate() {
                    *vec = self.ranvec[self.perm_x[((i + (di as i32)) & 255) as usize]
                        ^ self.perm_y[((j + (dj as i32)) & 255) as usize]
                        ^ self.perm_z[((k + (dk as i32)) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_interp(c: [[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, plane) in c.iter().enumerate() {
            for (j, row) in plane.iter().enumerate() {
                for (k, vec) in row.iter().enumerate() {
                    let weight_v = vector![u - (i as f64), v - (j as f64), w - (k as f64)];
                    accum += ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu))
                        * ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv))
                        * ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww))
                        * vec.dot(&weight_v);
                }
            }
        }

        accum
    }
}
