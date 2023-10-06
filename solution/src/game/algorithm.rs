use crate::game::{Player, State};

impl State {
    pub fn place_piece(&self) {
        let board = &self.instructions.board;
        let (_width, _height) = board.dimensions;
        let (p1, p2) = Player::init(board);
        vertical_prio(&p1, &p2);
        horizontal_prio(&p1, &p2);

        let piece = &self.instructions.piece;
        let _borders = piece.borders();
    }
}

fn vertical_prio(p1: &Player, p2: &Player) -> bool {
    p1.bottom < p2.top
}
fn horizontal_prio(p1: &Player, p2: &Player) -> bool {
    p1.right < p2.left
}
