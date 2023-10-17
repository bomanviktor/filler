use filler::game::{Board, Piece, Player, player, State};
fn main() {
    let player = player();
    let board = Board::new();
    let players = Player::init(&board);
    let mut state = State::new(board, player, players);
    loop {
        state.place_piece(&Piece::new());
        state.update();
    }
}
