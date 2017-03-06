use shape::Shape;
use ::std::default;

#[derive(Default, Copy, Clone)]
pub struct BitBoard {
    bits: [u64; 7],
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard { bits: [0; 7] }
    }

    #[inline]
    pub fn index(&self, index: usize) -> bool {
        let array_index = index / 64;
        let shift = index % 64;

        ((self.bits[array_index] >> shift) & 1) == 1
    }

    #[inline]
    pub fn index_xy(&self, x: usize, y: usize) -> bool {
        self.index(y * 20 + x)
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        let array_index = index / 64;
        let shift = index % 64;

        self.bits[array_index] |= 1 << shift;
    }

    #[inline]
    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.set(y * 20 + x)
    }

    #[inline]
    pub fn shape_intersects(&self, shape: &Shape, index: usize) -> bool {
        for &(i, bits) in shape.intersection_bitfields(index) {
            if bits & unsafe { self.bits.get_unchecked(i as usize) } != 0 {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn place_shape(&mut self, shape: &Shape, index: usize) {
        for &(i, bits) in shape.intersection_bitfields(index) {
            unsafe { *self.bits.get_unchecked_mut(i as usize) |= bits };
        }
    }
}

