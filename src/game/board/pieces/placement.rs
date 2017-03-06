use game::board::pieces::point::Point;
use game::board::pieces::points::Points;

// Represents the bitfield changes necessary to place a piece at a certain
// location. 
#[derive(Copy, Clone)]
pub struct Placement {
    pub len: u8,
    pub bits: [(u8, u64); 3],
}

impl Placement {
    pub fn new() -> Self {
        Placement {
            len: 0,
            bits: [(0, 0); 3],
        }
    }

    pub fn from_points(points: &Points, index: usize) -> Self {
        // Check if out of bounds.
        if (index + points.width as usize) / 20 > index / 20 ||
           index / 20 + (points.height as usize) > 19 {
            return Placement::new();
        }

        let mut board: [u64; 7] = [0; 7];

        for &Point { x, y } in points.iter() {
            let index = (y as isize) * 20 + (x as isize);

            if index < 0 {
                return Placement::new();
            }

            let array_index = index / 64;
            let shift = index % 64;

            board[array_index as usize] |= 1 << shift;
        }

        let mut bits: [(u8, u64); 3] = [(0, 0); 3];
        let mut len = 0;

        for (i, &field) in board.iter().enumerate() {
            if field != 0 {
                bits[len] = (i as u8, field);
                len += 1;
            }
        }

        Placement {
            len: len as u8,
            bits: bits,
        }
    }
}
