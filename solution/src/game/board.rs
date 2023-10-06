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

    fn dimensions(&mut self) {
        self.dimensions = (self.anfield[0].len(), self.anfield.len())
    }

    pub fn get_self_coords(&self) -> Coordinates {
        for (y, row) in self.anfield.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch.eq(&'$') || ch.eq(&'s') {
                    return Coordinates::new(x, y);
                }
            }
        }
        Coordinates::default()
    }
}
