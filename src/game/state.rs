use crate::game::board::Board;
use crate::game::player::*;

#[derive(Debug)]
pub struct State {
    pub board: Board,
    pub players: [Player; 2],
}

impl State {
    pub fn new(board: Vec<String>, players: [String; 2]) -> Self {
        let player1 = PlayerBuilder::new(players[0].clone())
            .piece('X')
            .start_location(0, 0)
            .build();

        let player2 = PlayerBuilder::new(players[1].clone())
            .piece('O')
            .start_location(69, 69)
            .build();
        Self {
            board: Board::new(),
            players: [player1, player2],
        }
    }
}
