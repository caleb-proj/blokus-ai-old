use game::board::pieces::point::Point;
use game::board::pieces::corners::direction::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Corner {
    pub coordinates: Point,
    pub direction: Direction,
}

impl Corner {
    pub fn new(point: Point, direction: Direction) -> Self {
        Corner {
            coordinates: point,
            direction: direction,
        }
    }
    
    #[inline]
    pub fn block_position(&self) -> Point {
        let offsets = [Point::new(1, 1), Point::new(-1, 1), Point::new(1, -1), Point::new(-1, -1)];
        self.coordinates + offsets[self.direction as usize]
    }
}
