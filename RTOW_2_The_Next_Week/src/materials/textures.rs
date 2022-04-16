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
}

impl Texture for Perlin_Noise {
    fn value(&self, u: f64, v: f64, p: &point3) -> colorRGB {
        colorRGB::from(1.,1.,1.) * self.noise(p)
    }
}