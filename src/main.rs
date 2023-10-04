use filler::config::window_conf;
use filler::input::handle_input;
use macroquad::prelude::*;
use filler::game::instruction;


#[macroquad::main(window_conf)]
async fn main() {
    loop {
        println!("26 33");
        clear_background(BLACK);
        handle_input();
        println!("{}", instruction());
        next_frame().await
    }
}
 