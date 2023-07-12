use std::time::Instant;

use cube::{Color, Cube};
use image::{ImageBuffer, Rgb};
use turn::Turnable;

use crate::solver::Solver;

mod cube;
pub mod solver;
pub mod turn;

fn save_cube(cube: &Cube) {
    let mut buf = ImageBuffer::new(4 * 3, 3 * 3);

    let col_to_rgb = |col| match col {
        Color::White => Rgb([255u8, 255, 255]),
        Color::Yellow => Rgb([255, 255, 0]),
        Color::Blue => Rgb([0, 0, 255]),
        Color::Green => Rgb([0, 255, 0]),
        Color::Orange => Rgb([255, 125, 0]),
        Color::Red => Rgb([255, 0, 0]),
    };
    buf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        if y >= 3 && y < 6 {
            let face = [Cube::LEFT, Cube::FRONT, Cube::RIGHT, Cube::BACK][x as usize / 3];
            let index = (y - 3) * 3 + x - (x / 3) * 3;
            *pixel = col_to_rgb(cube.face(face).get(index as usize));
        } else if x >= 3 && x < 6 {
            let face = [Cube::TOP, Cube::FRONT, Cube::BOTTOM][y as usize / 3];
            let index = (y - (y / 3) * 3) * 3 + (x - 3);
            *pixel = col_to_rgb(cube.face(face).get(index as usize));
        }
    });

    buf.save("out.png").unwrap();
}

fn main() {
    use turn::Move::*;

    let mut cube = Cube::default();
    cube.perform_all(&[R, L, F, B, R, U, RPrime, UPrime, R, LPrime, U, U, D, D]);
    save_cube(&cube);

    let mut solver = Solver::from_state(cube);
    let start = Instant::now();
    println!("{:?}", solver.solve(12));
    println!("took {:?}", start.elapsed());
}
