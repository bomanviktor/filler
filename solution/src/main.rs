use filler::game::{instruction, Instructions, State};
fn main() {
    let player = match instruction()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()[2]
    {
        "p1" => 1,
        _ => 2,
    };
    loop {
        State::new(Instructions::new(), player).place_piece();
    }
}
