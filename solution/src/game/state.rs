use super::Instructions;
use crate::game::Player;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct State {
    pub instructions: Instructions,
    pub score: (u64, u64),
    pub round: u64,
    pub p1: Player,
    pub p2: Player,
    pub player: u8,
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Coordinates {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn calc_dist(&self, other: &Coordinates) -> isize {
        if (self.x - other.x).abs() >= (self.y - other.y).abs() {
            (self.x - other.x).abs() - 1
        } else {
            (self.y - other.y).abs() - 1
        }
    }
}

impl State {
    pub fn new(instructions: Instructions, player: u8) -> Self {
        let mut state = Self {
            instructions: instructions.clone(),
            score: (1, 1),
            round: 0,
            p1: Player::default(),
            p2: Player::default(),
            player,
        };
        state.update(instructions);
        state
    }

    pub fn update(&mut self, instructions: Instructions) {
        self.instructions = instructions;
        self.update_score();
        self.round += 1;
        let (p1, p2) = Player::init(&self.instructions.board);
        self.p1 = p1;
        self.p2 = p2;
    }

    fn update_score(&mut self) {
        let mut p1_score = 0;
        let mut p2_score = 0;

        self.instructions.board.anfield.iter().for_each(|row| {
            for c in row.chars() {
                if c.eq(&'@') || c.eq(&'a') {
                    p1_score += 1;
                }
                if c.eq(&'$') || c.eq(&'s') {
                    p2_score += 1;
                }
            }
        });

        self.score = (p1_score, p2_score);
    }
}
