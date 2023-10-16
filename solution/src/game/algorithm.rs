use crate::game::{Coordinates, Piece, State};

type Distance = (isize, Coordinates, Coordinates);

impl State {
    pub fn place_piece(&self) {
        let piece = &self.instructions.piece;
        let coords = if self.player == 1 {
            &self.p1.coords
        } else {
            &self.p2.coords
        };

        let placeable_coords: Vec<Coordinates> = self
            .playable_coords(coords, piece)
            .into_iter()
            .filter(|c| self.placeable_coords(c, piece) && self.inside_board(c, piece))
            .collect::<Vec<Coordinates>>();

        if !placeable_coords.is_empty() {
            if self.instructions.board.last_piece(self.player).is_empty() {
                println!("{}", placeable_coords[0]);
            } else {
                println!("{}", self.shortest_dist(placeable_coords));
            }
        } else {
            println!("0 0");
        }
    }

    fn playable_coords(&self, coords: &Vec<Coordinates>, piece: &Piece) -> Vec<Coordinates> {
        let mut playable_coords = Vec::new();
        let (left, right, top, bottom) = self.iterable_coords(coords);
        let (piece_width, piece_height) = piece.dimensions;
        for y in (top - piece_height)..=bottom {
            for x in (left - piece_width)..=right {
                playable_coords.push(Coordinates::new(x, y));
            }
        }
        playable_coords
    }

    fn inside_board(&self, coord: &Coordinates, piece: &Piece) -> bool {
        let (offset_x, offset_y) = piece.offset();
        let (width, height) = self.instructions.board.dimensions;
        coord.x + piece.left() >= 0
            && coord.x + piece.width() + offset_x <= width
            && coord.y + piece.top() >= 0
            && coord.y + piece.height() + offset_y <= height
    }
    pub fn placeable_coords(&self, c: &Coordinates, piece: &Piece) -> bool {
        let mut overlapping_self = 0;
        for coord in piece.borders() {
            let x = c.x + coord.x;
            let y = c.y + coord.y;
            if x < 0 || y < 0 {
                continue;
            }
            let placement = Coordinates::new(x, y);
            if self.player == 1 {
                overlapping_self += self
                    .p1
                    .coords
                    .iter()
                    .filter(|&placed| placed.eq(&placement))
                    .count();
                if self.p2.coords.iter().any(|placed| placed.eq(&placement)) {
                    return false;
                }
            } else {
                overlapping_self += self
                    .p2
                    .coords
                    .iter()
                    .filter(|&placed| placed.eq(&placement))
                    .count();

                if self.p1.coords.iter().any(|placed| placed.eq(&placement)) {
                    return false;
                }
            }
        }
        overlapping_self == 1
    }

    fn iterable_coords(&self, coords: &Vec<Coordinates>) -> (isize, isize, isize, isize) {
        (
            self.left_coord(coords),
            self.right_coord(coords),
            self.top_coord(coords),
            self.bottom_coord(coords),
        )
    }
    fn top_coord(&self, coords: &Vec<Coordinates>) -> isize {
        let mut y = self.instructions.board.dimensions.1;
        for c in coords {
            if c.y < y {
                y = c.y;
            }
        }
        y
    }
    fn bottom_coord(&self, coords: &Vec<Coordinates>) -> isize {
        let mut y = 0;
        for c in coords {
            if c.y > y {
                y = c.y;
            }
        }
        y
    }
    fn left_coord(&self, coords: &Vec<Coordinates>) -> isize {
        let mut x = self.instructions.board.dimensions.0;
        for c in coords {
            if c.x < x {
                x = c.x;
            }
        }
        x
    }
    fn right_coord(&self, coords: &Vec<Coordinates>) -> isize {
        let mut x = 0;
        for c in coords {
            if c.x > x {
                x = c.x;
            }
        }
        x
    }
    fn shortest_dist(&self, self_coords: Vec<Coordinates>) -> Coordinates {
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
                if c1.calc_dist(c2) <= dist && c1.calc_dist(c2) >= 0 {
                    dist = c1.calc_dist(c2);
                    p1 = c1.clone();
                    p2 = c2.clone();
                }
            }
            distances.push((dist, p1, p2) as Distance)
        }
        distances.sort_by_key(|dist| dist.0);
        distances[0].1.clone()
    }
}
