use filler::config::window_conf;
use filler::input::handle_input;
use filler::render::score::render_scoreboard;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        render_scoreboard();
        handle_input();
        next_frame().await
    }
}
