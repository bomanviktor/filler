pub struct PlayerBuilder {
    name: String,
    start_location: Option<(u64, u64)>,
    piece: Option<char>,
}

impl PlayerBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_location: None,
            piece: None,
        }
    }

    pub fn start_location(&mut self, x: u64, y: u64) -> &mut PlayerBuilder {
        self.start_location = Some((x, y));
        self
    }

    pub fn piece(&mut self, piece: char) -> &mut PlayerBuilder {
        self.piece = Some(piece);
        self
    }

    pub fn build(&self) -> Player {
        Player {
            name: self.name.clone(),
            score: 0,
            start_location: self.start_location.unwrap(),
            piece: self.piece.unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u64,
    pub start_location: (u64, u64),
    pub piece: char,
}
