extern crate sdl2;
extern crate rand;
extern crate Mandelbrot;

use sdl2::rect::Rect;
use sdl2::keyboard;
use sdl2::scancode::ScanCode;
use sdl2::rect::Point;
use std::time;
use std::thread::sleep;
use sdl2::pixels;
use sdl2::render;
use Mandelbrot::mandelbrot;


fn main() {
    let sdl_context = sdl2::init().everything().build().unwrap();

    let screen_width = 1920;
    let screen_height = 1080;
    let unit = 4_f32 / screen_height as f32;

    let window = sdl_context.window("Title", screen_width, screen_height).build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();
    let settings = mandelbrot::Mandelbrot {
        screen_height,
        screen_width,
        unit,
        limit: 200,
        offset: mandelbrot::Offset::new(0.0, 0.0),
    };

    // draw(&mut drawer, screen_width, screen_height, unit);

    let colors = mandelbrot::generate(&settings);
    loop {


        mandelbrot::draw( &mut drawer, &colors);
        event_pump.pump_events();

        drawer.present();

        let keystate = keyboard::get_keyboard_state();
        let quit = keystate.get(&ScanCode::Escape).unwrap();

        if *quit {
            break;
        }
    }

}


