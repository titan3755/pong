use macroquad::prelude::*;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub color: Color
}

impl Ball {

    pub fn new(x: f32, y: f32, radius: f32, x_speed: f32, y_speed: f32, color: Color) -> Ball {
        Ball {
            x,
            y,
            radius,
            x_speed,
            y_speed,
            color
        }
    }

    pub fn move_ball(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;
    }

    pub fn draw_ball(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }


}