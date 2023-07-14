use crate::bidisearch::{Turn, Turnable};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Yellow,
    Green,
    Blue,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pyraminx {
    /// Front face is yellow, left face is blue, right face is red, bottom face is green.
    /// They are stored in that exact order.
    faces: [Face; 4],
}

impl Pyraminx {
    pub const FRONT: usize = 0;
    pub const LEFT: usize = 1;
    pub const RIGHT: usize = 2;
    pub const BOTTOM: usize = 3;

    fn r(&mut self) {
        let front = self.faces[Self::FRONT];
        let right = self.faces[Self::RIGHT];
        let bottom = self.faces[Self::BOTTOM];

        self.faces[Self::RIGHT].copy_from(&front, &[(1, 3), (2, 4), (3, 5)]);
        self.faces[Self::BOTTOM].copy_from(&right, &[(5, 2), (4, 1), (3, 0)]);
        self.faces[Self::FRONT].copy_from(&bottom, &[(0, 1), (1, 2), (2, 3)]);
    }

    fn r_prime(&mut self) {
        self.r();
        self.r();
    }

    fn l(&mut self) {
        let front = self.faces[Self::FRONT];
        let bottom = self.faces[Self::BOTTOM];
        let left = self.faces[Self::LEFT];

        self.faces[Self::LEFT].copy_from(&bottom, &[(0, 3), (4, 1), (5, 2)]);
        self.faces[Self::FRONT].copy_from(&left, &[(1, 3), (2, 4), (3, 5)]);
        self.faces[Self::BOTTOM].copy_from(&front, &[(5, 0), (4, 5), (3, 4)]);
    }

    fn l_prime(&mut self) {
        self.l();
        self.l();
    }

    fn b(&mut self) {
        let right = self.faces[Self::RIGHT];
        let left = self.faces[Self::LEFT];
        let bottom = self.faces[Self::BOTTOM];

        self.faces[Self::RIGHT].copy_from(&bottom, &[(2, 1), (3, 2), (4, 3)]);
        self.faces[Self::LEFT].copy_from(&right, &[(1, 3), (2, 4), (3, 5)]);
        self.faces[Self::BOTTOM].copy_from(&left, &[(5, 4), (4, 3), (3, 2)]);
    }

    fn b_prime(&mut self) {
        self.b();
        self.b();
    }

    fn u(&mut self) {
        let front = self.faces[Self::FRONT];
        let right = self.faces[Self::RIGHT];
        let left = self.faces[Self::LEFT];

        self.faces[Self::FRONT].copy_from(&right, &[(0, 0), (1, 1), (5, 5)]);
        self.faces[Self::LEFT].copy_from(&front, &[(0, 0), (1, 1), (5, 5)]);
        self.faces[Self::RIGHT].copy_from(&left, &[(0, 0), (1, 1), (5, 5)]);
    }

    fn u_prime(&mut self) {
        self.u();
        self.u();
    }
}

impl Default for Pyraminx {
    fn default() -> Self {
        Self {
            faces: [
                Face::fill(Color::Yellow),
                Face::fill(Color::Blue),
                Face::fill(Color::Red),
                Face::fill(Color::Green),
            ],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Face {
    /// Faces are stored like this:
    ///
    ///      0
    ///     1 2
    ///    3 4 5
    ///
    /// Note that the edge pieces are not stored, as they are trivial to solve.
    data: [Color; 6],
}

impl Face {
    pub fn fill(color: Color) -> Self {
        Self { data: [color; 6] }
    }

    /// Copies the given positions from another Face to this face.
    pub fn copy_from(&mut self, other: &Self, positions: &[(usize, usize)]) {
        for &(from, to) in positions {
            self.data[to] = other.data[from];
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PyraminxTurn {
    Left,
    LeftPrime,
    Right,
    RightPrime,
    Back,
    BackPrime,
    Up,
    UpPrime,
}

impl PyraminxTurn {
    const VALUES: &[Self] = &[
        Self::Left,
        Self::LeftPrime,
        Self::Right,
        Self::RightPrime,
        Self::Back,
        Self::BackPrime,
        Self::Up,
        Self::UpPrime,
    ];
}

impl Turnable for Pyraminx {
    type Turn = PyraminxTurn;

    fn possible_turns(&self) -> &[Self::Turn] {
        Self::Turn::VALUES
    }

    fn make_turn(&self, turn: Self::Turn) -> Self {
        let mut state = *self;
        match turn {
            PyraminxTurn::Left => state.l(),
            PyraminxTurn::LeftPrime => state.l_prime(),
            PyraminxTurn::Right => state.r(),
            PyraminxTurn::RightPrime => state.r_prime(),
            PyraminxTurn::Back => state.b(),
            PyraminxTurn::BackPrime => state.b_prime(),
            PyraminxTurn::Up => state.u(),
            PyraminxTurn::UpPrime => state.u_prime()
        }

        state
    }

    fn solved_state() -> Self {
        Self::default()
    }
}

impl Turn for PyraminxTurn {
    fn name(&self) -> &'static str {
        match self {
            PyraminxTurn::Left => "L",
            PyraminxTurn::LeftPrime => "L'",
            PyraminxTurn::Right => "R",
            PyraminxTurn::RightPrime => "R'",
            PyraminxTurn::Back => "B",
            PyraminxTurn::BackPrime => "B'",
            PyraminxTurn::Up => "U",
            PyraminxTurn::UpPrime => "U'",
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn r_order() {
        let mut pyraminx = Pyraminx::default();

        pyraminx.r();
        pyraminx.r();
        pyraminx.r();

        assert_eq!(pyraminx, Pyraminx::default());
    }

    #[test]
    fn l_order() {
        let mut pyraminx = Pyraminx::default();

        pyraminx.l();
        pyraminx.l();
        pyraminx.l();

        assert_eq!(pyraminx, Pyraminx::default());
    }

    #[test]
    fn u_order() {
        let mut pyraminx = Pyraminx::default();

        pyraminx.u();
        pyraminx.u();
        pyraminx.u();

        assert_eq!(pyraminx, Pyraminx::default());
    }

    #[test]
    fn b_order() {
        let mut pyraminx = Pyraminx::default();

        pyraminx.b();
        pyraminx.b();
        pyraminx.b();

        assert_eq!(pyraminx, Pyraminx::default());
    }

    #[test]
    fn sexy_rl() {
        let mut pyraminx = Pyraminx::default();

        for _ in 0..3 {
            pyraminx.r();
            pyraminx.l();
            pyraminx.r_prime();
            pyraminx.l_prime();
        }

        assert_eq!(pyraminx, Pyraminx::default());
    }

    #[test]
    fn orient_edges_order() {
        let mut pyraminx = Pyraminx::default();

        for _ in 0..3 {
            pyraminx.r();
            pyraminx.u();
            pyraminx.r_prime();
            pyraminx.u();
            pyraminx.r();
            pyraminx.u();
            pyraminx.r_prime();
        }

        assert_eq!(pyraminx, Pyraminx::default());
    }
}
