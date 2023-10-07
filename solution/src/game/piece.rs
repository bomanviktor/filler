use crate::game::{Coordinates, Dimensions};

#[derive(Debug, Clone, Default)]
pub struct Piece {
    pub dimensions: Dimensions,
    pub shape: Vec<String>,
}

impl Piece {
    pub fn new(rows: Vec<String>) -> Self {
        let mut piece = Self {
            dimensions: Dimensions::default(),
            shape: Vec::new(),
        };

        rows.iter().for_each(|r| piece.shape(r));
        piece.dimensions();
        piece
    }

    pub fn shape(&mut self, s: &str) {
        self.shape.push(s.trim_end().to_owned())
    }

    pub fn borders(&self) -> Vec<Coordinates> {
        let mut borders = Vec::new();
        for (y, row) in self.shape.iter().enumerate() {
            if !row.contains('O') {
                continue;
            }
            if row.find('O').unwrap() == row.rfind('O').unwrap() {
                borders.push(Coordinates::new(row.find('O').unwrap(), y));
            } else {
                borders.push(Coordinates::new(row.find('O').unwrap(), y));
                borders.push(Coordinates::new(row.rfind('O').unwrap(), y));
            }
        }
        borders
    }

    pub fn dimensions(&mut self) {
        self.dimensions = (
            self.shape[0].len() as isize - 1,
            self.shape.len() as isize - 1,
        )
    }

    pub fn width(&self) -> isize {
        let mut min = self.dimensions.0;
        let mut max = 0;

        for coords in self.borders() {
            if coords.x > max {
                max = coords.x;
            }

            if coords.x < min {
                min = coords.x;
            }
        }
        max - min
    }

    pub fn height(&self) -> isize {
        let mut min = self.dimensions.1;
        let mut max = 0;

        for coords in self.borders() {
            if coords.y > max {
                max = coords.y;
            }

            if coords.y < min {
                min = coords.y;
            }
        }
        max - min
    }

    pub fn offset(&self) -> (isize, isize) {
        let (mut x, mut y) = self.dimensions;
        for coords in self.borders() {
            if coords.x < x {
                x = coords.x;
            }
            if coords.y < y {
                y = coords.y;
            }
        }
        (x, y)
    }
}
