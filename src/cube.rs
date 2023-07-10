use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cubie {
    Center(Color),
    Edge([Color; 2]),
    Corner([Color; 3]),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0b000001,
    Yellow = 0b000010,
    Green = 0b000100,
    Blue = 0b001000,
    Orange = 0b010000,
    Red = 0b100000,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CubeFace {
    data: u64,
}

impl Display for CubeFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = (0..8).map(|i| self.get_at(i)).collect_vec();
        f.debug_struct("CubeFace").field("data", &vec).finish()
    }
}

impl CubeFace {
    pub fn new(colors: &[Color]) -> Self {
        let mut data = colors
            .into_iter()
            .enumerate()
            .fold(0, |curr, (i, &acc)| curr | ((acc as u64) << (6 * i)));

        Self { data }
    }

    pub fn get_at(&self, index: usize) -> Color {
        let mask = 0x3F << (6 * index);

        match mask >> (6 * index) {
            0b000001 => Color::White,
            0b000010 => Color::Yellow,
            0b000100 => Color::Green,
            0b001000 => Color::Blue,
            0b010000 => Color::Orange,
            0b100000 => Color::Red,
            flag => panic!("invalid color flag: {flag:b}"),
        }
    }

    pub fn set_at(&mut self, index: usize, color: Color) {
        let mask = 0x3F << (6 * index);
        self.data &= !mask;
        self.data |= (color as u64) << (6 * index);
    }

    pub fn copy_from_mask(&mut self, from: &Self, mask: u64) {
        let masked = from.data & mask;
        self.data &= !mask;
        self.data |= masked;
    }

    pub fn rotate_cw(&mut self) {
        let zero = self.get_at(0);
        let one = self.get_at(1);
        let two = self.get_at(2);
        let three = self.get_at(3);
        let five = self.get_at(5);
        let six = self.get_at(6);
        let seven = self.get_at(7);
        let eight = self.get_at(8);
        self.set_at(0, six);
        self.set_at(1, three);
        self.set_at(2, zero);
        self.set_at(3, seven);
        self.set_at(5, one);
        self.set_at(6, eight);
        self.set_at(7, five);
        self.set_at(8, two);
    }

    pub fn rotate_ccw(&mut self) {
        // TODO: this is slower
        self.rotate_cw();
        self.rotate_cw();
        self.rotate_cw();
    }

    pub fn red() -> Self {
        Self::new(&[Color::Red; 8])
    }

    pub fn orange() -> Self {
        Self::new(&[Color::Orange; 8])
    }

    pub fn green() -> Self {
        Self::new(&[Color::Green; 8])
    }

    pub fn blue() -> Self {
        Self::new(&[Color::Blue; 8])
    }

    pub fn white() -> Self {
        Self::new(&[Color::White; 8])
    }

