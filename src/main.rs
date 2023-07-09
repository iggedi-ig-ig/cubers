use crate::cube::{Cube3x3, Move};

mod cube;

fn main() {
    let mut cube = Cube3x3::default();
    let default_cube = Cube3x3::default();

    let mut i = 0;
    let num = loop {
        cube.perform_move(Move::R);
        i += 1;

        if cube == default_cube {
            break i;
        }

        if i % 100 == 0 {
            eprintln!("{i}");
        }
    };

    assert_eq!(num, 4)
}
