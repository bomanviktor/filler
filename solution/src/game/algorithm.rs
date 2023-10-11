use crate::game::{Coordinates, Piece, Player, State};
#[allow(dead_code)] // TODO: Remove
#[derive(Debug, Clone, PartialEq)]
pub enum Placement {
    Above,
    Below,
    Left,
    Right,
}

type Distance = (isize, Coordinates, Coordinates);
/*
const PLACEMENTS: [Placement; 4] = [
    Placement::Above,
    Placement::Below,
    Placement::Left,
    Placement::Right,
];
 */
impl State {
    pub fn place_piece(&self) {
        let board = &self.instructions.board;
        let (width, height) = board.dimensions;

        horizontal_prio(&self.p1, &self.p2);

        let piece = &self.instructions.piece;

        if self.bottom_coord().y + piece.height() <= height {
            println!("{}", piece.placement_coord(self.bottom_coord()));
            return;
        } else {
            for c in &self.p1.coords {
                if self.can_place(piece, c) {
                    println!("{}", piece.placement_coord(c));
                    return;
                }
            }
        }
    }

    fn inside_board(&self, c: &Coordinates, piece: &Piece, placement: Placement) -> bool {
        match placement {
            Placement::Above => c.y - piece.height() >= 0,
            Placement::Below => c.y + piece.height() <= self.instructions.board.height(),
            Placement::Left => c.x - piece.width() >= 0,
            Placement::Right => c.y + piece.width() >= self.instructions.board.width(),
        }
    }
    pub fn can_place(&self, piece: &Piece, c: &Coordinates) -> bool {
        let (offset_x, offset_y) = piece.offset();
        let mut overlapping = 0;

        for coord in piece.borders() {
            let x = c.x + coord.x - offset_x;
            let y = c.y + coord.y - offset_y;
            let placement = Coordinates::new(x, y);
            overlapping += self.p1.coords
                .iter()
                .chain(self.p2.coords.iter())
                .filter(|&placed| placed.eq(&placement))
                .count();
        }

        overlapping == 1
    }


    fn bottom_coord(&self) -> &Coordinates {
        self.p1.coords.last().unwrap()
    }
    fn shortest_dist(&self) -> Vec<Distance> {
        let mut distances = Vec::new();
        let p1_coords = &self.p1.coords;
        let p2_coords = &self.p2.coords;

        for c1 in p1_coords.clone() {
            let (mut dist, mut p1, mut p2): Distance =
                (999999999, Coordinates::default(), Coordinates::default());
            for c2 in p2_coords.clone() {
                if c1.calc_dist(&c2) < dist && c1.calc_dist(&c2) > 0 {
                    dist = c1.calc_dist(&c2);
                    p1 = c1.clone();
                    p2 = c2.clone();
                }
            }
            distances.push((dist, p1, p2) as Distance)
        }

        distances.sort_by_key(|dist| dist.0);
        distances
    }
}

fn vertical_prio(p1: &Player, p2: &Player) -> bool {
    p1.bottom < p2.top
}
fn horizontal_prio(p1: &Player, p2: &Player) -> bool {
    p1.right < p2.left
}
