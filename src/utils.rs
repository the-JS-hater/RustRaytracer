use rand::Rng;
use std::f64::consts::PI;

pub fn degrees_to_radian(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rnd_float() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    return y;
}

pub fn rnd_float_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(min..max);
    return y;
}
