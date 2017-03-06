use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Direction {
    NW = 0, // Northwest (up left)
    NE, // Northeast (up right)
    SW, // Southwest (down left)
    SE, // Southeast (down right)
}

impl Direction {
    #[inline]
    pub fn opposite(self) -> Direction {
        unsafe {
            // I don't trust the optimizer.
            mem::transmute(3 - (self as u8))
        }
    }

    #[inline]
    pub fn next(self) -> Direction {
        debug_assert!(self != Direction::SE);

        unsafe { mem::transmute((self as u8) + 1) }
    }

    #[inline]
    pub fn up(self) -> bool {
        self <= Direction::NE
    }
}
