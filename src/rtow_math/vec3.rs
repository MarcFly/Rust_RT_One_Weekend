pub type point3 = vec3;
pub type colorRGB = vec3;

#[derive(Debug, Copy, Clone)]
pub struct vec3 {
    v: [f64; 3],
}

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

type vec3_scalar = vec3;
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

impl vec3 {
    pub fn new() -> vec3 { vec3{v: [0.,0.,0.], } }
    pub fn from(x:f64, y:f64, z:f64) -> vec3 { vec3{v: [x,y,z]}}
    pub fn from_vec(v: &vec3) -> vec3 { vec3::from(v.v[0], v.v[1], v.v[2]) }
    pub fn x(&self) -> &f64 {&self.v[0]}
    pub fn y(&self) -> &f64 {&self.v[1]}
    pub fn z(&self) -> &f64 {&self.v[2]}

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

    pub fn write_color(self) {
        println!("{} {} {}", 
            (255.999 * self.v[0]) as i32,
            (255.999 * self.v[1]) as i32,
            (255.999 * self.v[2]) as i32);
    }
}
