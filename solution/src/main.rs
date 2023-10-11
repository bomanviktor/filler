use filler::game::{Instructions, State};
fn main() {
    loop {
        State::new(Instructions::new()).place_piece();
    }
}
