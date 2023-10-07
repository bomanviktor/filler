use crate::game::{Board, Piece};
use std::io;

pub type Dimensions = (isize, isize);
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Instructions {
    pub board: Board,
    pub piece: Piece,
}

impl Instructions {
    pub fn new() -> Self {
        let mut board = Vec::new();
        let mut piece = Vec::new();
        let piece_loops;
        loop {
            let instruction = instruction();
            if instruction.contains("exec") || instruction.contains("Anfield") {
                continue;
            }

            if instruction.contains("Piece") {
                piece_loops = instruction
                    .trim_end()
                    .trim_end_matches(':')
                    .split_whitespace()
                    .next_back()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                break;
            } else {
                board.push(instruction);
            }
        }
        for _ in 0..piece_loops {
            piece.push(instruction());
        }

        Self {
            board: Board::new(board),
            piece: Piece::new(piece),
        }
    }
}

pub fn instruction() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

impl Default for Instructions {
    fn default() -> Self {
        Self::new()
    }
}
