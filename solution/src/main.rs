use filler::game::{Instructions, State};
fn main() {
    let mut state = State::new(Instructions::new());
    loop {
        state.place_piece();
        state.update(Instructions::new());
    }
}
