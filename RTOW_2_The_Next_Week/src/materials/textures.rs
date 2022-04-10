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

}