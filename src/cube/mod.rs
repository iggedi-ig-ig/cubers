//! This @module contains all rubiks-cube-esqe puzzles that are vanilla cube shaped,
//! like the 2x2, 3x3, 4x4 and any NxN cube in general.

use std::ops::{Index, IndexMut};

use itertools::Itertools;

use self::turn::{TurnAxis, TurnDirection};

pub mod turn;

pub const FRONT: usize = 1;
pub const BACK: usize = 3;
pub const LEFT: usize = 0;
pub const RIGHT: usize = 2;
pub const TOP: usize = 4;
pub const BOTTOM: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Yellow,
    Green,
    Blue,
    Orange,
    Red,
}

/// This struct represents an NxNxN rub    pub const BACK: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct Cube<const N: usize> {
    ///   4
    /// 0 1 2 3
    ///   5
    faces: [CubeFace<N>; 6],
}

impl<const N: usize> Cube<N> {
    pub fn perform_turn(
        &mut self,
        _other: &Self,
        layer: usize,
        axis: TurnAxis,
        dir: TurnDirection,
    ) {
        // the layers that take part in this turn
        let involved_faces = match (axis, dir) {
            (TurnAxis::X, TurnDirection::Cw) => [FRONT, TOP, BACK, BOTTOM],
            (TurnAxis::X, TurnDirection::Ccw) => [FRONT, BOTTOM, BACK, TOP],
            (TurnAxis::Y, TurnDirection::Cw) => [FRONT, TOP, BACK, BOTTOM],
            (TurnAxis::Y, TurnDirection::Ccw) => [FRONT, BOTTOM, BACK, TOP],
            (TurnAxis::Z, TurnDirection::Cw) => [TOP, RIGHT, BOTTOM, LEFT],
            (TurnAxis::Z, TurnDirection::Ccw) => [TOP, LEFT, BOTTOM, RIGHT],
        }
        .map(|idx| self.faces[idx]);

        match layer {
            0 => {
                let cycle_layer = match axis {
                    TurnAxis::X => FRONT,
                    TurnAxis::Y => TOP,
                    TurnAxis::Z => LEFT,
                };
                self.faces[cycle_layer].rotate_layer(dir);
            }
            i if i == N - 1 => {
                let cycle_layer = match axis {
                    TurnAxis::X => BACK,
                    TurnAxis::Y => BOTTOM,
                    TurnAxis::Z => RIGHT,
                };
                self.faces[cycle_layer].rotate_layer(dir.inverse());
            }
            i if (0..N).contains(&i) => {
                for (i, ele) in involved_faces.into_iter().enumerate() {
                    let next = (i + 1) % involved_faces.len();
                    let (from, to) = (ele, involved_faces[next]);
                }
            }
            i => panic!("tried to turn layer {i} of cube with {N} layers."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CubeFace<const N: usize> {
    /// representative representation of 3x3
    /// 0 1 2   0 0 0       0 1 2
    /// 0 1 2   1 1 1  -->  3 4 5
    /// 0 1 2   2 2 2       6 7 8
    colors: [[Color; N]; N],
}

impl<const N: usize> CubeFace<N> {
    pub fn rotate_layer(&mut self, dir: TurnDirection) {
        let colors = self.colors;
        // TODO: this currently only rotates the very outer edges. This works only for N < 4
        match dir {
            TurnDirection::Cw => {
                for i in 0..N {
                    self.colors[0][i] = colors[N - i - 1][0];
                    self.colors[i][N - 1] = colors[0][N - i - 1];
                    self.colors[N - 1][i] = colors[N - i - 1][N - 1];
                    self.colors[i][0] = colors[N - 1][N - i - 1];
                }
            }
            TurnDirection::Ccw => {
                for i in 0..N {
                    self.colors[0][i] = colors[N - i - 1][N - 1];
                    self.colors[i][0] = colors[0][N - i - 1];
                    self.colors[N - 1][i] = colors[N - i - 1][0];
                    self.colors[i][N - 1] = colors[N - 1][N - i - 1];
                }
            }
        }
    }

    pub fn copy_from(&mut self, from: &Self, positions: &[(usize, usize)]) {
        for &(from_idx, to_idx) in positions {
            self[to_idx] = from[from_idx];
        }
    }
}

impl<const N: usize> Index<usize> for CubeFace<N> {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        let x = index % N;
        let y = index / N;
        &self.colors[y][x]
    }
}

impl<const N: usize> IndexMut<usize> for CubeFace<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let x = index % N;
        let y = index / N;
        &mut self.colors[y][x]
    }
}
