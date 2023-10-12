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
        let piece = &self.instructions.piece;
        let (piece_width, piece_height) = piece.dimensions;
        let (offset_x, offset_y) = piece.offset();

        let mut placeable_coords = Vec::new();
        for y in (0 - piece_height)..=(height - piece_height) {
            for x in (0 - piece_width)..=(width - piece_width) {
                let coord = Coordinates::new(x, y);
                if self.can_place(piece, &coord)
                    && coord.x + offset_x >= 0
                    && coord.x + piece_width <= width
                    && coord.y + offset_y >= 0
                    && coord.y + piece_height <= height {
                        placeable_coords.push(coord);
                }
            }
        }
        // println!("{placeable_coords:?}");
        println!("{}", self.shortest_dist(placeable_coords).1);

        /*
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
        */
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
        let mut overlapping_self = 0;
        for coord in piece.borders() {
            if self.player == 1 {
                let x = c.x + coord.x - offset_x;
                let y = c.y + coord.y - offset_y;
                let placement = Coordinates::new(x, y);
                overlapping_self += self.p1.coords
                    .iter()
                    .filter(|&placed| placed.eq(&placement))
                    .count();
                if self.p2.coords
                    .iter()
                    .any(|placed| placed.eq(&placement)) {
                    return false;
                }
            } else {
                let x = c.x + coord.x;
                let y = c.y + coord.y;
                let placement = Coordinates::new(x, y);
                overlapping_self += self.p2.coords
                    .iter()
                    .filter(|&placed| placed.eq(&placement))
                    .count();

                if self.p1.coords
                    .iter()
                    .any(|placed| placed.eq(&placement)) {
                    return false;
                }
            }
        }
        overlapping_self == 1
    }

    fn bottom_coord(&self) -> &Coordinates {
        if self.player == 1 {
            self.p1.coords.last().unwrap()
        } else {
            self.p2.coords.first().unwrap()
        }
    }
    fn shortest_dist(&self, self_coords: Vec<Coordinates>) -> Distance {
        let mut distances = Vec::new();

        let other_coords = if self.player == 1 {
            &self.p2.coords
        } else {
            &self.p1.coords
        };

        for c1 in self_coords {
            let (mut dist, mut p1, mut p2): Distance =
                (999999999, Coordinates::default(), Coordinates::default());
            for c2 in other_coords {
                if c1.calc_dist(&c2) < dist && c1.calc_dist(&c2) > 0 {
                    dist = c1.calc_dist(&c2);
                    p1 = c1.clone();
                    p2 = c2.clone();
                }
            }
            distances.push((dist, p1, p2) as Distance)
        }

        distances.sort_by_key(|dist| dist.0);
        distances[0].clone()
    }
}
