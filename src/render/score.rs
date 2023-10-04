use crate::config::WINDOW_SIZE;
use macroquad::color::{RED, WHITE};
use macroquad::prelude::{BLACK, BLUE};
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::text::draw_text;

// to be substituted with value from parse where bot-names get taken
const PLAYER1: &str = "Player1";
const PLAYER2: &str = "Player2";

const SCOREBOARD_HEIGHT: f32 = WINDOW_SIZE as f32 / 20.0;

const FONT_SIZE: f32 = 24.0;

const NAME_Y_POS: f32 = SCOREBOARD_HEIGHT / 2.0 + FONT_SIZE / 4.0;
const P1_X_POS: f32 = SCOREBOARD_HEIGHT;
const P2_X_POS: f32 = WINDOW_SIZE as f32 / 2.0 + P1_X_POS;

pub fn render_scoreboard() {
    draw_rectangle(0.0, 0.0, WINDOW_SIZE as f32, SCOREBOARD_HEIGHT, WHITE);
    draw_line(
        WINDOW_SIZE as f32 / 2.0,
        0.0,
        WINDOW_SIZE as f32 / 2.0,
        SCOREBOARD_HEIGHT,
        2.0,
        BLACK,
    );
    draw_text(PLAYER1, P1_X_POS, NAME_Y_POS, FONT_SIZE, RED);
    draw_text(PLAYER2, P2_X_POS, NAME_Y_POS, FONT_SIZE, BLUE);
}
