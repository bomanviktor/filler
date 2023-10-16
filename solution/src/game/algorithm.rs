use crate::game::{Coordinates, Piece, State};

type Distance = (isize, Coordinates, Coordinates);

impl State {
    pub fn place_piece(&mut self, piece: &Piece) {
        let coords = if self.player == 1 {
            &self.p1.coords
        } else {
            &self.p2.coords
        };

        let placeable_coords: Vec<Coordinates> = self
            .playable_coords(coords, piece)
            .into_iter()
            .filter(|c| {
                self.board.empty_neighbor(c)
                    && self.inside_board(c, piece)
                    && self.placeable(c, piece)
                })
            .collect::<Vec<Coordinates>>();

        if !placeable_coords.is_empty() {
            let placed_coords = self.shortest_dist(placeable_coords, piece);
            self.insert(&placed_coords, piece);
            println!("{}", placed_coords);
        } else {
            println!("0 0");
        }
    }

    fn playable_coords(&self, coords: &Vec<Coordinates>, piece: &Piece) -> Vec<Coordinates> {
        let mut playable_coords = Vec::new();
        let (left, right, top, bottom) = self.iterable_coords(coords);
        let (mut piece_width, mut piece_height) = piece.dimensions;
        piece_width -= piece.left();
        piece_height -= piece.top();
        for y in (top - piece_height)..=bottom {
            for x in (left - piece_width)..=right {
                playable_coords.push(Coordinates::new(x, y));
            }
        }
        playable_coords
    }

    fn inside_board(&self, coord: &Coordinates, piece: &Piece) -> bool {
        let (offset_x, offset_y) = piece.offset();
        let (width, height) = self.board.dimensions;
        coord.x + piece.left() >= 0
            && coord.x + piece.width() + offset_x < width
            && coord.y + piece.top() >= 0
            && coord.y + piece.height() + offset_y < height
    }
    pub fn placeable(&self, c: &Coordinates, piece: &Piece) -> bool {
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
            if overlapping_self > 1 {
                return false;
            }
        }
        overlapping_self == 1
    }

    fn iterable_coords(&self, coords: &[Coordinates]) -> (isize, isize, isize, isize) {
        (
            self.left_coord(coords),
            self.right_coord(coords),
            self.top_coord(coords),
            self.bottom_coord(coords),
        )
    }
    fn top_coord(&self, coords: &[Coordinates]) -> isize {
        let mut y = self.board.dimensions.1;
        for c in coords {
            if c.y < y {
                y = c.y;
            }
        }
        y
    }
    fn bottom_coord(&self, coords: &[Coordinates]) -> isize {
        let mut y = 0;
        for c in coords {
            if c.y > y {
                y = c.y;
            }
        }
        y
    }
    fn left_coord(&self, coords: &[Coordinates]) -> isize {
        let mut x = self.board.dimensions.0;
        for c in coords {
            if c.x < x {
                x = c.x;
            }
        }
        x
    }
    fn right_coord(&self, coords: &[Coordinates]) -> isize {
        let mut x = 0;
        for c in coords {
            if c.x > x {
                x = c.x;
            }
        }
        x
    }
    fn shortest_dist(&self, self_coords: Vec<Coordinates>, piece: &Piece) -> Coordinates {
        let mut distances = Vec::new();
        let other_coords = if self.player == 1 {
            &self.p2.coords
        } else {
            &self.p1.coords
        };

        for c1 in self_coords.iter().filter(|c| self.board.empty_neighbor(c)) {

            let (mut dist, mut p1, mut p2): Distance =
                (99999, Coordinates::default(), Coordinates::default());

            for c2 in other_coords
                .iter()
                .filter(|c| self.board.empty_neighbor(c)) {

                let placement = piece.placement_coord(&c1);
                if placement.calc_dist(c2) < dist {
                        dist = placement.calc_dist(c2);
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
