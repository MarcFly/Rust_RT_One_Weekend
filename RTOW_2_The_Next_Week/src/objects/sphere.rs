use crate::rtow_math::vec3::*;
use crate::rtow_math::ray::*;
use crate::materials::Material;
use crate::materials::Default;
use crate::objects::hit::*;
use crate::objects::aabb::*;
use crate::rtow_math::vec2::*;

/// Knowing that a point is outside a sphere if x^2 + y^2 + z^2 > Radius^2
/// Assuming sphere is at origin, else we just transalate point by center and calculate again
/// Basically (Point - Center)^2 < R^2 to calculate if poitn is in radius
/// 
/// But we are not doing points, we are throwing rays
/// So a ray that goes form an arbitrary origin and passes through the Point
/// Ray(origin, dir = (point - dir))
/// As we don't have an exact point, we use a function do declare Points that pass along a direction that intersects that point
/// P(t) = A + tb -> Where P would be an arbitray point
/// ((A + tb) - Center)*((A + tb) - Center) = R^2
/// ((A + tb)^2 - 2*(Center * (A + tb)) + C^2) = R^2
/// ( A^2 + 2*A*tb + tb^2) - 2*(C*A + C*tb) + C^2 = R^2
/// A^2 + 2*A*tb + tb^2 + 2AC + 2tbC + C^2 - R^2 = 0
/// tb^2 + 2(A+C)tb (A^2 + 2AC + C^2 - R^2) = 0
/// Knowing b, A, C -> Solve for t as a Quadratic equation
/// - 2(A+C)b +- sqrt( (2(A+C)b)^2 - 4*(b)*(A^2 + 2AC + C^2 - R^2)) / (2 * b)
/// The possible solutions for t and a quadrative equation are { 0 = 1 point, Negative = 0 points, Positive = 2 points}

pub fn hit_sphere(center: &point3, radius: f64, ray_dir: &ray) -> f64 {
    let origin_center = ray_dir.origin - *center;
    // let a = ray_dir.dir.dot(&ray_dir.dir); // dot of a vector against istelf = sqrt(length(vec))
    let a = ray_dir.dir.length_squared();
    // let b = 2.0 * origin_center.dot(&ray_dir.dir); // Because of the sphere interesection solving, we can ignore the 2.0
    // That's because the equation by itself shas everything * 2 except b, which we do have it as it is 2*(A+C)
    // In the sqrt(b^2 - 4ac) -> sqrt((2*(A+C)) ^ 2 - 4 *b*(A^2 + 2AC + C^2 - R^2)) 
    // -> which we can take out into -> 2 * sqrt((A+C)^2 - b*(A^2 + 2AC + C^2 - R^2)), so we can cancel out a 2 in all the operations
    let half_b = origin_center.dot(&ray_dir.dir);
    let c = origin_center.dot(&origin_center) - radius*radius;

    // (b*b - 4.*a*c) > 0. // Dirty Check discriminant
    // let discriminant = b*b - 4.*a*c;// Because we half b, we do too in other parts
    let discriminant = half_b*half_b - a*c;
    check_discriminant(discriminant, half_b, a)
}

fn check_discriminant(discriminant: f64, b: f64, a: f64) -> f64 {
    if(discriminant < 0.) {  
        -1. 
    }
    else {
        //(-b - discriminant.sqrt()) / (2.*a) // We cancel out the 2 form previous operations
        (-b - discriminant.sqrt()) / (a)
    }
}

use std::f64::consts::PI;
use std::sync::Arc;
pub struct sphere {
    pub center: point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

use crate::materials::*;

impl sphere {
    //pub fn new() -> sphere {sphere{center: point3::new(), radius: 1., mat: Arc::new(def_material)}}
    //pub fn from(p: point3, r: f64) -> sphere { sphere{center: p, radius: r, mat: Arc::new(def_material)}}
    pub fn from_mat(p: point3, r: f64, mat_p: Arc::<dyn Material>) -> sphere {
        sphere { center: p, radius: r, mat: mat_p}
    }

    fn get_uv(&self, hit_pos: &point3, uv: &mut point2) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        
        let theta = (-hit_pos.y()).acos();
        let phi = (-hit_pos.z()).atan2(*hit_pos.x()) + PI;
        
        uv.v[0] = phi / (2.*PI);
        uv.v[1] = theta / PI;
    }
}

