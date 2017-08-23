extern crate sdl2;

pub mod mandelbrot;
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
    for x in 0..limit {
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
    use super::*;
    #[test]
    fn origo() {
        let result = iterate( (0.0 , 0.0), 100);

        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn outside() {
        let result = iterate( (1.0, 6.0 ) , 100);

        let expected = Some(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn inside() {
        let result = iterate( (-1.0 , 0.0 ), 100);

        let expected = None;

        assert_eq!( result, expected);
    }

    
    #[test]
    fn inside_two () {
        let expected = None;

        let result = iterate( (-1.0, 0.2), 100);

        assert_eq!( expected, result );
    }

}
