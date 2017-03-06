use game::board::pieces::points::Points;
use game::board::pieces::placement::Placement;

pub struct PlacementMap {
    maps: [Placement; 400],
}

type BitfieldIter<'a> = ::std::iter::Take<::std::slice::Iter<'a, (u8, u64)>>;

impl PlacementMap {
    pub fn new() -> Self {
        PlacementMap {
            maps: [Placement::new(); 400],
        }
    }

    pub fn from_points(points: &Points) -> Self {
        let mut maps = [Placement::new(); 400];

        for (i, map) in maps.iter_mut().enumerate() {
            *map = Placement::from_points(points, i);
        }

        PlacementMap {
            maps: maps,
        }
    }

    #[inline]
    pub fn intersection_bitfields(&self, index: usize) -> BitfieldIter {
        let map = &self.maps[index];
        map.bits.iter().take(map.len as usize)
    }
}
