extern crate sdl2;

///
/// Calculates whether the point c is a part of the Mandelbrot set
///
/// Returns the number of iterations(wrapped in a Some) before Z crosses the radius 2, or if it doesn't: None
///
///
///
///
pub fn iterate(c: (f32, f32), limit: u32) -> Option<u32>  {
    let (c_real, c_imag) = c;
    let (mut z_real, mut z_imag) = (0.0_f32 , 0.0_f32 );
    let mut temp: f32;
    let mut z_real_squared:f32;
    let mut z_imag_squared:f32;
    for x in 1..limit {
        z_real_squared = z_real * z_real;
        z_imag_squared = z_imag * z_imag;
        if z_real_squared + z_imag_squared > 4_f32 {
            return Some(x);
        }

        temp = z_real;

        z_real = z_real * z_real - z_imag * z_imag + c_real;
        z_imag = 2_f32 * temp * z_imag + c_imag;
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
