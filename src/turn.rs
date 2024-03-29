use crate::cube::Cube;

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    L, LPrime,
    R, RPrime,
    U, UPrime,
    D, DPrime,
    F, FPrime,
    B, BPrime
}
pub trait Turnable {
    fn perform_all(&mut self, turns: &[Move]) {
        for &turn in turns {
            self.perform(turn);
        }
    }

    fn perform(&mut self, r#move: Move) {
        match r#move {
            Move::L => self.l(),
            Move::LPrime => self.lprime(),
            Move::R => self.r(),
            Move::RPrime => self.rprime(),
            Move::U => self.u(),
            Move::UPrime => self.uprime(),
            Move::D => self.d(),
            Move::DPrime => self.dprime(),
            Move::F => self.f(),
            Move::FPrime => self.fprime(),
            Move::B => self.b(),
            Move::BPrime => self.bprime(),
        }
    }

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

    fn d(&mut self);

    fn dprime(&mut self) {
        self.d();
        self.d();
        self.d();
    }

    fn f(&mut self);

    fn fprime(&mut self) {
        self.f();
        self.f();
        self.f();
    }

    fn b(&mut self);

    fn bprime(&mut self) {
        self.b();
        self.b();
        self.b();
    }
}

impl Turnable for Cube {
    fn r(&mut self) {
        let top = self.top();
        let front = self.front();
        let bottom = self.bottom();
        let back = self.back();

        const MASK: u64 = (0x1F << (5 * 2)) | (0x1f << (5 * 5) | (0x1f << (5 * 8)));
        self.right_mut().cycle_edges_cw();
        self.top_mut().copy_from_mask(&front, MASK);
        self.front_mut().copy_from_mask(&bottom, MASK);
        self.bottom_mut()
            .copy_from_positions(&back, &[(0, 8), (3, 5), (6, 2)]);
        self.back_mut()
            .copy_from_positions(&top, &[(2, 6), (5, 3), (8, 0)])
    }

    fn l(&mut self) {
        let top = self.top();
        let front = self.front();
        let bottom = self.bottom();
        let back = self.back();

        const MASK: u64 = 0x1F | (0x1F << (3 * 5)) | (0x1F << (6 * 5));
        self.left_mut().cycle_edges_cw();
        self.front_mut().copy_from_mask(&top, MASK);
        self.bottom_mut().copy_from_mask(&front, MASK);
        self.back_mut()
            .copy_from_positions(&bottom, &[(0, 8), (3, 5), (6, 2)]);
        self.top_mut()
            .copy_from_positions(&back, &[(2, 6), (5, 3), (8, 0)]);
    }

    fn u(&mut self) {
        let front = self.front();
        let right = self.right();
        let back = self.back();
        let left = self.left();

        const MASK: u64 = 0x1f | (0x1f << 5 | (0x1f << (5 * 2)));
        self.top_mut().cycle_edges_cw();
        self.front_mut().copy_from_mask(&right, MASK);
        self.right_mut().copy_from_mask(&back, MASK);
        self.back_mut().copy_from_mask(&left, MASK);
        self.left_mut().copy_from_mask(&front, MASK);
    }

    fn d(&mut self) {
        let front = self.front();
        let right = self.right();
        let back = self.back();
        let left = self.left();

        const MASK: u64 = (0x1f << (5 * 6)) | (0x1f << (5 * 7) | (0x1f << (5 * 8)));
        self.bottom_mut().cycle_edges_cw();
        self.front_mut().copy_from_mask(&left, MASK);
        self.right_mut().copy_from_mask(&front, MASK);
        self.back_mut().copy_from_mask(&right, MASK);
        self.left_mut().copy_from_mask(&back, MASK);
    }

    fn f(&mut self) {
        let top = self.top();
        let right = self.right();
        let bottom = self.bottom();
        let left = self.left();

        self.front_mut().cycle_edges_cw();
        self.right_mut()
            .copy_from_positions(&top, &[(6, 0), (7, 3), (8, 6)]);
        self.bottom_mut()
            .copy_from_positions(&right, &[(0, 2), (3, 1), (6, 0)]);
        self.left_mut()
            .copy_from_positions(&bottom, &[(0, 2), (1, 5), (2, 8)]);
        self.top_mut()
            .copy_from_positions(&left, &[(2, 8), (5, 7), (8, 6)])
    }

