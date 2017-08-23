extern crate sdl2;

use std::vec::Vec;
use std::vec;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::pixels;
use sdl2::render;
use super::mandelbrot;
use iterate;

pub struct Mandelbrot {
    pub screen_width: u32,
    pub screen_height: u32,
    pub unit: f32,
    pub offset: Offset,
    pub limit: u32,
}


pub struct Offset {
    coords: (f32, f32),
}

impl Offset  {
    pub fn new(x:f32, y:f32) -> Offset {
        let coords = (x - 2.0, y - 2.0 );
        println!("{:?}", coords);

        Offset {
            coords
        }
    }

    pub fn get(&self) -> &(f32, f32) {
        &self.coords
    }

    pub fn change(&self, c:(f32, f32) ) -> Offset {
        let &(x, y) = self.get();
        let (a, b) = c;
        Offset {
            coords: (x + a, y + b)
        }
    }
}


pub fn draw( drawer: &mut render::RenderDrawer, mandelbrot: &Vec<Vec<Color>>) {
    drawer.clear();

    for (x, ary) in mandelbrot.iter().enumerate() {
        for (y, color) in ary.iter().enumerate() {
            drawer.set_draw_color(*color);
            drawer.draw_point(Point {x:x as i32, y:y as i32});
        }
    }
}

pub fn generate ( mandelbrot: &Mandelbrot) -> Vec<Vec<Color>> {
    let &Mandelbrot {
        screen_width,
        screen_height,
        unit,
        ref offset,
        limit,
    } = mandelbrot;
    let &(offset_real, offset_imag) = offset.get();
    let mut returnAry = vec![vec![Color::RGB(0, 0, 0) ; screen_height as usize] ; screen_width as usize];

    for (x, ary) in returnAry.iter_mut().enumerate() {
        for (y, element) in ary.iter_mut().enumerate() {
            let c = (x as f32 * unit + offset_real , y as f32 * unit + offset_imag );
            // println!("{:?}", c);
            // println!("{:?}", c.0);

            *element = match iterate(c, limit) {
                Some(i) =>  Color::RGB(255, 255, 255), // c is outside the set
                None => Color::RGB(0, 0, 255),
            };
        }
    }

    returnAry
}
