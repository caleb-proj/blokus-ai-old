pub mod pieces;

/*
use std::clone::Clone;
use self::bitboard::BitBoard;
use color::Color;
use shape::Shape;
use pieces;
use point::Point;
use direction::Direction;
use piece::Orientation;
use corner::Corner;
use rand;

#[derive(Default)]
pub struct Board {
    pub occupied: [BitBoard; 4],
    pub auras: [BitBoard; 4],
    pub corners: [Vec<Corner>; 4],
}

impl Board {
    pub fn new() -> Self {
        // The initial corners a player may play at.
        let corners = vec![Corner::new(Point::new(-1, -1), Direction::SE), Corner::new(Point::new(20, -1), Direction::SW), Corner::new(Point::new(-1, 20), Direction::NE), Corner::new(Point::new(20, 20), Direction::NW)];

        Board {
            occupied: Default::default(),
            auras: Default::default(),
            corners: [corners.clone(), corners.clone(), corners.clone(), corners.clone()],
        }
    }

    // Makes appropriate board changes for a turn, but only if the move is
    // legal. Returns a boolean which represents whether or not the move was
    // made.

    #[inline]
    pub fn do_move_if_legal(&mut self,
                            orientation: &Orientation,
                            corner_index: usize,
                            connect: Point,
                            color: Color)
                            -> bool {
        let corner = self.get_corner(corner_index, color);
        let index = Board::get_index(corner, connect);

        if self.is_illegal(index, &orientation.shape, color) {
            false
        } else {
            self.do_move(corner_index, orientation, index, color);
            true
        }
    }

    #[inline]
    pub fn do_move(&mut self,
                   corner_index: usize,
                   orientation: &Orientation,
                   index: usize,
                   color: Color) {
        self.occupied[color as usize].place_shape(&orientation.shape, index);
        self.auras[color as usize].place_shape(&orientation.shape, index);

        let corners = &mut self.corners[color as usize];

        corners.extend(orientation.corners().map(|c| Corner::new(c.coordinates + Point::from_index(corner_index), c.direction)));
        corners.swap_remove(corner_index);
    }

    #[inline]
    pub fn monomino_fits(&self, corner: usize, color: Color) -> bool {
        let corner = self.get_corner(corner, color);

        let orientation = &pieces::piece(0).orientation(0);
        let connect = orientation.get_corners(corner.direction.opposite())[0];
        let index = Board::get_index(corner, connect);
        !self.is_illegal(index, &orientation.shape, color)
    }

    #[inline]
    fn is_illegal(&self, index: usize, shape: &Shape, color: Color) -> bool {
        // Tests if piece intersects one of its own pieces or one of its
        // own pieces' aura.
        let auras = &self.auras[color as usize];
        let aura_intersect = auras.shape_intersects(shape, index);

        aura_intersect || self.intersects_others(index, shape, color)
    }

    #[inline]
    fn intersects_others(&self, index: usize, shape: &Shape, color: Color) -> bool {
        let intersects = |bitboard: &BitBoard| bitboard.shape_intersects(shape, index);
        
        let mut iter = self.occupied.iter();
        if iter.by_ref().take(color as usize).any(&intersects) {
            true
        } else if let Some(_) = iter.next() {
            iter.any(intersects)
        } else {
            false
        }
    }

    fn get_index(corner: Corner, connect: Point) -> usize {
        /*
        if corner.direction.up() {
            (corner.coordinates - connect.block_position()).to_index()
        } else {
            (corner.coordinates + connect).to_index()
        }
        */
        
        let connect = Corner::new(connect, corner.direction.opposite());

        (corner.coordinates + connect.block_position()).to_index()
    }

    pub fn random_corner<R: rand::Rng>(&self, color: Color, rng: &mut R) -> Option<usize> {
        if self.corners[color as usize].is_empty() {
            None
        } else {
            Some(rng.gen_range(0, self.corners[color as usize].len()))
        }
    }

    pub fn place_monomino(&mut self, corner_index: usize, color: Color) -> Turn {
        let corner = self.get_corner(corner_index, color);

        let orientation = pieces::piece(0).orientation(0);
        let connect = orientation.get_corners(corner.direction.opposite())[0];

        let index = Board::get_index(corner, connect);
        let orientation = pieces::piece(0).orientation(0);

        self.do_move(corner_index, orientation, index, color);
        Turn::new(0, 0, corner_index, connect, color)
    }

    pub fn get_corner(&self, index: usize, color: Color) -> Corner {
        self.corners[color as usize][index]
    }

    pub fn print(&self) {
        for y in 0..20 {
            for x in 0..20 {
                let index = y*20 + x;
                
                if self.occupied[0].index(index) {
                    print!("\x1B[34m*");
                } else if self.occupied[1].index(index) {
                    print!("\x1B[93m*");
                } else if self.occupied[2].index(index) {
                    print!("\x1B[92m*");
                } else if self.occupied[3].index(index) {
                    print!("\x1B[91m*");
                } else {
                    print!("\x1B[39m-");
                }
            }

            println!("\x1B[39m");
        }
    }

    pub fn print_auras(&self) {
        for y in 0..20 {
            for x in 0..20 {
                let index = y*20 + x;
                
                if self.auras[0].index(index) {
                    print!("\x1B[34m*");
                } else if self.auras[1].index(index) {
                    print!("\x1B[93m*");
                } else if self.auras[2].index(index) {
                    print!("\x1B[92m*");
                } else if self.auras[3].index(index) {
                    print!("\x1B[91m*");
                } else {
                    print!("\x1B[39m-");
                }
            }

            println!("\x1B[39m");
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        // Ew.
        Board {
            occupied: self.occupied.clone(),
            auras: self.auras.clone(),
            corners: [self.corners[0].clone(),
                      self.corners[1].clone(),
                      self.corners[2].clone(),
                      self.corners[3].clone()],
        }
    }
}
*/
