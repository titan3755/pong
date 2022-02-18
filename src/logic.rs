use macroquad::prelude::*;
use crate::game::*;

pub fn game_logic(game: &mut Game) {

    clear_background(BLACK);

    if game.ball.x < game.paddle_1.x + game.paddle_1.width && game.ball.x > game.paddle_1.x && game.ball.y > game.paddle_1.y && game.ball.y < game.paddle_1.y + game.paddle_1.height {
        game.ball.x_speed = -game.ball.x_speed;
    }

    if game.ball.x < game.paddle_2.x + game.paddle_2.width && game.ball.x > game.paddle_2.x && game.ball.y > game.paddle_2.y && game.ball.y < game.paddle_2.y + game.paddle_2.height {
        game.ball.x_speed = -game.ball.x_speed;
    }

    if game.paddle_1.y > 600.0 - game.paddle_1.height {
        game.paddle_1.y = 600.0 - game.paddle_1.height;
    }

    if game.paddle_1.y < 0.0 {
        game.paddle_1.y = 0.0;
    }

    if game.paddle_2.y > 600.0 - game.paddle_2.height {
        game.paddle_2.y = 600.0 - game.paddle_2.height;
    }

    if game.paddle_2.y < 0.0 {
        game.paddle_2.y = 0.0;
    }

    if game.scores.player_1 >= 10 {
        game.gameover(1);
    }

    if game.scores.player_2 >= 10 {
        game.gameover(2);
    }

    game.update();
}