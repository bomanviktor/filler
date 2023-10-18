use crate::game::{Coordinates, Piece, State};

type Distance = (f64, Coordinates);


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


        if let Some(block_coords) = self.block(piece) {
            let sorted_coords = self.sorted_distances(&block_coords);
            for c in &sorted_coords {
                if let Some(placement_coords) = self.can_place(c, piece) {
                    let placement = self.shortest_distance(&placement_coords);
                    self.insert(&placement, piece);
                    println!("{}", placement);
                    return;
                }
            }

        }

        let sorted_coords = self.sorted_distances(&self.get_playable_coords().0);
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

    fn block(&self, p: &Piece) -> Option<Vec<Coordinates>> {
        let (self_left, other_left) = self.left();
        let (self_right, other_right) = self.right();
        let (self_top, other_top) = self.top();
        let (self_bot, other_bot) = self.bottom();

        if !p.wide() {
            return None;
        }

        let width = self.board.width();
        let margin = width / 5;

        if (0..margin).contains(&(self_left - p.width())){
            return Some(
                self.get_playable_coords().0
                    .into_iter()
                    .filter(|c| c.x == self_left)
                    .collect()
            )
        }

        if (width - margin - 1..width).contains(&(self_right + p.width())) {
            return Some(
                self.get_playable_coords().0
                    .into_iter()
                    .filter(|c| c.x == self_right)
                    .collect()
            )
        }

            None
            /*
            let top_diff = if self_top <= other_top {
                (self_top - other_top).abs()
            } else {
                0
            };

            let bot_diff = if self_bot >= other_bot {
                (self_bot - other_bot).abs()
            } else {
                0
            };

            if top_diff + bot_diff == 0 {
                return None;
            }

            if top_diff > bot_diff {
                Some(
                    self.get_playable_coords().0
                        .into_iter()
                        .filter(|c| c.y < self_top + p.height())
                        .collect()
                )
            } else {
                Some(
                    self.get_playable_coords().0
                        .into_iter()
                        .filter(|c| c.y > self_bot - p.height())
                        .collect()
                )
            }

             */

    }

    fn sorted_distances(&self, self_coords: &[Coordinates]) -> Vec<Coordinates> {
        let other_coords= self.get_playable_coords().1;
        let mut distances: Vec<Distance> = Vec::new();
        for c1 in self_coords.into_iter() {
            let mut dist = f64::MAX;
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
        let in_range_coords = self.in_range_coords(c, p);

        let mut placeable = Vec::new();
        let borders = p.borders();

        for overlap in &borders {
            let mut allowed_to_place = true;
            for piece_coords in borders.iter().filter(|&coord| coord != overlap) {
                let x = c.x + piece_coords.x - overlap.x;
                let y = c.y + piece_coords.y - overlap.y;

                let piece_coordinate = Coordinates::new(x, y);
                if self.out_of_bounds(&piece_coordinate)
                    || in_range_coords.contains(&piece_coordinate) {
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

        let mut shortest_distance = f64::MAX;
        let mut closest_coord = Coordinates::default();

        for coord in other_coords {
            for piece_coord in coords {
                let distance = coord.calc_dist(piece_coord);

                if distance < shortest_distance {
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

    fn get_player_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.coords.clone(), self.p2.coords.clone())
        } else {
            (self.p2.coords.clone(), self.p1.coords.clone())
        }
    }

    fn in_range_coords(&self, placement: &Coordinates, p: &Piece) -> Vec<Coordinates> {
        let (p1_coords, p2_coords) = self.get_player_coords();
            p1_coords
            .into_iter()
            .chain(p2_coords.into_iter())
            .filter(|c| (
                (placement.x - p.width()..= placement.x +p.width()).contains(&c.x) &&
                    (placement.y - p.height()..= placement.y + p.height()).contains(&c.y)
                ))
            .collect()
    }

    fn get_playable_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.playable.clone(), self.p2.playable.clone())
        } else {
            (self.p2.playable.clone(), self.p1.playable.clone())
        }
    }
}
