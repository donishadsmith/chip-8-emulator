use macroquad::prelude::*;

pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

pub struct Display {
    pub height: usize,
    pub width: usize,
    pub panel: [[bool; WIDTH]; HEIGHT],
}

impl Display {
    pub fn on() -> Self {
        Self {
            height: HEIGHT,
            width: WIDTH,
            panel: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn draw(&self) {
        let scale_w = screen_width() / WIDTH as f32;
        let scale_h = screen_height() / HEIGHT as f32;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.panel[y][x] {
                    draw_rectangle(
                        x as f32 * scale_w,
                        y as f32 * scale_h,
                        scale_w,
                        scale_h,
                        WHITE,
                    );
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.panel = [[false; 64]; 32];
    }
}
