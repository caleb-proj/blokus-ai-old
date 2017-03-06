pub mod corner;
pub mod direction;

use std::default::Default;
use game::board::pieces::point::Point;
use game::board::pieces::corners::corner::Corner;
use game::board::pieces::corners::direction::Direction;

pub struct Corners {
    corners: [CornerArray; 4],
}

impl Corners {
    pub fn new() -> Self {
        Corners { corners: Default::default() }
    }

    pub fn add_corner(&mut self, corner: Point, direction: Direction) {
        self.corners[direction as usize].add_corner(corner);
    }

    pub fn get_corners(&self, direction: Direction) -> &[Point] {
        self.corners[direction as usize].as_slice()
    }

    pub fn iter(&self) -> CornerIterator {
        CornerIterator {
            corners: [self.corners[0].as_slice().iter(),
                      self.corners[1].as_slice().iter(),
                      self.corners[2].as_slice().iter(),
                      self.corners[3].as_slice().iter()],
            which: Direction::NW,
        }
    }
}

#[derive(Default)]
struct CornerArray {
    corners: [Point; 8],
    len: u8,
}

impl CornerArray {
    fn add_corner(&mut self, corner: Point) {
        debug_assert!(self.len <= 8);

        self.corners[self.len as usize] = corner;
        self.len += 1;
    }

    fn as_slice(&self) -> &[Point] {
        &self.corners[0..self.len as usize]
    }
}

// Iteration!

type PointIterator<'a> = ::std::slice::Iter<'a, Point>;

pub struct CornerIterator<'a> {
    corners: [PointIterator<'a>; 4],
    which: Direction,
}

impl<'a> Iterator for CornerIterator<'a> {
    type Item = Corner;

    fn next(&mut self) -> Option<Corner> {
        let next = self.corners[self.which as usize].next();

        match next {
            Some(&point) => Some(Corner::new(point, self.which)),

            None => {
                if self.which == Direction::SE {
                    None
                } else {
                    self.which = self.which.next();
                    self.next()
                }
            }
        }
    }
}
