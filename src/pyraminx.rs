#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Yellow,
    Green,
    Blue,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pyraminx {
    /// Front face is yellow, left face is green, right face is blue, bottom face is red.
    /// They are stored in that exact order.
    faces: [Face; 4],
}

impl Pyraminx {
    pub fn perform_turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => self.l(),
            Turn::LeftPrime => self.l_prime(),
            Turn::Right => self.r(),
            Turn::RightPrime => self.r_prime(),
            Turn::Back => self.b(),
            Turn::BackPrime => self.b_prime(),
        }
    }

    fn r(&mut self) {}

    fn r_prime(&mut self) {}

    fn l(&mut self) {}

    fn l_prime(&mut self) {}

    fn b(&mut self) {}

    fn b_prime(&mut self) {}
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
    pub fn copy_from(&mut self, other: &Self, positions: &[(usize, usize)]) {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Turn {
    Left,
    LeftPrime,
    Right,
    RightPrime,
    Back,
    BackPrime,
}
