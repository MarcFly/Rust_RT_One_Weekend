use rand::Rng;

pub fn rand_f64() -> f64 {
    rand::thread_rng().gen_range(0.0..1000.0) / 1000.
}

pub fn rand_f64_r(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_i8_r(min: i8, max: i8) -> i8 {
    rand::thread_rng().gen_range(min..max)
}