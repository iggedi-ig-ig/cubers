use std::{
    collections::{hash_map::Entry, VecDeque},
    f32::consts::E,
};

use fxhash::FxHashMap;

use crate::{
    cube::Cube,
    turn::{Move, Turnable},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// This enum represents a distance to either the solved state, or the scramble of a rubiks cube.
enum DepthFromEnd {
    Solved(u8),
    Unsolved(u8),
}

impl DepthFromEnd {
    pub fn depth(&self) -> u8 {
        match *self {
            DepthFromEnd::Solved(i) => i,
            DepthFromEnd::Unsolved(i) => i,
        }
    }

    pub fn inc(self) -> Self {
        match self {
            DepthFromEnd::Solved(i) => Self::Solved(i + 1),
            DepthFromEnd::Unsolved(i) => Self::Unsolved(i + 1),
        }
    }
}

#[derive(Debug)]
pub struct Solver {
    queue: VecDeque<(Cube, DepthFromEnd)>,
    visited: FxHashMap<Cube, DepthFromEnd>,
}

impl Solver {
    pub fn from_state(cube: Cube) -> Self {
        let solved = Cube::default();
        Self {
            queue: VecDeque::from([
                (cube, DepthFromEnd::Unsolved(0)),
                (solved, DepthFromEnd::Solved(0)),
            ]),
            visited: FxHashMap::from_iter(
                [
                    (cube, DepthFromEnd::Unsolved(0)),
                    (solved, DepthFromEnd::Solved(0)),
                ]
                .into_iter(),
            ),
        }
    }

    pub fn solve(&mut self, max_depth: u8) -> Option<u8> {
        #[rustfmt::skip]
        const NEIGHBORS: &[Move] = &[
            Move::R, Move::RPrime,
            Move::L, Move::LPrime,
            Move::U, Move::UPrime,
            Move::D, Move::DPrime,
            Move::F, Move::FPrime,
            Move::B, Move::BPrime
        ];

        let mut last_depth = 0;
        while let Some((state, started_from)) = self.queue.pop_front() {
            if last_depth != started_from.depth() {
                last_depth = started_from.depth();
                println!("depth: {last_depth}, positions: {}", self.visited.len())
            }

            if started_from.depth() > max_depth {
                continue;
            }

            for &neighbor in NEIGHBORS {
                let mut state = state;
                state.perform(neighbor);

                match self.visited.entry(state) {
                    Entry::Occupied(e) => {
                        let other_from = e.get();

                        // if the two BFS fronts have met, we're done
                        if matches!(
                            (started_from, other_from),
                            (DepthFromEnd::Solved(_), &DepthFromEnd::Unsolved(_))
                                | (DepthFromEnd::Unsolved(_), &DepthFromEnd::Solved(_))
                        ) {
                            // Note that we add one here, because after making a move on state
                            // we're one layer deeper already
                            return Some(started_from.depth() + 1 + other_from.depth());
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(started_from.inc());
                        self.queue.push_back((state, started_from.inc()));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_turn() {
        let mut cube = Cube::default();
        cube.r();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(1), Some(1))
    }

    #[test]
    fn two_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(1), Some(2))
    }

    #[test]
    fn three_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(1), Some(3));
    }

    #[test]
    fn four_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(2), Some(4));
    }

    #[test]
    fn five_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();
        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(3), Some(5));
    }

    #[test]
    fn six_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();
        cube.u();
        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(3), Some(6));
    }

    #[test]
    fn seven_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();
        cube.u();
        cube.rprime();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(3), Some(7));
    }

    #[test]
    fn eight_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(3), Some(8));
    }

    #[test]
    fn nine_turns() {
        let mut cube = Cube::default();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();
        cube.u();
        cube.rprime();
        cube.uprime();
        cube.r();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(4), Some(9));
    }

    #[test]
    fn sune() {
        let mut cube = Cube::default();

        cube.r();
        cube.u();
        cube.rprime();
        cube.u();
        cube.r();
        cube.u();
        cube.u();
        cube.rprime();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(4), Some(8));
    }

    #[test]
    fn checkerboard() {
        let mut cube = Cube::default();

        cube.r();
        cube.r();

        cube.l();
        cube.l();

        cube.f();
        cube.f();

        cube.b();
        cube.b();

        cube.u();
        cube.u();

        cube.d();
        cube.d();

        let mut solver = Solver::from_state(cube);
        assert_eq!(solver.solve(6), Some(12));
    }

    #[test]
    fn superflip() {
        use crate::turn::Move::*;

        let mut cube = Cube::default();
        cube.perform_all(&[
            U, R, R, B, R, B, B, R, U, U, L, B, B, R, UPrime, DPrime, R, R, F, RPrime, L, B, B, U,
            U, F, F,
        ]);

        let mut solver = Solver::from_state(cube);

        assert_eq!(solver.solve(12), Some(24));
    }
}
