use super::Instructions;

#[derive(Debug)]
pub struct State {
    pub instructions: Instructions,
    pub score: (u64, u64),
    pub round: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl State {
    pub fn new(instructions: Instructions) -> Self {
        let mut state = Self {
            instructions: instructions.clone(),
            score: (1, 1),
            round: 0,
        };
        state.update(instructions);
        state
    }

    pub fn update(&mut self, instructions: Instructions) {
        self.instructions = instructions;
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
        self.round += 1;
    }

    pub fn place_piece(&self) {
        let piece = &self.instructions.piece;
        let board = &self.instructions.board;
        let (_width, _height) = self.instructions.board.dimensions;
        println!("{:?}", board.right_coords());
        // let mut opponent_placed_coords: Coordinates = Coordinates::default();

        // for (y, row) in board.iter().enumerate() {
        //     for (x, ch) in row.chars().enumerate() {
        //         if ch.eq(&'@') || ch.eq(&'a') {
        //             opponent_placed_coords = Coordinates::new(x, y);
        //         }
        //     }
        // }

        let self_placed_coords: Coordinates = self.instructions.board.get_self_coords();
        let mut bottom_piece_coords: Coordinates = Coordinates::default();

        for (y, row) in piece.shape.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch.eq(&'O') {
                    bottom_piece_coords = Coordinates::new(x, y);
                }
            }
        }

        if (self_placed_coords.y - bottom_piece_coords.y) <= 0 {
            println!(
                "{} {}",
                self_placed_coords.x + bottom_piece_coords.x,
                self_placed_coords.y
            )
        } else {
            println!(
                "{} {}",
                self_placed_coords.x - bottom_piece_coords.x,
                self_placed_coords.y - bottom_piece_coords.y
            )
        }
    }
}
