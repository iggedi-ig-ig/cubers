use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::IndexMut,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Color {
    White  = 0b00000,
    Yellow = 0b00001,
    Green  = 0b00010,
    Blue   = 0b00100,
    Orange = 0b01000,
    Red    = 0b10000
}

impl Color {
    pub const BITS: usize = 5;

    /// # Panic
    /// This method panics if the u8 given as an argument does not represent
    /// any valid color.
    pub fn from_u8(n: u8) -> Self {
        match n {
            0b00000 => Self::White,
            0b00001 => Self::Yellow,
            0b00010 => Self::Green,
            0b00100 => Self::Blue,
            0b01000 => Self::Orange,
            0b10000 => Self::Red,
            n => panic!("unknown color: {n:0>8b}"),
        }
    }
}

/// This struct represents the colors one one face of a 3x3 rubiks cube.
/// Each color is represented by a 5 bit number in this order:
///     + --- +
///     | 012 |
///     | 345 |
///     | 678 |
///     + --- |

/// in this format:
///     0bxxxxxxxxxxxxxxxxxxx888887777766666555554444433333222221111100000
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Face(u64);

impl Face {
    pub const WHITE: Self = Face(0x0);
    pub const YELLOW: Self = Face(0x10842108421);
    pub const GREEN: Self = Face(0x21084210842);
    pub const BLUE: Self = Face(0x42108421084);
    pub const RED: Self = Face(0x108421084210);
    pub const ORANGE: Self = Face(0x84210842108);

    pub fn get(&self, index: usize) -> Color {
        let mask = 0x1F << (5 * index);
        let masked = (self.0 & mask) >> (5 * index);
        Color::from_u8(masked as u8)
    }

    pub fn set(&mut self, index: usize, color: Color) {
        let mask = 0x1F << (5 * index);
        self.0 &= !mask;
        self.0 |= (color as u64) << (5 * index);
    }

    pub fn copy_from_mask(&mut self, from: &Self, mask: u64) {
        let masked = from.0 & mask;
        self.0 &= !mask;
        self.0 |= masked;
    }

    pub fn copy_from_positions(&mut self, from_face: &Self, positions: &[(usize, usize)]) {
        positions.into_iter().for_each(|&(from, to)| {
            let from_mask = 0x1f << (5 * from);
            let to_mask = 0x1f << (5 * to);

            let from_bits = (from_face.0 & from_mask) >> (5 * from);
            let to_bits = from_bits << (5 * to);

            self.0 &= !to_mask;
            self.0 |= to_bits;
        });
    }

    pub fn cycle_edges_cw(&mut self) {
        let zero = self.get(0);
        let three = self.get(3);
        self.set(0, self.get(6));
        self.set(3, self.get(7));
        self.set(6, self.get(8));
        self.set(7, self.get(5));
        self.set(8, self.get(2));
        self.set(5, self.get(1));
        self.set(2, zero);
        self.set(1, three);
    }

    pub fn cycle_edges_ccw(&mut self) {
        let zero = self.get(0);
        let one = self.get(1);
        self.set(0, self.get(2));
        self.set(1, self.get(5));
        self.set(2, self.get(8));
        self.set(5, self.get(7));
        self.set(8, self.get(6));
        self.set(7, self.get(3));
        self.set(6, zero);
        self.set(3, one);
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries((0..9).map(|i| self.get(i))).finish()
    }
}

impl Hash for Face {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0)
    }
}

/// This struct represents a 3x3 rubiks cube. It does so by storing the colors of all six faces,
/// where the middle (front) face is green and the top is white (this the default orientation when scrambling a cube).
///
/// The indices of the faces are oriented like this:
///
///             + --- +
///             | 012 |
///             | 345 |
///             | 678 |
///       + --- + --- + --- + --- +
///       | 012 | 012 | 012 | 012 |
///       | 345 | 345 | 345 | 345 |
///       | 678 | 678 | 678 | 678 |
///       + --- + --- + --- + --- +
///             | 012 |
///             | 345 |
///             | 678 |
///             + --- +
///
/// Faces are stored in TOP BOTTOM LEFT RIGHT FRONT BACK order (white yellow orange red green blue)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cube {
    data: [Face; 6],
}

impl Cube {
    pub const TOP: usize = 0;
    pub const BOTTOM: usize = 1;
    pub const LEFT: usize = 2;
    pub const RIGHT: usize = 3;
    pub const FRONT: usize = 4;
    pub const BACK: usize = 5;

    pub fn face(&self, index: usize) -> Face {
        self.data[index]
    }

    pub fn top(&self) -> Face {
        self.data[Self::TOP]
    }
    pub fn top_mut(&mut self) -> &mut Face {
        &mut self.data[Self::TOP]
    }

    pub fn bottom(&self) -> Face {
        self.data[Self::BOTTOM]
    }
    pub fn bottom_mut(&mut self) -> &mut Face {
        &mut self.data[Self::BOTTOM]
    }

    pub fn left(&self) -> Face {
        self.data[Self::LEFT]
    }
    pub fn left_mut(&mut self) -> &mut Face {
        &mut self.data[Self::LEFT]
    }

    pub fn right(&self) -> Face {
        self.data[Self::RIGHT]
    }
    pub fn right_mut(&mut self) -> &mut Face {
        &mut self.data[Self::RIGHT]
    }

    pub fn front(&self) -> Face {
        self.data[Self::FRONT]
    }
    pub fn front_mut(&mut self) -> &mut Face {
        &mut self.data[Self::FRONT]
    }

    pub fn back(&self) -> Face {
        self.data[Self::BACK]
    }
    pub fn back_mut(&mut self) -> &mut Face {
        &mut self.data[Self::BACK]
    }

    pub fn face_mut(&mut self, index: usize) -> &mut Face {
        &mut self.data[index]
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            data: [
                Face::WHITE,
                Face::YELLOW,
                Face::ORANGE,
                Face::RED,
                Face::GREEN,
                Face::BLUE,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_cw_once() {
        let mut face = Face(Color::Yellow as u64);
        face.cycle_edges_cw();

        assert_eq!(face, Face((Color::Yellow as u64) << (5 * 2)))
    }

    #[test]
    fn cycle_ccw_once() {
        let mut face = Face(Color::Yellow as u64);
        face.cycle_edges_ccw();

        assert_eq!(face, Face((Color::Yellow as u64) << (5 * 6)))
    }

    #[test]
    fn cycle_order_cw() {
        let mut face = Face(Color::Yellow as u64);
        for _ in 0..8 {
            face.cycle_edges_cw();
        }

        assert_eq!(face, Face(Color::Yellow as u64));
    }

    #[test]
    fn cycle_order_ccw() {
        let mut face = Face(Color::Yellow as u64);
        for _ in 0..8 {
            face.cycle_edges_ccw();
        }

        assert_eq!(face, Face(Color::Yellow as u64));
    }
}
