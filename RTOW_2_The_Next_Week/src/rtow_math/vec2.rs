use crate::rtow_math::vec3::*;
pub type point2 = vec2;
use num;

#[derive(Debug, Copy, Clone)]
pub struct vec2 {
    v: [f64; 2],
}
type vec2_scalar = vec2;

use std::ops;

impl ops::Add<vec2> for vec2 {
    type Output = vec2;
    fn add(self, other: vec2) -> vec2 {
        vec2::from(
            self.v[0] + other.x(),
            self.v[1] + other.y(),
        )
    }
}

impl ops::Add<f64> for vec2_scalar {
    type Output = vec2;
    fn add(self, other: f64) -> vec2 {
        vec2::from(
            self.v[0] + other,
            self.v[1] + other,
        )
    }
}

impl ops::Sub<vec2> for vec2 {
    type Output = vec2;
    fn sub(self, other: vec2) -> vec2 {
        vec2::from(
            self.v[0] - other.x(),
            self.v[1] - other.y(),
        )
    }
}

impl ops::Mul<vec2> for vec2 {
    type Output = vec2;
    fn mul(self, other: vec2) -> vec2 {
        vec2::from(
            self.v[0] * other.x(),
            self.v[1] * other.y(),
        )
    }
}

impl ops::Mul<f64> for vec2_scalar {
    type Output = vec2;
    fn mul(self, other: f64) -> vec2 {
        vec2::from(
            self.v[0] * other,
            self.v[1] * other,
        )
    }
}


impl ops::Div<f64> for vec2_scalar {
    type Output = vec2;
    fn div(self, other: f64) -> vec2 {
        vec2::from(
            self.v[0] / other,
            self.v[1] / other,
        )
    }
}

use std::sync::Arc;
use std::sync::Mutex;

impl vec2 {
    pub fn new() -> vec2 { vec2{v: [0.,0.], } }
    pub fn from(x:f64, y:f64) -> vec2 { vec2{v: [x,y]}}
    pub fn from_vec(v: vec2) -> vec2 { vec2::from(v.v[0], v.v[1]) }
    pub fn x(&self) -> &f64 {&self.v[0]}
    pub fn y(&self) -> &f64 {&self.v[1]}
    
    pub fn near_zero(&self) -> bool {
        let min = 1e-8;
        (self.v[0] < min) && (self.v[1] < min)
    }

    pub fn length_squared(&self) -> f64 {
        self.v[0]*self.v[0] + self.v[1]*self.v[1]
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    pub fn dot(&self, other: &vec2) -> f64 {
        self.v[0] * other.x() + self.v[1] * other.y()
    }

    pub fn crossA(&self, other: &vec2) -> f64 {
        self.v[0] * other.y() - self.v[1] * other.x()
    }

    pub fn crossB(&self) -> vec2 {
        vec2::from(self.v[1], - self.v[0])
    }

    pub fn unit_vec(&self) -> vec2 { 
        let l = self.length();
        let x = self.v[0] / l;
        let y = self.v[1] / l;
        vec2::from( x, y )
    }

    pub fn sqrt(&self) -> vec2 {
        vec2::from(
            self.v[0].sqrt(),
            self.v[1].sqrt()
        )
    }

    pub fn reflect(&self, n: &vec2) -> vec2 {
        *self - *n * 2. * self.dot(n)
    }

    pub fn refract(&self, n: &vec2, coef: f64) -> vec2 {
        let dot = (self.unit_vec() * -1.).dot(n);
        let cos = dot.min(1.);
        let perp = (*self + *n*cos ) * coef;
        let parl = *n * (num::abs(1. - perp.length_squared())).sqrt() * -1.;
        perp + parl
    }   

    pub fn write_color(&self, samples_pp: f64) {
        let scale = 1. / samples_pp;
        let mut col = *self * scale;
        col = col.sqrt(); // Gamma Correct
        let t_col = colorRGB::from(
            (256.0 * num::clamp(col.v[0], 0., 0.999)),
            (256.0 * num::clamp(col.v[1], 0., 0.999)), 0.);
        // Test without clamp
        print!("{} {} {}\n", 
            (256.0 * num::clamp(col.v[0], 0., 0.999)) as i32,
            (256.0 * num::clamp(col.v[1], 0., 0.999)) as i32, 0);
    }

    pub fn write_col_to(&self, pixel:  Arc<Mutex<colorRGB>>, idx: usize)
    {
        //let scale = 1. / samples_pp;
        let mut col = *self; // * scale;
        //col = col.sqrt(); // Gamma Correct
        // Test without clamp
        let mut check = true;
        let mut pix = pixel.lock().unwrap();
        *pix = colorRGB::from(self.v[0], self.v[1], 0.);

    }

    pub fn clamp(&mut self, min: f64, max:f64) {
        if self.v[0] < min {self.v[0] = min};
        if self.v[1] < min {self.v[1] = min};

        if self.v[0] > max {self.v[0] = max};
        if self.v[1] > max {self.v[1] = max};
    }
}


unsafe impl Send for vec2 {}