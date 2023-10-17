use std::ops::ControlFlow::Continue;
use crate::game::{Coordinates, Piece, State};

type Distance = (isize, Coordinates);

impl State {
    pub fn place_piece(&mut self, piece: &Piece) {
        if self.endgame {
            for c in &self.get_playable_coords().0 {
                if let Some(placement_coords) = self.can_place(c, piece) {
                    println!("{}", placement_coords[0]);
                    return;
                }
            }
            println!("0 0");
            return;
        }


        let sorted_coords = self.sorted_distances();
        for c in &sorted_coords {
            if let Some(placement_coords) = self.can_place(c, piece) {

                let placement = self.shortest_distance(&placement_coords);
                self.insert(&placement, piece);
                println!("{}", placement);
                return;
            }
        }
        println!("0 0");
    }

    fn sorted_distances(&self) -> Vec<Coordinates> {
        let (self_coords, other_coords) = self.get_playable_coords();
        let mut distances: Vec<Distance> = Vec::new();
        for c1 in self_coords.into_iter() {
            let mut dist = isize::MAX;
            for c2 in &other_coords {
                let current_distance = c1.calc_dist(&c2);
                if current_distance < dist {
                    dist = current_distance;
                }
            }
            distances.push((dist, c1.clone()));
        }
        distances
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        distances
            .into_iter()
            .map(|d| d.1)
            .collect()
    }

    fn can_place(&self, c: &Coordinates, p: &Piece) -> Option<Vec<Coordinates>> {
        let (self_coords, other_coords) = self.get_player_coords();

        let mut placeable = Vec::new();
        let borders = p.borders();

        for overlap in &borders {
            let mut allowed_to_place = true;
            for piece_coords in borders.iter().filter(|&coord| coord != overlap) {
                let x = c.x + piece_coords.x - overlap.x;
                let y = c.y + piece_coords.y - overlap.y;

                let piece_coordinate = Coordinates::new(x, y);
                if self.out_of_bounds(&piece_coordinate)
                    || other_coords.contains(&piece_coordinate)
                    || self_coords.contains(&piece_coordinate)
                    {
                    allowed_to_place = false;
                    break
                }
            }
            if allowed_to_place {
                placeable.push(Coordinates::new(c.x - overlap.x, c.y - overlap.y));
            }
        }

        if placeable.is_empty() {
            None
        } else {
            Some(placeable)
        }
    }

    fn shortest_distance(&self, coords: &[Coordinates]) -> Coordinates {
        let other_coords = if self.player == 1 {
            &self.p2.playable
        } else {
            &self.p1.playable
        };

        let mut shortest_distance = 999;
        let mut closest_coord = Coordinates::default();

        for coord in other_coords {
            for piece_coord in coords {
                let distance = coord.calc_dist(piece_coord);

                if distance <= shortest_distance {
                    shortest_distance = distance;
                    closest_coord = piece_coord.clone();
                }

            }
        }
        closest_coord
    }

    fn out_of_bounds(&self, coord: &Coordinates) -> bool {
        let (width, height) = self.board.dimensions;
        coord.x < 0
            || coord.x >= width
            || coord.y < 0
            || coord.y >= height
    }
    pub fn placeable(&self, c: &Coordinates, piece: &Piece) -> bool {
        let mut overlapping_self = 0;
        let (self_coords, other_coords) = self.get_player_coords();

        for coord in piece.borders() {
            let x = c.x + coord.x;
            let y = c.y + coord.y;
            if x < 0 || y < 0 {
                continue;
            }
            let placement = Coordinates::new(x, y);
            overlapping_self += self_coords
                .iter()
                .filter(|&placed| placed.eq(&placement))
                .count();

            if other_coords
                .iter()
                .any(|placed| placed.eq(&placement)) {
                return false;
            }

            if overlapping_self > 1 {
                return false;
            }
        }
        overlapping_self == 1
    }

    fn get_player_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.coords.clone(), self.p2.coords.clone())
        } else {
            (self.p2.coords.clone(), self.p1.coords.clone())
        }
    }

    fn get_playable_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.playable.clone(), self.p2.playable.clone())
        } else {
            (self.p2.playable.clone(), self.p1.playable.clone())
        }
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


}
