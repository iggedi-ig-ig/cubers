use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use cube::{Color, Cube};
use fxhash::{FxHashMap, FxHashSet};
use image::{ImageBuffer, Rgb};
use turn::Turnable;

mod cube;
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
    let mut cube = Cube::default();

    cube.b();

    save_cube(&cube);
}
