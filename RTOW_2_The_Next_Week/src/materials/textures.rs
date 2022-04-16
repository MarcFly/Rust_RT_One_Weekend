use crate::rtow_math::vec3::*;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB;
}

pub struct Solid_Color {
    color: colorRGB,
}

impl Solid_Color {
    pub fn new() -> Solid_Color {
        Solid_Color { color: colorRGB::new() }
    }

    pub fn from(red: f64, green: f64, blue: f64) -> Solid_Color {
        Solid_Color { color: colorRGB::from(red, green, blue)} 
    }

    pub fn from_colorRGB(color: colorRGB) -> Solid_Color {
        Solid_Color { color }
    }
}

impl Texture for Solid_Color {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        self.color
    }
}

// -----------------------------------------------------------------
pub struct Checkerboard_Tex {
    odd: colorRGB,
    even: colorRGB,
}

impl Checkerboard_Tex {
    pub fn new() -> Checkerboard_Tex {
        Checkerboard_Tex { odd: colorRGB::from(0.,0.,0.), even: colorRGB::from(1.,1.,1.) }
    }
}

impl Texture for Checkerboard_Tex {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        let sines = (p.z() * 10.).sin() * (p.y() * 10.).sin() * (p.x() * 10.).sin();
        if sines < 0. { self.odd } else { self.even }
    }
}

// ----------------------------------------------------------------
use crate::rtow_math::rng::*;
pub struct Tile_Noise {
    values: Vec<f64>,
}

impl Tile_Noise {
    pub fn new(num_vals: i32) -> Tile_Noise {
        let mut values: Vec<f64> = Vec::new();
        for i in 0..num_vals {
            values.push(rand_f64_r(0.,1.));
        }

        Tile_Noise { values }
    }
}

impl Texture for Tile_Noise {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        let id = (self.values.len() as f64 * (u*v / 2.)).floor() as usize;
        let sines = (p.z() * 10.).sin() * (p.y() * 10.).sin() * (p.x() * 10.).sin();
        let mut sin_id = ((sines+1.) / 2. * self.values.len() as f64) as usize;
        let col = self.values[sin_id];
        colorRGB::from(col, col, col)
    }
}
pub struct Perlin_Noise {
    ranfloats: [f64; 256], 
    x_ind: [usize; 256],
    y_ind: [usize; 256],
    z_ind: [usize; 256],
}

use rand::Rng;

impl Perlin_Noise {
    pub fn new() ->  Perlin_Noise {
        let mut ranfloats: [f64; 256] = [0.; 256];
        for i in 0..256 {
            ranfloats[i]  = rand_f64();
        }
        
        Perlin_Noise{
            ranfloats,
            x_ind: Perlin_Noise::perlin_gen_permutation(),
            y_ind: Perlin_Noise::perlin_gen_permutation(),
            z_ind: Perlin_Noise::perlin_gen_permutation(),
        }
    }

    pub fn noise(&self, p: &point3) -> f64 {
        if (*p.x() < 0. || *p.z() < 0.) {
            let things_go_wrong = false;
        }
        let i = ((4.*p.x()) as i32 & 255) as usize;
        let j = ((4.*p.y()) as i32 & 255) as usize;
        let k = ((4.*p.z()) as i32 & 255) as usize;
        let test_val = self.x_ind[i] ^ self.y_ind[j] ^ self.z_ind[k];
        self.ranfloats[self.x_ind[i] ^ self.y_ind[j] ^ self.z_ind[k]]
    }

    pub fn tile_noise(&self, p: &point3) -> f64 {
        let i = ((4.*p.x()) as i32 & 255) as usize;
        let j = ((4.*p.y()) as i32 & 255) as usize;
        let k = ((4.*p.z()) as i32 & 255) as usize;
       
        self.ranfloats[i ^ j ^ k]
    }

    // Private Funs
    
    fn perlin_gen_permutation() -> [usize; 256] {

        let mut perm: [usize; 256] = [0; 256];
        for i in 0..256 {
            perm[i] = i;
        }
        Perlin_Noise::permute(&mut perm);
        perm
    }

