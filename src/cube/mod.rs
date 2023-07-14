//! This module contains all rubiks-cube-esqe puzzles that are vanilla cube shaped,
//! like the 2x2, 3x3, 4x4 and any NxN cube in general.

pub mod turn;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Yellow,
    Green,
    Blue,
    Orange,
    Red,
}

/// This struct represents an NxNxN rubiks cube.
#[derive(Copy, Clone, Debug)]
pub struct Cube<const N: usize> {
    faces: [CubeFace<N>; 6],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CubeFace<const N: usize> {
    colors: [[Color; N]; N],
}
