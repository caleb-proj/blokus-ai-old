use std::ops::Add;
use std::ops::Sub;

// A simple abstraction over a coordinate. Probably redundant.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    #[inline]
    pub fn new(x: i8, y: i8) -> Self {
        Point {
            x: x as i8,
            y: y as i8,
        }
    }

    #[inline]
    pub fn from_index(index: usize) -> Self {
        Point {
            x: (index % 20) as i8,
            y: (index / 20) as i8,
        }
    }

    #[inline]
    pub fn to_index(&self) -> usize {
        (self.y as usize) * 20 + (self.x as usize)
    }
}

impl Add for Point {
    type Output = Point;

    #[inline]
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    #[inline]
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

