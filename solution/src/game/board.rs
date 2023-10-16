use crate::game::{Dimensions, dimensions, instruction};

use super::Coordinates;

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub dimensions: Dimensions,
    pub anfield: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Self {
        let dimensions = dimensions(&instruction());
        let mut board = Self {
            dimensions,
            anfield: Vec::with_capacity(dimensions.1 as usize),
        };

        for _ in 0..=dimensions.1 as usize {
            let instruction = &instruction();
            if  instruction.contains(" .")
                || instruction.contains(" @")
                || instruction.contains(" $")
                || instruction.contains(" a")
                || instruction.contains(" s") {
                board.anfield(instruction);
            }
        }
        board
    }



    pub fn anfield(&mut self, s: &str) {
        let row = s
            .split_whitespace()
            .next_back()
            .unwrap()
            .trim_end();
        self.anfield.push(row.chars().collect())
    }

    pub fn all_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        let mut p1_coords = Vec::new();
        let mut p2_coords = Vec::new();
        for (y, row) in self.anfield.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c.eq(&'@') || c.eq(&'a') {
                    p1_coords.push(Coordinates::new(x as isize, y as isize));
                }
                if c.eq(&'$') || c.eq(&'s') {
                    p2_coords.push(Coordinates::new(x as isize, y as isize));
                }
            }
        }

        (p1_coords, p2_coords)
    }

    pub fn empty_neighbor(&self, c: &Coordinates) -> bool {
        let x = c.x as usize;
        let y = c.y as usize;
        if x <= 0 || y <= 0 {
            return true;
        }
        for row in self.anfield.iter().skip(y - 1).take(3) {
            if row.iter().skip(x - 1).take(3).any(|c| c == &'.') {
                return true;
            }
        }
        false
    }

    pub fn last_piece(&self, player: u8) -> Vec<Coordinates> {
        let mut last_piece = Vec::new();
        if player == 1 {
            for (y, row) in self.anfield.iter().enumerate() {
                for (x, ch) in row.iter().enumerate() {
                    if ch.eq(&'s') {
                        last_piece.push(Coordinates::new(x as isize, y as isize));
                    }
                }
            }
        } else {
            for (y, row) in self.anfield.iter().enumerate() {
                for (x, ch) in row.iter().enumerate() {
                    if ch.eq(&'a') {
                        last_piece.push(Coordinates::new(x as isize, y as isize));
                    }
                }
            }
        }
        last_piece
    }

    pub fn width(&self) -> isize {
        self.dimensions.0
    }
    pub fn height(&self) -> isize {
        self.dimensions.1
    }

    pub fn top_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.all_coords();
        (p1_coords[0].y, p2_coords[0].y)
    }

    pub fn bottom_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.all_coords();
        (
            p1_coords.iter().next_back().unwrap().y,
            p2_coords.iter().next_back().unwrap().y,
        )
    }

    pub fn left_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.all_coords();

        let mut p1_left = self.width();
        for coordinates in p1_coords {
            if coordinates.x < p1_left {
                p1_left = coordinates.x;
            }
        }

        let mut p2_left = self.width();
        for coordinates in p2_coords {
            if coordinates.x < p2_left {
                p2_left = coordinates.x;
            }
        }

        (p1_left, p2_left)
    }

    pub fn right_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.all_coords();

        let mut p1_right = 0;
        for coordinates in p1_coords {
            if coordinates.x > p1_right {
                p1_right = coordinates.x;
            }
        }

        let mut p2_right = 0;
        for coordinates in p2_coords {
            if coordinates.x > p2_right {
                p2_right = coordinates.x;
            }
        }

        (p1_right, p2_right)
    }
}
