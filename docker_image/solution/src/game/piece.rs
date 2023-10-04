use crate::game::Dimensions;

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

    pub fn dimensions(&mut self) {
        self.dimensions = (self.shape[0].len(), self.shape.len())
    }

    pub fn is_wide(&self) -> bool {
        self.dimensions.0 > self.dimensions.1
    }

    pub fn is_tall(&self) -> bool {
        self.dimensions.0 < self.dimensions.1
    }

    pub fn is_square(&self) -> bool {
        self.dimensions.0 == self.dimensions.1
    }
}