    fn permute(vals: &mut [usize; 256]) {
        for i in 0..256 {
            let target = rand::thread_rng().gen_range(0..256);
            let tmp = vals[i];
            vals[i] = vals[target];
            vals[target] = tmp;
        }
    }

    // Filtered Noises

    pub fn lerp_noise(&self, p: &point3) -> f64 {
        let _val =  p.v[rand_i8_r(0, 3) as usize];
        let u = _val - _val.floor();

        let i = (p.x().floor()) as usize;
        let j = (p.y().floor()) as usize;
        let k = (p.z().floor()) as usize;

        let vals = [
            self.ranfloats[self.x_ind[i & 255]],
            self.ranfloats[self.x_ind[((i as i32 + 1) & 255) as usize]]
        ];
        f64_lerp(vals, u)
    }

    pub fn trilerp_noise(&self, p: &point3) -> f64 {
        let u = p.v[0] - p.v[0].floor();
        let v = p.v[1] - p.v[1].floor();
        let w = p.v[2] - p.v[2].floor();
        
        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;

        let mut vals : [f64; 2*2*2] = [0.; 2*2*2];
        for it1 in 0..2 {
            let ind_1 = ((i + it1 as i32) & 255) as usize;

            for it2 in 0..2 {
                let ind_2 = ((j + it2 as i32) & 255) as usize;

                for it3 in 0..2 {
                    let ind_3 = ((k + it3 as i32) & 255) as usize;

                    vals[it1*2*2 + it2*2 + it3] = self.ranfloats[self.x_ind[ind_1] ^ self.y_ind[ind_2] ^ self.z_ind[ind_3]];
                }
            }
        };

        f64_trilerp(vals, [u, v, w])
    }

    pub fn trilerp_and_hermit_cubic_noise(&self, p: &point3) -> f64 {
        let mut u = p.v[0] - p.v[0].floor();
        let mut v = p.v[1] - p.v[1].floor();
        let mut w = p.v[2] - p.v[2].floor();
        
        // Apply 3rd hermit cubic to uvw
        u = u*u*(3.-2.*u);
        v = v*v*(3.-2.*v);
        w = w*w*(3.-2.*w);


        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;

        let mut vals : [f64; 2*2*2] = [0.; 2*2*2];
        for it1 in 0..2 {
            let ind_1 = ((i + it1 as i32) & 255) as usize;

            for it2 in 0..2 {
                let ind_2 = ((j + it2 as i32) & 255) as usize;

                for it3 in 0..2 {
                    let ind_3 = ((k + it3 as i32) & 255) as usize;

                    vals[it1*2*2 + it2*2 + it3] = self.ranfloats[self.x_ind[ind_1] ^ self.y_ind[ind_2] ^ self.z_ind[ind_3]];
                }
            }
        };

        f64_trilerp(vals, [u, v, w])
    }
}


impl Texture for Perlin_Noise {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        colorRGB::from(1.,1.,1.) * 
            //self.noise(p)
            //self.lerp_noise(p)
            //self.trilerp_noise(p)
            self.trilerp_and_hermit_cubic_noise(p)
    }
}

/// Linear Interpolations

pub fn f64_lerp(v: [f64; 2], t:f64) -> f64 {
     v[0] + ((v[1] - v[0]) * t)
}

pub fn f64_trilerp(v: [f64; 2*2*2], t: [f64; 3]) -> f64 {
    let mut accum = 0.;
    for i in 0..2 {
        let f_i = i as f64;
        
        for j in 0..2 {
            let f_j = j as f64;
            
            for k in 0..2 {
                let f_k = k as f64;
                accum += 
                    ((f_i*t[0] + (1.-f_i)*(1.-t[0])) *
                    (f_j*t[1] + (1.-f_j)*(1.-t[1])) *
                    (f_k*t[2] + (1.-f_k)*(1.-t[2])) *
                    v[i*2*2 +j*2 + k] );
            }
        }
    };

    accum
}