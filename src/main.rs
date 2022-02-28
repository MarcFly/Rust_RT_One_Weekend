mod ppm;
mod rtow_math;

fn main() {
    //ppm::output_ppm();
    ppm::output_ppm_vec3();
}

#[cfg(test)]
mod ppm_tests {
    #[test]
    fn output_ppm() {
        // Image
        let w: i32 = 256;
        let h: i32 = 256;
        let float_w: f64 = 256.;
        let float_h: f64 = 256.;

        // Render
        println!("P3\n{}' '{}\n255\n", w,h);
        
        for i in (0..(h-1)).rev() {
            println!("Line {}", i);
            let float_i: f64 = (i).into();

            for j in (0..w) {
                println!("Row {}", j);
                let float_j: f64 = (i).into();
                let r: f64 = float_i / (float_w-1.);
                let g: f64 = float_j / (float_h-1.);
                let b: f64 = 0.25;
                
                let ir: i32 = (255.999 * r) as i32;
                let ig: i32 = (255.999 * g) as i32;
                let ib: i32 = (255.999 * b) as i32;
 
                println!("{}r {}g {}b",r,g,b);
            }
        }
    }
}