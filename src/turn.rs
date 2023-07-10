use crate::cube::Cube;

pub trait Turnable {
    fn r(&mut self);

    fn rprime(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    fn l(&mut self);

    fn lprime(&mut self) {
        self.l();
        self.l();
        self.l();
    }

    fn u(&mut self);

    fn uprime(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    fn f(&mut self);

    fn fprime(&mut self) {
        self.f();
        self.f();
        self.f();
    }
}

impl Turnable for Cube {
    fn r(&mut self) {
        let top = self.top();
        let front = self.front();
        let bottom = self.bottom();
        let back = self.back();

        self.right_mut().cycle_edges_cw();
        self.top_mut().copy_from_mask(
            &front,
            (0x1F << (5 * 2)) | (0x1f << (5 * 5) | (0x1f << (5 * 8))),
        );
        self.front_mut().copy_from_mask(
            &bottom,
            (0x1F << (5 * 2)) | (0x1f << (5 * 5) | (0x1f << (5 * 8))),
        );
        self.bottom_mut()
            .copy_from_positions(&back, &[(0, 8), (3, 5), (6, 2)]);
        self.back_mut()
            .copy_from_positions(&top, &[(2, 6), (5, 3), (8, 0)])
    }

    fn rprime(&mut self) {
        let top = self.top();
        let front = self.front();
        let bottom = self.bottom();
        let back = self.back();

        self.right_mut().cycle_edges_ccw();
        self.front_mut().copy_from_mask(
            &top,
            (0x1F << (5 * 2)) | (0x1f << (5 * 5) | (0x1f << (5 * 8))),
        );
        self.bottom_mut().copy_from_mask(
            &front,
            (0x1F << (5 * 2)) | (0x1f << (5 * 5) | (0x1f << (5 * 8))),
        );
        self.back_mut()
            .copy_from_positions(&bottom, &[(2, 6), (5, 3), (8, 0)]);
        self.top_mut()
            .copy_from_positions(&back, &[(0, 8), (3, 5), (6, 2)])
    }

    fn l(&mut self) {
        todo!()
    }

    fn u(&mut self) {
        let front = self.front();
        let right = self.right();
        let back = self.back();
        let left = self.left();

        self.top_mut().cycle_edges_cw();
        self.front_mut().copy_from_mask(
            &right,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.right_mut().copy_from_mask(
            &back,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.back_mut().copy_from_mask(
            &left,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.left_mut().copy_from_mask(
            &front,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
    }

    fn uprime(&mut self) {
        let front = self.front();
        let right = self.right();
        let back = self.back();
        let left = self.left();

        self.top_mut().cycle_edges_ccw();

        self.front_mut().copy_from_mask(
            &left,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.right_mut().copy_from_mask(
            &front,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.back_mut().copy_from_mask(
            &right,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
        self.left_mut().copy_from_mask(
            &back,
            (0x1f << (5 * 0)) | (0x1f << (5 * 1) | (0x1f << (5 * 2))),
        );
    }

    fn f(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_order() {
        let mut cube = Cube::default();
        cube.r();
        cube.r();
        cube.r();
        cube.r();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn r_prime_order() {
        let mut cube = Cube::default();
        cube.rprime();
        cube.rprime();
        cube.rprime();
        cube.rprime();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn u_order() {
        let mut cube = Cube::default();

        cube.u();
        cube.u();
        cube.u();
        cube.u();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn u_prime_order() {
        let mut cube = Cube::default();

        cube.uprime();
        cube.uprime();
        cube.uprime();
        cube.uprime();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn sexy_r() {
        let mut cube = Cube::default();

        for _ in 0..6 {
            cube.r();
            cube.u();
            cube.rprime();
            cube.uprime();
        }

        assert_eq!(cube, Cube::default());
    }
}
