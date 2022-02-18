use macroquad::prelude::*;

pub struct Score {
    pub player_1: u32,
    pub player_2: u32,
}

impl Score {

    pub fn new() -> Score {
        Score {
            player_1: 0,
            player_2: 0,
        }
    }

    pub fn increment_score(&mut self, player: u32) {
        if player == 1 {
            self.player_1 += 1;
        } else {
            self.player_2 += 1;
        }
    }

    pub fn draw(&self) {
        draw_text(&format!("Score: {}", self.player_1), 10.0, 20.0, 25.0, WHITE);
        draw_text(&format!("Score: {}", self.player_2), 700.0, 20.0, 25.0, WHITE);
    }

}