    pub fn yellow() -> Self {
        Self::new(&[Color::Yellow; 8])
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
enum FaceIndex {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube3x3 {
    /// Stores the 6 faces of the cube. When looking at it from the green site with white site on top,
    /// stored in LEFT RIGHT TOP BOTTOM FRONT BACK order
    /// (so orange, red, white, yellow, green, blue)
    ///
    /// Index order is like this
    ///         + --- +
    ///         | 012 |
    ///         | 345 |
    ///         | 678 |
    ///   + --- + --- + --- + --- +
    ///   | 012 | 012 | 012 | 012 |
    ///   | 345 | 345 | 345 | 345 |
    ///   | 678 | 678 | 678 | 678 |
    ///   + --- + --- + --- + --- +
    ///         | 012 |
    ///         | 345 |
    ///         | 678 |
    ///         + --- +     
    data: [CubeFace; 6],
}

impl Default for Cube3x3 {
    fn default() -> Self {
        Self {
            data: [
                CubeFace::orange(),
                CubeFace::red(),
                CubeFace::white(),
                CubeFace::yellow(),
                CubeFace::green(),
                CubeFace::blue(),
            ],
        }
    }
}

impl Index<FaceIndex> for Cube3x3 {
    type Output = CubeFace;

    fn index(&self, index: FaceIndex) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<FaceIndex> for Cube3x3 {
    fn index_mut(&mut self, index: FaceIndex) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl Cube3x3 {
    pub fn perform_move(&mut self, r#move: Move) {
        let (axis, dir) = r#move.axis();
        let layer_mask = r#move.layer_mask();
        match axis {
            x @ (MoveAxis::XPositive | MoveAxis::XNegative) => {
                let pos = matches!(x, MoveAxis::XPositive);

                let top = self[FaceIndex::Top];
                let bottom = self[FaceIndex::Bottom];
                let front = self[FaceIndex::Front];
                let back = self[FaceIndex::Back];

                self[FaceIndex::Top].copy_from_mask(&front, layer_mask);
                self[FaceIndex::Front].copy_from_mask(&bottom, layer_mask);
                self[FaceIndex::Bottom].copy_from_mask(&back, layer_mask);
                self[FaceIndex::Back].copy_from_mask(&top, layer_mask);

                match (dir, pos) {
                    (MoveAxisDir::ClockWise, true) => self[FaceIndex::Right].rotate_cw(),
                    (MoveAxisDir::ClockWise, false) => self[FaceIndex::Left].rotate_ccw(),
                    (MoveAxisDir::CounterClockWise, true) => self[FaceIndex::Right].rotate_ccw(),
                    (MoveAxisDir::CounterClockWise, false) => self[FaceIndex::Left].rotate_cw(),
                }
            }
            y @ (MoveAxis::YPositive | MoveAxis::YNegative) => {
                let pos = matches!(y, MoveAxis::YPositive);

                let front = self[FaceIndex::Front];
                let left = self[FaceIndex::Left];
                let back = self[FaceIndex::Back];
                let right = self[FaceIndex::Right];

                self[FaceIndex::Front].copy_from_mask(&right, layer_mask);
                self[FaceIndex::Left].copy_from_mask(&front, layer_mask);
                self[FaceIndex::Back].copy_from_mask(&left, layer_mask);
                self[FaceIndex::Right].copy_from_mask(&back, layer_mask);

                //match (dir, pos) {
                //    (MoveAxisDir::ClockWise, true) => self[FaceIndex::Top].rotate_cw(),
                //    (MoveAxisDir::ClockWise, false) => self[FaceIndex::Bottom].rotate_ccw(),
                //    (MoveAxisDir::CounterClockWise, true) => self[FaceIndex::Top].rotate_ccw(),
                //    (MoveAxisDir::CounterClockWise, false) => self[FaceIndex::Bottom].rotate_cw(),
                //}
            }
            z @ (MoveAxis::ZPositive | MoveAxis::ZNegative) => {
                let pos = matches!(z, MoveAxis::ZPositive);
                todo!()
            }
        }
    }
}

pub enum MoveAxis {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
    ZPositive,
    ZNegative,
}

pub enum MoveAxisDir {
    ClockWise,
    CounterClockWise,
}

#[rustfmt::skip]
pub enum Move {
    R, RPrime,
    L, LPrime,
    U, UPrime,
    D, DPrime,
    F, FPrime,
    B, BPrime,
    M, MPrime,
    S, SPrime,
    E, EPrime,
}

impl Move {
    pub fn layer_mask(&self) -> u64 {
        match self {
            Move::R | Move::RPrime => (0x3F << 2) | (0x3F << 5) | (0x3F << 8),
            Move::L | Move::LPrime => (0x3F << 0) | (0x3F << 3) | (0x3F << 6),
            Move::U | Move::UPrime => (0x3F << 0) | (0x3F << 1) | (0x3F << 2),
            Move::D | Move::DPrime => (0x3F << 6) | (0x3F << 7) | (0x3F << 8),
            Move::F | Move::FPrime => todo!("edge case"),
            Move::B | Move::BPrime => todo!("edge case"),
            Move::M | Move::MPrime => (0x3F << 1) | (0x3F << 4) | (0x3F | 7),
            Move::S | Move::SPrime => todo!("edge case"),
            Move::E | Move::EPrime => (0x3F << 3) | (0x3F << 4) | (0x3F << 5),
        }
    }

    pub fn axis(&self) -> (MoveAxis, MoveAxisDir) {
        use MoveAxis::*;
        use MoveAxisDir::*;

        match self {
            Self::R => (XPositive, ClockWise),
            Self::RPrime => (XPositive, CounterClockWise),
            Self::L => (XNegative, ClockWise),
            Self::LPrime => (XNegative, CounterClockWise),
            Self::M => (XPositive, ClockWise),
            Self::MPrime => (XNegative, CounterClockWise),
            Self::U => (YPositive, ClockWise),
            Self::UPrime => (YPositive, CounterClockWise),
            Self::D => (YNegative, ClockWise),
            Self::DPrime => (YNegative, CounterClockWise),
            Self::E => (YPositive, ClockWise),
            Self::EPrime => (YPositive, CounterClockWise),
            Self::F => (ZPositive, ClockWise),
            Self::FPrime => (ZPositive, CounterClockWise),
            Self::B => (ZNegative, ClockWise),
            Self::BPrime => (ZNegative, CounterClockWise),
            Self::S => (ZPositive, ClockWise),
            Self::SPrime => (ZPositive, CounterClockWise),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_order() {
        let mut cube = Cube3x3::default();
        let default_cube = Cube3x3::default();

        cube.perform_move(Move::R);
        cube.perform_move(Move::R);
        cube.perform_move(Move::R);
        cube.perform_move(Move::R);

        eprintln!(
            "{} {} {} {} {} {}",
            cube.data[0], cube.data[1], cube.data[2], cube.data[3], cube.data[4], cube.data[5]
        );
        assert_eq!(cube, default_cube)
    }

    #[test]
    fn l_order() {
        let mut cube = Cube3x3::default();

        cube.perform_move(Move::L);
        cube.perform_move(Move::L);
        cube.perform_move(Move::L);
        cube.perform_move(Move::L);

        eprintln!(
            "{} {} {} {} {} {}",
            cube.data[0], cube.data[1], cube.data[2], cube.data[3], cube.data[4], cube.data[5]
        );
        assert_eq!(cube, Cube3x3::default());
    }

    #[test]
    fn u_order() {
        let mut cube = Cube3x3::default();
        cube.perform_move(Move::U);
        cube.perform_move(Move::U);
        cube.perform_move(Move::U);

        eprintln!(
            "{} {} {} {} {} {}",
            cube.data[0], cube.data[1], cube.data[2], cube.data[3], cube.data[4], cube.data[5]
        );
        assert_eq!(cube, Cube3x3::default());
    }

    #[test]
    fn sexy_r() {
        let mut cube = Cube3x3::default();
        for _ in 0..6 {
            cube.perform_move(Move::R);
            cube.perform_move(Move::U);
            cube.perform_move(Move::RPrime);
            cube.perform_move(Move::UPrime);
        }

        assert_eq!(cube.data, Cube3x3::default().data);
    }

    #[test]
    fn sexy_l() {
        let mut cube = Cube3x3::default();
        for _ in 0..5 {
            cube.perform_move(Move::L);
            cube.perform_move(Move::UPrime);
            cube.perform_move(Move::LPrime);
            cube.perform_move(Move::U);
        }

        assert_eq!(cube.data, Cube3x3::default().data);
    }
}
