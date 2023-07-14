use pyraminx::Pyraminx;

use crate::{
    bidisearch::{Solver, Turnable},
    pyraminx::PyraminxTurn,
};

pub mod bidisearch;
pub mod cube;
pub mod pyraminx;

fn main() {
    use PyraminxTurn::*;
    let pyraminx = Pyraminx::default()
        .make_turn(Back)
        .make_turn(Right)
        .make_turn(Left)
        .make_turn(RightPrime)
        .make_turn(Left)
        .make_turn(Right)
        .make_turn(Up)
        .make_turn(Left)
        .make_turn(Right);

    let mut solver = Solver::from_state(pyraminx);
    dbg!(solver.try_solve(6));
}
