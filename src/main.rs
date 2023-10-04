use filler::config::window_conf;
use filler::input::handle_input;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        handle_input();
        next_frame().await
    }
}
