use crate::game::Dimensions;

use super::Coordinates;

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub dimensions: Dimensions,
    pub anfield: Vec<String>,
}

impl Board {
    pub fn new(rows: Vec<String>) -> Self {
        let mut board = Self {
            dimensions: Dimensions::default(),
            anfield: Vec::new(),
        };

        rows.iter()
            .filter(|&r| {
                r.contains(" .")
                    || r.contains(" @")
                    || r.contains(" $")
                    || r.contains(" a")
                    || r.contains(" s")
            })
            .for_each(|r| board.anfield(r));

        board.dimensions();
        board
    }

    pub fn anfield(&mut self, s: &str) {
        let row = s
            .split_whitespace()
            .next_back()
            .unwrap()
            .trim_end()
            .to_owned();
        self.anfield.push(row)
    }



    pub fn all_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        let mut p1_coords = Vec::new();
        let mut p2_coords = Vec::new();

        for (y, row) in self.anfield.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c.eq(&'@') || c.eq(&'a') {
                    p1_coords.push(Coordinates::new(x, y));
                }
                if c.eq(&'$') || c.eq(&'s') {
                    p2_coords.push(Coordinates::new(x, y));
                }
            }
        }
        (p1_coords, p2_coords)
    }

    pub fn dimensions(&mut self) {
        self.dimensions = (self.anfield[0].len() - 1, self.anfield.len() - 1)
    }

    pub fn width(&self) -> isize {
        self.dimensions.0 as isize
    }
    pub fn height(&self) -> isize {
        self.dimensions.1 as isize
    }

    pub fn top_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.get_coordinates();
        (p1_coords[0].y, p2_coords[0].y)
    }

    pub fn bottom_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.get_coordinates();
        (
            p1_coords.iter().next_back().unwrap().y,
            p2_coords.iter().next_back().unwrap().y,
        )
    }

    pub fn left_coords(&self) -> (isize, isize) {
        let (p1_coords, p2_coords) = self.get_coordinates();

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
        let (p1_coords, p2_coords) = self.get_coordinates();

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
