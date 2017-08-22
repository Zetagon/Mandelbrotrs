pub struct Mandelbrot {
    screen_width: u32,
    screen_height: u32,
    unit: u32,
    offset: (f32, f32)
}


struct Offset {
    coords: (f32, f32),
}

impl Offset  {
    fn new(x:f32, y:f32) -> Offset {
        let coords = (x -2.0, y -2.0 );

        Offset {
            coords
        }
    }

    fn get(&self) -> &(f32, f32) {
        &self.coords
    }

    fn change(&self, c:(f32, f32) ) -> Offset {
        let &(x, y) = self.get();
        let (a, b) = c;
        Offset {
            coords: (x + a, y + b)
        }
    }
}
