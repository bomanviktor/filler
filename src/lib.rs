pub mod config {
    use macroquad::prelude::Conf;

    pub const WINDOW_SIZE: i32 = 1000;
    pub fn window_conf() -> Conf {
        Conf {
            window_title: "Smart-Road | Grit:lab".to_owned(),
            window_width: WINDOW_SIZE,
            window_height: WINDOW_SIZE,
            window_resizable: false,
            ..Default::default()
        }
    }
}

pub mod input {
    use macroquad::input::{is_key_pressed, KeyCode};

    pub fn handle_input() {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}

pub mod render {
    mod board;
    mod score;
}
pub mod game {
    mod state;

    pub mod board;

    mod algorithm;
    pub mod parse;

    pub use parse::*;
    pub mod player;
    pub use player::*;
}
