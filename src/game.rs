use macroquad::prelude::*;
use crate::ball::*;
use crate:: paddle::*;
use crate::score::*;

pub struct Game {
    pub paddle_1: Paddle,
    pub paddle_2: Paddle,
    pub ball: Ball,
    pub scores: Score,
    pub is_gameover: bool,
    pub winner: u32,
}

impl Game {

    pub fn new() -> Game {
        Game {
            paddle_1: Paddle::new(0.0, 300.0, 10.0, 100.0, BLUE),
            paddle_2: Paddle::new(790.0, 300.0, 10.0, 100.0, GREEN),
            ball: Ball::new(400.0, 300.0, 12.0, 5.0, 5.0, WHITE),
            scores: Score::new(),
            is_gameover: false,
            winner: 0,
        }
    }

    pub fn move_ball(&mut self) {
        self.ball.move_ball();
    }

    pub fn move_paddle(&mut self) {

        if is_key_down(KeyCode::W) {
            self.paddle_1.up();
        }

        if is_key_down(KeyCode::S) {
            self.paddle_1.down();
        }

        if is_key_down(KeyCode::Up) {
            self.paddle_2.up();
        }

        if is_key_down(KeyCode::Down) {
            self.paddle_2.down();
        }

    }

    pub fn draw_ball(&self) {
        self.ball.draw_ball();
    }

    pub fn draw_paddles(&self) {
        self.paddle_1.draw();
        self.paddle_2.draw();
    }

    pub fn change_scores(&mut self, player: u32) {
        self.scores.increment_score(player);
    }

    pub fn draw_scores(&self) {
        self.scores.draw();
    }

    pub fn update(&mut self) {

        self.move_ball();
        self.move_paddle();
        self.draw_ball();
        self.draw_paddles();
        self.draw_scores();

        if self.ball.x > 800.0 {
            self.change_scores(1);
            self.ball.x = 400.0;
            self.ball.y = 300.0;
            self.ball.x_speed = -5.0;
            self.ball.y_speed = 5.0;
        }

        if self.ball.x < 0.0 {
            self.change_scores(2);
            self.ball.x = 400.0;
            self.ball.y = 300.0;
            self.ball.x_speed = 5.0;
            self.ball.y_speed = 5.0;
        }

        if self.ball.y > 600.0 {
            self.ball.y_speed = -self.ball.y_speed;
        }

        if self.ball.y < 0.0 {
            self.ball.y_speed = -self.ball.y_speed;
        }

        if self.is_gameover {
            self.draw_gameover_screen();
        }

    }

    pub fn gameover(&mut self, winner: u32) {
        self.is_gameover = true;
        self.winner = winner;
    }

    pub fn draw_gameover_screen(&mut self) {

        if self.is_gameover {
            self.ball.x_speed = 0.0;
            self.ball.y_speed = 0.0;
            if self.winner == 1 {
                clear_background(BLACK);
                draw_text(
                    "Blue wins!",
                    400.0 - 50.0 * 2.0,
                    300.0,
                    50.0,
                    BLUE
                );
            } else {
                clear_background(BLACK);
                draw_text(
                    "Green wins!",
                    400.0 - 50.0 * 2.0,
                    300.0,
                    50.0,
                    GREEN
                );
            }
        }

    }

}