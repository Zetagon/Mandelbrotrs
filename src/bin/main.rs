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


fn main() {
    let sdl_context = sdl2::init().everything().build().unwrap();

    let screen_width = 200;
    let screen_height = 200;
    let unit = 4_f32 / screen_height as f32;

    let window = sdl_context.window("Title", screen_width, screen_height).build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();

    draw(&mut drawer, screen_width, screen_height, unit);

    loop {

        event_pump.pump_events();

        drawer.present();

        let keystate = keyboard::get_keyboard_state();
        let quit = keystate.get(&ScanCode::Escape).unwrap();

        if *quit {
            break;
        }
    }

}


fn draw(drawer: &mut render::RenderDrawer, screen_width:u32, screen_height: u32, unit: f32) {
    //TODO: Put results in an array and then continuosly draw that onto the screen
    drawer.clear();
    let mut color = pixels::Color::RGB(0, 0, 0,);
    for y in 0..screen_height {
        for x in 0..screen_width {
            let (a, bi) = (x as f32 * unit, y as f32 * unit);
            let result = Mandelbrot::iterate( (a - 2.0, bi - 2.0), 1000);


            drawer.set_draw_color(match result {
                None =>  pixels::Color::RGB(0, 0, 0),
                Some(s) =>  pixels::Color::RGB(255, 255, 255),
            });
            let (x , y ) = (x as i32, y as i32);
            drawer.draw_point(Point {x, y});
        }
    }
}
