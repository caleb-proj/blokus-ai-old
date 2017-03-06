use std::mem;
use game::board::pieces::point::Point;
use game::board::pieces::corners::Corners;
use game::board::pieces::corners::direction::Direction;

// A list of points with a defined width and height.
// Storing the width and the height allows for the shape the points
// represent to easily be mirrored or rotated.
// As such, it's a useful intermediate representation of a piece.
#[derive(PartialEq, Eq, Clone)]
pub struct Points {
    pub points: Vec<Point>,
    pub width: u8,
    pub height: u8,
}

impl Points {
    pub fn new() -> Self {
        Points {
            points: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    // Creates the list of points from the default representation of a
    // piece.
    pub fn from_piece(piece: [u8; 49]) -> Self {
        let mut points = Points::new();

        for y in 0..7 {
            for x in 0..7 {
                if piece[y * 7 + x] == 1 {
                    points.add_point(x as i8, y as i8);
                }
            }
        }

        points
    }

    /*
    // Converts the points into a 7x7 bitfield representing where the
    // points were located.
    pub fn bitfield(&self) -> u64 {
        let mut field: u64 = 0;

        for &point in self.iter() {
            field |= 1 << (point.y * 7 + point.x);
        }

        field
    }
    */

    pub fn aura(&self) -> Points {
        let mut aura = Points::new();

        for &Point { x, y } in self.iter() {
            aura.add_point(x - 1, y - 1);
            aura.add_point(x + 1, y - 1);
            aura.add_point(x - 1, y + 1);
            aura.add_point(x + 1, y + 1);
        }

        aura
    }

    pub fn orientations(&self) -> Vec<Points> {
        let mut orientations: Vec<Points> = Vec::with_capacity(8);

        orientations.push(self.clone());
        orientations.push(self.flip_horizontal());
        orientations.push(self.flip_vertical());
        orientations.push(self.turn());

        let horizontal_symmetry = orientations[0] == orientations[1];
        let vertical_symmetry = orientations[0] == orientations[2];
        let absolute_symmetry = orientations[0] == orientations[3];

        if absolute_symmetry {
            orientations.truncate(1);
        } else if horizontal_symmetry && vertical_symmetry {
            orientations[1] = orientations[3].clone();
            orientations.truncate(2);
        } else if horizontal_symmetry {
            orientations[1] = orientations[2].clone();
            orientations[2] = orientations[3].clone();
            orientations[3] = orientations[3].flip_horizontal();
            orientations.truncate(4);
        } else if vertical_symmetry {
            orientations[2] = orientations[3].clone();
            orientations[3] = orientations[3].flip_vertical();
            orientations.truncate(4);
        } else {
            // They'll fix this eventually.
            let tmp = orientations[3].clone();
            orientations[3] = orientations[2].flip_horizontal();
            orientations.push(tmp);
            let tmp = orientations[4].flip_horizontal();
            orientations.push(tmp);
            let tmp = orientations[4].flip_vertical();
            orientations.push(tmp);
            let tmp = orientations[6].flip_horizontal();
            orientations.push(tmp);
        }

        orientations
    }

    fn flip_vertical(&self) -> Self {
        let mut copy = self.clone();

        for point in copy.iter_mut() {
            point.y = (self.height as i8) - point.y;
        }

        // Yeah... the technique of just straight up sorting the new list does
        // seem kinda slow. But the time it takes to generate all the pieces
        // remains <200ms unoptimized and <20ms optimized, so I guess it's not
        // that big a deal.
        copy.points.sort();

        copy
    }

    fn flip_horizontal(&self) -> Self {
        let mut copy = self.clone();

        for point in copy.iter_mut() {
            point.x = (self.width as i8) - point.x;
        }

        copy.points.sort();

        copy
    }

    fn turn(&self) -> Self {
        let mut copy = self.clone();

        for point in copy.iter_mut() {
            mem::swap(&mut point.x, &mut point.y);
        }

        mem::swap(&mut copy.height, &mut copy.width);
        copy.points.sort();

        copy
    }

    // Finding corners: now in O(n^2)!
    pub fn corners(&self) -> Corners {
        let mut corners = Corners::new();

        for &Point { x: x1, y: y1 } in self.iter() {
            let mut possible = [true, true, true, true];

            for &Point { x: x2, y: y2 } in self.iter() {
                let distance = (x2 - x1).abs() + (y2 - y1).abs();

                if distance == 1 {
                    if y1 == y2 {
                        if x1 > x2 {
                            possible[0] = false;
                            possible[2] = false;
                        } else {
                            possible[1] = false;
                            possible[3] = false;
                        }
                    } else {
                        if y1 < y2 {
                            possible[0] = false;
                            possible[1] = false;
                        } else {
                            possible[2] = false;
                            possible[3] = false;
                        }
                    }
                } else if distance == 2 && x1 != x2 && y1 != y2 {
                    if y2 < y1 {
                        if x2 < x1 {
                            possible[0] = false;
                        } else {
                            possible[1] = false;
                        }
                    } else {
                        if x1 < x2 {
                            possible[2] = false;
                        } else {
                            possible[3] = false;
                        }
                    }
                }
            }

            if possible[0] {
                corners.add_corner(Point::new(x1 - 1, y1 - 1), Direction::NW);
            }

            if possible[1] {
                corners.add_corner(Point::new(x1 + 1, y1 - 1), Direction::NE);
            }

            if possible[2] {
                corners.add_corner(Point::new(x1 + 1, y1 + 1), Direction::SE);
            }

            if possible[3] {
                corners.add_corner(Point::new(x1 - 1, y1 + 1), Direction::SW);
            }
        }

        corners
    }


    pub fn add_point(&mut self, x: i8, y: i8) {
        // We don't care about ordering and need to check if an element is
        // already in the list, which means that keeping it sorted and using
        // binary search is the most efficient method.

        let point = Point::new(x, y);

        if let Err(i) = self.points.binary_search(&point) {
            self.points.insert(i, point);

            if x > self.width as i8 {
                self.width = x as u8;
            }

            if y > self.height as i8 {
                self.height = y as u8;
            }
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<Point> {
        self.points.iter()
    }

    pub fn iter_mut(&mut self) -> ::std::slice::IterMut<Point> {
        self.points.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
