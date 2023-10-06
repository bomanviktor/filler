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
        self.update_score();
        self.round += 1;
        self.place_piece();
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
