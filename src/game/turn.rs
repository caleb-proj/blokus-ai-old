// A representation of a single move of the game. It needs to be extremely
// space efficient since its the main data at each node of the monte carlo
// tree.

use game::color::Color;
use corner::Corner;
use point::Point;
use direction::Direction;

pub struct Turn {
    piece: u8,
    orientation: u8,
    corner: u16,
    connect: Point,
    color: Color,
}

impl Turn {
    #[inline]
    pub fn new(piece: u8, orientation: u8, corner: usize, connect: Point, color: Color) -> Self {
        Turn {
            piece: piece,
            orientation: orientation,
            corner: corner as u16,
            connect: connect,
            color: color,
        }
    }

    // #[inline]
    // pub fn shape(&self) -> &Orientation {
    // let corner = self.0 & !(1 << 10);
    // let piece = (self.0 >> 9) & !(1 << 10);
    // let orientation = (self.0 >> 14) & !(1 << 15);
    // pieces::piece(piece as usize).orientation(orientation as usize)
    // }
    //

    // #[inline]
    // pub fn corner(&self) -> usize {
    // (self.0 & !(1 << 10)) as usize
    // }
    //
}
