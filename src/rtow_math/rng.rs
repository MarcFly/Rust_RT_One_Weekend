use rand::Rng;

pub fn rand_f64() -> f64 {
    rand::thread_rng().gen_range(0.0..1000.0) / 1000.
}

pub fn rand_f64_r(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}