    fn b(&mut self) {
        let top = self.top();
        let left = self.left();
        let bottom = self.bottom();
        let right = self.right();

        self.back_mut().cycle_edges_cw();
        self.left_mut()
            .copy_from_positions(&top, &[(0, 6), (1, 3), (2, 0)]);
        self.bottom_mut()
            .copy_from_positions(&left, &[(0, 6), (3, 7), (6, 8)]);
        self.right_mut()
            .copy_from_positions(&bottom, &[(6, 8), (7, 5), (8, 2)]);
        self.top_mut()
            .copy_from_positions(&right, &[(2, 0), (5, 1), (8, 2)]);
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
    fn l_order() {
        let mut cube = Cube::default();

        cube.l();
        cube.l();
        cube.l();
        cube.l();

        assert_eq!(cube, Cube::default())
    }

    #[test]
    fn l_prime_order() {
        let mut cube = Cube::default();

        cube.lprime();
        cube.lprime();
        cube.lprime();
        cube.lprime();

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
    fn d_order() {
        let mut cube = Cube::default();

        cube.d();
        cube.d();
        cube.d();
        cube.d();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn d_prime_order() {
        let mut cube = Cube::default();

        cube.dprime();
        cube.dprime();
        cube.dprime();
        cube.dprime();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn f_order() {
        let mut cube = Cube::default();

        cube.f();
        cube.f();
        cube.f();
        cube.f();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn f_prime_order() {
        let mut cube = Cube::default();

        cube.fprime();
        cube.fprime();
        cube.fprime();
        cube.fprime();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn b_order() {
        let mut cube = Cube::default();

        cube.b();
        cube.b();
        cube.b();
        cube.b();

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn b_prime_order() {
        let mut cube = Cube::default();

        cube.bprime();
        cube.bprime();
        cube.bprime();
        cube.bprime();

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

    #[test]
    fn sexy_l() {
        let mut cube = Cube::default();

        for _ in 0..6 {
            cube.l();
            cube.uprime();
            cube.lprime();
            cube.u();
        }

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn sledge_hammer_r() {
        let mut cube = Cube::default();

        for _ in 0..6 {
            cube.rprime();
            cube.f();
            cube.r();
            cube.fprime();
        }

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn sledge_hammer_l() {
        let mut cube = Cube::default();

        for _ in 0..6 {
            cube.l();
            cube.fprime();
            cube.lprime();
            cube.f();
        }

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn sune_order() {
        let mut cube = Cube::default();

        fn sune(cube: &mut Cube) {
            cube.r();
            cube.u();
            cube.rprime();
            cube.u();
            cube.r();
            cube.u();
            cube.u();
            cube.rprime();
        }

        for _ in 0..6 {
            sune(&mut cube);
        }

        assert_eq!(cube, Cube::default());
    }

    #[test]
    fn antisune_order() {
        let mut cube = Cube::default();

        fn antisune(cube: &mut Cube) {
            cube.lprime();
            cube.uprime();
            cube.l();
            cube.uprime();
            cube.lprime();
            cube.u();
            cube.u();
            cube.l();
        }

        for _ in 0..6 {
            antisune(&mut cube);
        }

        assert_eq!(cube, Cube::default());
    }

    #[test]
    // Note that the superflip uses all types of turns, so this is a pretty comprehensive test case
    fn superflip_order() {
        use Move::*;

        let mut cube = Cube::default();
        cube.perform_all(&[
            U, R, R, F, B, R, B, B, R, U, U, L, B, B, R, UPrime, DPrime, R, R, F, RPrime, L, B, B,
            U, U, F, F,
        ]);
        cube.perform_all(&[
            U, R, R, F, B, R, B, B, R, U, U, L, B, B, R, UPrime, DPrime, R, R, F, RPrime, L, B, B,
            U, U, F, F,
        ]);

        assert_eq!(cube, Cube::default());
    }
}
