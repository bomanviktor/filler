use crate::game::{Board, Coordinates};

pub struct Player {
    pub coords: Vec<Coordinates>,
    pub top: isize,
    pub right: isize,
    pub bottom: isize,
    pub left: isize,
}

impl Player {
    pub fn new(
        coords: Vec<Coordinates>,
        top: isize,
        right: isize,
        bottom: isize,
        left: isize,
    ) -> Self {
        Self {
            coords,
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn init(board: &Board) -> (Self, Self) {
        let (p1_coords, p2_coords) = board.all_coords();
        let (p1_top, p2_top) = board.top_coords();
        let (p1_right, p2_right) = board.right_coords();
        let (p1_bottom, p2_bottom) = board.bottom_coords();
        let (p1_left, p2_left) = board.left_coords();
        (
            Player::new(p1_coords, p1_top, p1_right, p1_bottom, p1_left),
            Player::new(p2_coords, p2_top, p2_right, p2_bottom, p2_left),
        )
    }
}