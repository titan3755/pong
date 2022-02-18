use macroquad::prelude::*;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Paddle {

    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Paddle {
        Paddle {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn up(&mut self) {
        self.y -= 10.0;
    }

    pub fn down(&mut self) {
        self.y += 10.0;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }

}