use crate::objects::hittable_list::*;
use std::sync::Mutex;

unsafe impl Send for sphere {}
unsafe impl Sync for sphere {}
impl Hittable for sphere {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:& mut hit_record) -> bool {
        let origin_center = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = origin_center.dot(&r.dir);
        let c = origin_center.dot(&origin_center) - self.radius*self.radius;
        
        let mut discriminant = half_b*half_b - a*c;
        //discriminant = check_discriminant(discriminant, half_b, a);
        if (discriminant < 0.) {return false}
        
        let sq_discr = discriminant.sqrt();
        let mut root = (-half_b - sq_discr) / a;
        if(root < t_min || root > t_max) {
            root = (-half_b +sq_discr) / a;
            if(root < t_min || t_max < root) {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.n = (rec.p - self.center) / self.radius; // This is bad, only returns normal pointing outwards
        // What if we need to differentiate between from and back face!
        rec.set_face_normal(r);
        rec.mat = Arc::clone(&self.mat);
        self.get_uv(&rec.p, &mut rec.uv);

        true
    }   

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        (true, aabb::from(
            self.center - vec3::from(self.radius, self.radius, self.radius),
            self.center + vec3::from(self.radius, self.radius, self.radius)
        ))
    }
}

use crate::rtow_math::defines;
use crate::rtow_math::rng::*;

pub fn random_in_sphere() -> point3 {
    while(true) {
        let p = point3::from(
            rand_f64_r(-1., 1.),
            rand_f64_r(-1., 1.),
            rand_f64_r(-1., 1.));
        if p.length() < 1. { return p }
    };
    point3::new() // Something has gone terribly bad
}

pub fn random_in_sphere_1() -> point3 {
    point3::from(
        rand_f64_r(-1., 1.),
        rand_f64_r(-1., 1.),
        rand_f64_r(-1., 1.),
    )
}

pub fn random_in_sphere_bad() -> point3 {
    point3::from(
        rand_f64_r(*defines::side_min, *defines::side_max),
        rand_f64_r(*defines::side_min, *defines::side_max),
        rand_f64_r(*defines::side_min, *defines::side_max),
    )
}

// MOVING SPHERE

pub struct moving_sphere {
    pub center0: point3,
    pub center1: point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}


impl moving_sphere {    
    pub fn from_all(center0: point3, center1: point3, time0: f64, time1: f64, r: f64, mat: Arc::<dyn Material>) -> moving_sphere {
        moving_sphere {
            center0,
            center1,
            time0,
            time1,
            radius: r,
            mat,
        }
    }

    pub fn center(&self, time: f64) -> point3 {
        point3::from_vec(self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0)))
    }
}


unsafe impl Send for moving_sphere {}
unsafe impl Sync for moving_sphere {}
impl Hittable for moving_sphere {
    fn hit(&self, r: &ray, t_min: f64, t_max: f64, rec:& mut hit_record) -> bool {
        let origin_center = r.origin - self.center(r.time);
        let a = r.dir.length_squared();
        let half_b = origin_center.dot(&r.dir);
        let c = origin_center.dot(&origin_center) - self.radius*self.radius;
        
        let mut discriminant = half_b*half_b - a*c;
        //discriminant = check_discriminant(discriminant, half_b, a);
        if (discriminant < 0.) {return false}
        
        let sq_discr = discriminant.sqrt();
        let mut root = (-half_b - sq_discr) / a;
        if(root < t_min || root > t_max) {
            root = (-half_b +sq_discr) / a;
            if(root < t_min || t_max < root) {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.n = (rec.p - self.center(r.time)) / self.radius; // This is bad, only returns normal pointing outwards
        rec.iters += 1;
        // What if we need to differentiate between from and back face!
        rec.set_face_normal(r);
        rec.mat = Arc::clone(&self.mat);

        true
    }   

    fn get_aabb(&self, time0: f64, time1: f64) -> (bool, aabb) {
        let time0_aabb = aabb::from(
            self.center(time0) - vec3::from(self.radius, self.radius, self.radius),
            self.center(time0) + vec3::from(self.radius, self.radius, self.radius)
        );

        let time1_aabb = aabb::from(
            self.center(time1) - vec3::from(self.radius, self.radius, self.radius),
            self.center(time1) + vec3::from(self.radius, self.radius, self.radius)
        );

        (true, aabb::from_2_aabb(time0_aabb, time1_aabb))
    }
}