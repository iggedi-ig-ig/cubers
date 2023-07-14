//! This module defines a bidirectional search strategy to solve some kind of puzzles. This is only feasible for
//! puzzles with semi-small search spaces, like the 2x2, pyraminx, etc.
use std::{
    collections::{hash_map::Entry, VecDeque},
    fmt::Display,
    hash::Hash,
    time::{Duration, Instant},
};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum PathFrom {
    Solved(usize),
    Unsolved(usize),
}

impl PathFrom {
    fn increment(self) -> Self {
        match self {
            PathFrom::Solved(i) => Self::Solved(i + 1),
            PathFrom::Unsolved(i) => Self::Unsolved(i + 1),
        }
    }

    fn depth(&self) -> usize {
        match *self {
            PathFrom::Solved(i) => i,
            PathFrom::Unsolved(i) => i,
        }
    }

    fn is_solved(&self) -> bool {
        matches!(self, PathFrom::Solved(_))
    }
}

pub struct Solver<C: Turnable> {
    queue: VecDeque<C>,
    visited: FxHashMap<C, PathFrom>,
}

impl<C: Turnable> Solver<C> {
    pub fn from_state(state: C) -> Self {
        let solved = C::solved_state();

        Self {
            queue: VecDeque::from([state, solved]),
            visited: FxHashMap::from_iter(
                [
                    (state, PathFrom::Unsolved(0)),
                    (solved, PathFrom::Solved(0)),
                ]
                .into_iter(),
            ),
        }
    }

    /// TODO: return solution, not number of moves to solution
    pub fn try_solve(&mut self, max_depth: usize) -> Option<Solution<C::Turn>> {
        let start = Instant::now();
        let mut last_depth = 0;
        while let Some(state) = self.queue.pop_front() {
            let depth = self.visited[&state];
            if depth.depth() >= max_depth {
                continue;
            } else if last_depth != depth.depth() {
                last_depth = depth.depth();
                println!("depth {}: {} positions", last_depth, self.visited.len());
            }
            for &neighbor in state.possible_turns() {
                let new_state = state.make_turn(neighbor);

                match self.visited.entry(new_state) {
                    Entry::Occupied(e) => {
                        let other = e.get();
                        if other.is_solved() ^ depth.is_solved() {
                            return Some(Solution {
                                n_turns: other.depth() + depth.depth() + 1,
                                duration: start.elapsed(),
                                turns: Vec::default(),
                            });
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(depth.increment());
                        self.queue.push_back(new_state);
                    }
                }
            }
        }

        None
    }
}

pub trait Turnable: Copy + PartialEq + Eq + Hash {
    type Turn: Turn;

    fn possible_turns(&self) -> &[Self::Turn];

    fn make_turn(&self, turn: Self::Turn) -> Self;

    fn solved_state() -> Self;
}
pub trait Turn: Copy + PartialEq + Eq + Hash {
    fn name(&self) -> &'static str;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Solution<T: Turn> {
    n_turns: usize,
    duration: Duration,
    turns: Vec<T>,
}

impl<T: Turn> Display for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.turns.iter().map(|t| t.name()).join(" "))
    }
}
