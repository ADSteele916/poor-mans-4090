use nalgebra::{vector, Vector3};
use rand::Rng;

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_vector3() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    vector![rng.gen(), rng.gen(), rng.gen()]
}

pub fn random_range_vector3(min: f64, max: f64) -> Vector3<f64> {
    vector![min, min, min] + (max - min) * random_vector3()
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_range_vector3(-1.0, 1.0);
        if p.norm_squared() <= 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}
