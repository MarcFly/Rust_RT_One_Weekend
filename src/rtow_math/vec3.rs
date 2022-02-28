type point3 = vec3;
type colorRGB = vec3;

pub struct vec3 {
    v: [f32; 3],
}

impl vec3 {
    pub fn new() -> vec3 { vec3{v: [0.,0.,0.], } }
    pub fn from(x:&f32, y:&f32, z:&f32) -> vec3 { vec3{v: [*x,*y,*z]}}
    pub fn from_vec(v: &vec3) -> vec3 { vec3::from(v.x(), v.y(), v.z()) }
    pub fn x(&self) -> &f32 {&self.v[0]}
    pub fn y(&self) -> &f32 {&self.v[1]}
    pub fn z(&self) -> &f32 {&self.v[2]}
    
    pub fn add(&mut self, other: &vec3) -> &vec3 {
        self.v[0] += other.x();
        self.v[1] += other.y();
        self.v[2] += other.z();
        self
    }

    pub fn substract(&mut self, other: &vec3) -> &vec3 {
        self.v[0] -= other.x();
        self.v[1] -= other.y();
        self.v[2] -= other.z();
        self
    }

    pub fn mult(&mut self, other: &vec3) -> &vec3 {
        self.v[0] *= other.x();
        self.v[1] *= other.y();
        self.v[2] *= other.z();
        self
    }

    pub fn mult_sc(&mut self, other: &f32) -> &vec3 {
        self.v[0] *= other;
        self.v[1] *= other;
        self.v[2] *= other;
        self
    }

    pub fn div(&mut self, other: &f32) -> &vec3 {
        let inv = 1./other;
        self.mult_sc(&inv)
    }

    pub fn length_squared(&self) -> f32 {
        self.v[0]*self.v[0] + self.v[1]*self.v[1] + self.v[2]*self.v[2]
    }

    pub fn length(&self) -> f32 {
        (self.length_squared()).sqrt()
    }

    pub fn dot(&self, other: &vec3) -> f32 {
        self.v[0] * other.x() + self.v[1] * other.y() + self.v[2] * other.z()
    }

    pub fn cross(&self, other: &vec3) -> vec3 {
        let x = self.v[1] * other.z() - self.v[2] * other.y();
        let y = self.v[2] * other.x() - self.v[0] * other.z();
        let z = self.v[0] * other.y() - self.v[1] * other.x();
        vec3::from( &x, &y, &z)
    }

    pub fn unit_vec(&self) -> vec3 { 
        let l = self.length();
        let x = self.v[0] / l;
        let y = self.v[1] / l;
        let z = self.v[2] / l;
        vec3::from(&x,&y,&z)
    }

    pub fn write_color(self) {
        println!("{} {} {}", 
            (self.v[0] * 259.999) as i32,
            (self.v[1] * 259.999) as i32,
            (self.v[2] * 259.999) as i32,);
    }
}

