pub type point3 = vec3;
pub type colorRGB = vec3;
use num;

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub struct vec3 {
    pub v: [f64; 3],
}
type vec3_scalar = vec3;

use std::f64::INFINITY;
use std::ops;

impl ops::Add<vec3> for vec3 {
    type Output = vec3;
    fn add(self, other: vec3) -> vec3 {
        vec3::from(
            self.v[0] + other.x(),
            self.v[1] + other.y(),
            self.v[2] + other.z(),
        )
    }
}

impl ops::Add<f64> for vec3_scalar {
    type Output = vec3;
    fn add(self, other: f64) -> vec3 {
        vec3::from(
            self.v[0] + other,
            self.v[1] + other,
            self.v[2] + other,
        )
    }
}

impl ops::Sub<vec3> for vec3 {
    type Output = vec3;
    fn sub(self, other: vec3) -> vec3 {
        vec3::from(
            self.v[0] - other.x(),
            self.v[1] - other.y(),
            self.v[2] - other.z(),
        )
    }
}

impl ops::Mul<vec3> for vec3 {
    type Output = vec3;
    fn mul(self, other: vec3) -> vec3 {
        vec3::from(
            self.v[0] * other.x(),
            self.v[1] * other.y(),
            self.v[2] * other.z(),
        )
    }
}

impl ops::Mul<f64> for vec3_scalar {
    type Output = vec3;
    fn mul(self, other: f64) -> vec3 {
        vec3::from(
            self.v[0] * other,
            self.v[1] * other,
            self.v[2] * other,
        )
    }
}


impl ops::Div<f64> for vec3_scalar {
    type Output = vec3;
    fn div(self, other: f64) -> vec3 {
        vec3::from(
            self.v[0] / other,
            self.v[1] / other,
            self.v[2] / other,
        )
    }
}

use std::sync::Arc;
use std::sync::Mutex;
use crate::rtow_math::rng::*;

impl vec3 {
    pub fn new() -> vec3 { vec3{v: [0.,0.,0.], } }
    pub fn from(x:f64, y:f64, z:f64) -> vec3 { vec3{v: [x,y,z]}}
    pub fn from_vec(v: vec3) -> vec3 { vec3::from(v.v[0], v.v[1], v.v[2]) }
    pub fn one() -> vec3 { vec3::from(1.,1.,1.) }
    pub fn up() -> vec3 {vec3::from(0.,1.,0.)}
    pub fn new_rand(min: f64, max: f64) -> vec3 { vec3{ v: [rand_f64_r(min,max), rand_f64_r(min,max), rand_f64_r(min,max)]}}
    pub fn inf_min() -> vec3 { vec3 {v:[-INFINITY, -INFINITY, -INFINITY]}}
    pub fn inf_max() -> vec3 { vec3 {v:[INFINITY, INFINITY, INFINITY]}}
    pub fn x(&self) -> &f64 {&self.v[0]}
    pub fn y(&self) -> &f64 {&self.v[1]}
    pub fn z(&self) -> &f64 {&self.v[2]}
    
    pub fn compare_relaxed(&self, cmp: &vec3) -> bool {
        (
            (self.v[0].abs() - cmp.v[0].abs()) < 0.00001 &&
            (self.v[1].abs() - cmp.v[1].abs()) < 0.00001 &&
            (self.v[2].abs() - cmp.v[2].abs()) < 0.00001
        )
    }

    pub fn near_zero(&self) -> bool {
        let min = 1e-8;
        (self.v[0].abs() < min) && (self.v[1].abs() < min) && (self.v[2].abs() < min) 
    }

    pub fn length_squared(&self) -> f64 {
        self.v[0]*self.v[0] + self.v[1]*self.v[1] + self.v[2]*self.v[2]
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }

    pub fn dot(&self, other: &vec3) -> f64 {
        self.v[0] * other.x() + self.v[1] * other.y() + self.v[2] * other.z()
    }

    pub fn cross(&self, other: &vec3) -> vec3 {
        let x = self.v[1] * other.z() - self.v[2] * other.y();
        let y = self.v[2] * other.x() - self.v[0] * other.z();
        let z = self.v[0] * other.y() - self.v[1] * other.x();
        vec3::from( x, y, z)
    }

    pub fn unit_vec(&self) -> vec3 { 
        let l = self.length();
        let x = self.v[0] / l;
        let y = self.v[1] / l;
        let z = self.v[2] / l;
        vec3::from( x, y, z)
    }

    pub fn sqrt(&self) -> vec3 {
        vec3::from(
            self.v[0].sqrt(),
            self.v[1].sqrt(),
            self.v[2].sqrt(),
        )
    }

    pub fn reflect(&self, n: &vec3) -> vec3 {
        *self - *n * 2. * self.dot(n)
    }

    pub fn refract(&self, n: &vec3, coef: f64) -> vec3 {
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
            (256.0 * num::clamp(col.v[1], 0., 0.999)),
            (256.0 * num::clamp(col.v[2], 0., 0.999)));
        // Test without clamp
        print!("{} {} {}\n", 
            (256.0 * num::clamp(col.v[0], 0., 0.999)) as i32,
            (256.0 * num::clamp(col.v[1], 0., 0.999)) as i32,
            (256.0 * num::clamp(col.v[2], 0., 0.999)) as i32);
    }

    pub fn write_col_to(&self, pixel:  Arc<Mutex<colorRGB>>, idx: usize)
    {
        //let scale = 1. / samples_pp;
        let mut col = *self; // * scale;
        //col = col.sqrt(); // Gamma Correct
        // Test without clamp
        let mut check = true;
        let mut pix = pixel.lock().unwrap();
        *pix = colorRGB::from_vec(*self);
    }



    pub fn clamp(&mut self, min: f64, max:f64) {
        if self.v[0] < min {self.v[0] = min};
        if self.v[1] < min {self.v[1] = min};
        if self.v[2] < min {self.v[2] = min};

        if self.v[0] > max {self.v[0] = max};
        if self.v[1] > max {self.v[1] = max};
        if self.v[2] > max {self.v[2] = max};
    }
}


unsafe impl Send for vec3 {}

#[derive(Clone)]
pub struct Par_Pixel {
    pub color: colorRGB,
    pub i: i32,
    pub j: i32,
}

unsafe impl Send for Par_Pixel {}

impl Par_Pixel {
    pub fn set_col(&mut self, col: colorRGB) {
        self.color = col;
    }
}

use rayon::prelude::*;

//impl<T: Send> FromParallelITerator<T> for Vec<Par_Pixel> {
//    fn from_par_iter<I>(par_iter: I) -> Self
//        where I: IntoParallelIterator<Item=T>
//        {
//            let par_iter = par_iter.into_par_iter();
//
//        }
//}