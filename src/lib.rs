#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}


impl Rectangle {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x &&
            self.y < other.y + other.height && self.y + self.height > other.y
    }

    fn split(&self) -> (Rectangle, Rectangle, Rectangle, Rectangle) {
        (
            Rectangle::new(self.x, self.y, self.width / 2, self.height / 2),
            Rectangle::new(self.x + self.width / 2, self.y, self.width / 2, self.height / 2),
            Rectangle::new(self.x, self.y + self.height / 2, self.width / 2, self.height / 2),
            Rectangle::new(self.x + self.width / 2, self.y + self.height / 2, self.width / 2, self.height / 2),
        )
    }
}

struct Quad {
    bounds: Rectangle,
    capacity: usize,
    data: Vec<(u32, u32)>,
    divided: bool,

    ne: Option<Box<Quad>>,
    nw: Option<Box<Quad>>,
    se: Option<Box<Quad>>,
    sw: Option<Box<Quad>>,
}

impl Quad {
    fn new(bounds: Rectangle, capacity: usize) -> Quad {
        Quad {
            bounds,
            capacity,
            data: Vec::new(),
            divided: false,

            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    fn subdivide(&mut self) -> () {
        let (ne, nw, se, sw) = self.bounds.split();

        self.ne = Some(Box::new(Quad::new(ne, self.capacity)));
        self.nw = Some(Box::new(Quad::new(nw, self.capacity)));
        self.se = Some(Box::new(Quad::new(se, self.capacity)));
        self.sw = Some(Box::new(Quad::new(sw, self.capacity)));

        self.divided = true;
    }

    fn insert(&mut self, point: (u32, u32)) -> bool {
        if !self.bounds.contains(point.0, point.1) {
            false
        }

        else if self.data.len() < self.capacity {
            self.data.push(point);
            true
        }

        else {
            if !self.divided {
                self.subdivide();
            }

            if (self.ne.as_mut().unwrap()).insert(point) {
                return true
            }
            else if (self.nw.as_mut().unwrap()).insert(point) {
                return true
            }
            else if (self.se.as_mut().unwrap()).insert(point) {
                return true
            }
            else if (self.sw.as_mut().unwrap()).insert(point) {
                return true
            }

            false
        }
    }

    fn remove(&mut self, point: (u32, u32)) -> bool {

        todo!("Quad::remove");
        self.clear();
    }

    fn query(&self, bounds: Rectangle) -> Vec<(u32, u32)> {
        if !bounds.intersects(&self.bounds) {
            return Vec::new();
        }

        let mut result = Vec::new();

        for point in &self.data {
            if bounds.contains(point.0, point.1) {
                result.push(*point);
            }
        }

        if self.divided {
            result.extend((self.ne.as_ref().unwrap()).query(bounds));
            result.extend((self.nw.as_ref().unwrap()).query(bounds));
            result.extend((self.se.as_ref().unwrap()).query(bounds));
            result.extend((self.sw.as_ref().unwrap()).query(bounds));
        }

        result
    }

    fn clear(&mut self) -> () {
        self.data.clear();
        self.divided = false;
        self.ne = None;
        self.nw = None;
        self.se = None;
        self.sw = None;
    }
}



#[cfg(test)]
mod tests {
    extern crate sdl2;

    use crate::*;

    const WINDOW_WIDTH: u32 = 1200;
    const WINDOW_HEIGHT: u32 = 1200;

    const GRID_WIDTH: u32 = 200;
    const GRID_HEIGHT: u32 = 200;

    const CELL_WIDTH: u32 = WINDOW_WIDTH / GRID_WIDTH;
    const CELL_HEIGHT: u32 = WINDOW_HEIGHT / GRID_HEIGHT;

    impl Quad {
        fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color) -> () {
            let rect = sdl2::rect::Rect::new(self.bounds.x as i32 * CELL_WIDTH as i32, self.bounds.y as i32 * CELL_HEIGHT as i32, (self.bounds.width) as u32 * CELL_WIDTH, (self.bounds.height) as u32 * CELL_HEIGHT);
            canvas.set_draw_color(color);
            canvas.draw_rect(rect).unwrap();
            if self.divided {
                (self.ne.as_ref().unwrap()).draw(canvas, color);
                (self.nw.as_ref().unwrap()).draw(canvas, color);
                (self.se.as_ref().unwrap()).draw(canvas, color);
                (self.sw.as_ref().unwrap()).draw(canvas, color);
            }
        }
    }

    #[test]
    fn demo() {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo: Video", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        
        let mut points: Vec<(u32, u32)> = Vec::new();

        let mut mouse_down: bool = false;
        let mut mouse_pos: (u32, u32) = (0, 0);
        let mut erase: bool = false;

        let mut event_pump = sdl.event_pump().unwrap();
        'running: loop {

            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                        points.clear();
                    },
                    sdl2::event::Event::MouseMotion { x, y, .. } => {
                        mouse_pos = (x as u32, y as u32);
                    },
                    sdl2::event::Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                        mouse_down = true;
                    },
                    sdl2::event::Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Right, .. } => {
                        erase = true;
                    },
                    sdl2::event::Event::MouseButtonUp { .. } => {
                        mouse_down = false;
                        erase = false;
                    },
                    _ => {}
                }
            }

            if mouse_down {
                let x = mouse_pos.0 / CELL_WIDTH;
                let y = mouse_pos.1 / CELL_HEIGHT;
                let point = (x, y);

                if !points.contains(&point) {
                    points.push(point);
                }
            }

            if erase {
                let x = mouse_pos.0 / CELL_WIDTH;
                let y = mouse_pos.1 / CELL_HEIGHT;
                let point = (x, y);
                if points.contains(&point) {
                    let point_index = points.iter().position(|&p| p == point).unwrap();
                    points.remove(point_index);
                }
            }

            

            let mut quad: Quad = Quad::new(Rectangle::new(0, 0, GRID_WIDTH, GRID_HEIGHT), 10);

            for point in &points {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas.fill_rect(sdl2::rect::Rect::new((point.0 * CELL_WIDTH) as i32 , (point.1 * CELL_HEIGHT) as i32, CELL_WIDTH, CELL_HEIGHT)).unwrap();

                quad.insert(*point);
            }
            
            quad.draw(&mut canvas, sdl2::pixels::Color::RGB(125, 125, 125));

            canvas.present();
        }
    }
}
