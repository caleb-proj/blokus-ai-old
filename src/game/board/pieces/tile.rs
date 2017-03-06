use game::board::pieces::placement_map::PlacementMap;
use game::board::pieces::points::Points;
use game::board::pieces::corners::Corners;
use game::board::pieces::corners::CornerIterator;
use game::board::pieces::corners::direction::Direction;
use game::board::pieces::point::Point;
use rand;

pub struct Tile {
    pub orientations: [Shape; 8],
    pub num_orientations: u8,
    pub score: u8,
    pub index: u8,
}

type ShapeIterator<'a> = ::std::iter::Take<::std::slice::Iter<'a, Shape>>;

impl Tile {
    pub fn new(piece: [u8; 49], index: usize) -> Self {
        // Rust really needs to make a not-shitty way to create fixed-size
        // arrays of non-copy types.
        let mut orientations = [Shape::new(),
                                Shape::new(),
                                Shape::new(),
                                Shape::new(),
                                Shape::new(),
                                Shape::new(),
                                Shape::new(),
                                Shape::new()];

        let piece = Points::from_piece(piece);

        let orientation_points = piece.orientations();
        let num_orientations = orientation_points.len() as u8;
        let score = piece.len() as u8;

        let mut index = 0;

        for (orientation, points) in orientations.iter_mut()
            .zip(orientation_points.iter()) {
            *orientation = Shape::from_points(points, index);
            index += 1;
        }

        Tile {
            orientations: orientations,
            num_orientations: num_orientations,
            score: score,
            index: index as u8,
        }
    }

    #[inline]
    pub fn orientations(&self) -> ShapeIterator {
        self.orientations.iter().take(self.num_orientations as usize)
    }

    #[inline]
    pub fn get_orientations(&self) -> &[Shape] {
        &self.orientations[0..(self.num_orientations as usize)]
    }

    #[inline]
    pub fn orientation(&self, index: usize) -> &Shape {
        &self.orientations[index]
    }
}

// Internal representation of a shape; that is, a contiguous set of tiles.
pub struct Shape {
    pub shape: PlacementMap,
    pub aura: PlacementMap,
    pub corners: Corners,
    pub index: u8,
}

impl Shape {
    fn new() -> Self {
        Shape {
            shape: PlacementMap::new(),
            aura: PlacementMap::new(),
            corners: Corners::new(),
            index: 0,
        }
    }

    fn from_points(points: &Points, index: usize) -> Self {
        Shape {
            shape: PlacementMap::from_points(points),
            aura: PlacementMap::from_points(&points.aura()),
            corners: points.corners(),
            index: index as u8,
        }
    }

    pub fn corners(&self) -> CornerIterator {
        self.corners.iter()
    }

    pub fn get_corners(&self, direction: Direction) -> &[Point] {
        self.corners.get_corners(direction)
    }
}
