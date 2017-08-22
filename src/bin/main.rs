extern crate sdl2;
extern crate rand;
use sdl2::rect::Rect;
use sdl2::keyboard;
use sdl2::scancode::ScanCode;
use sdl2::rect::Point;
use std::time;
use std::thread::sleep;
use sdl2::pixels;
use rand::distributions::Range;
use rand::distributions::IndependentSample;


fn main() {
    let sdl_context = sdl2::init().everything().build().unwrap();

    let window = sdl_context.window("Title", 100, 100).build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();
    for _ in 0..10 {
        event_pump.pump_events();

        drawer.clear();
        let ten_millis = time::Duration::from_millis(10);
        let range = Range::new(0,256);
        let rng = rand::thread_rng();
        for x in 0..100 {
            for y in 0..100 {
                // let range = Range::new(0,256);
                // let color = pixels::Color::RGB(range.ind_sample(&mut rng),range.ind_sample(&mut rng) ,range.ind_sample(&mut rng) );
                // drawer.set_draw_color(color);
                drawer.draw_point(Point {x, y});
                drawer.present();
                sleep(ten_millis);
            }
        };

        let keystate = keyboard::get_keyboard_state();
        let quit = keystate.get(&ScanCode::Escape).unwrap();

        if *quit {
            break;
        }
    }

}

