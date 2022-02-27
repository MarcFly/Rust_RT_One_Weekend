#[cfg(test)]
mod ppm_tests {
    #[test]
    fn output_ppm() {
        // Image
        let w: int32 = 256;
        let h: int32 = 256;

        // Render
        println!("P3\n{}' '{}\n255\n", w,h);
    }
}