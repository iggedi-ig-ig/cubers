#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Center(Color),
    Edge([Color; 2]),
    Corner([Color; 3]),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Yellow,
    Green,
    Blue,
    Orange,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube3x3 {
    /// Stores the parts of a 3x3 cube when looking at it with green side towards you with white side up,
    /// starting at top left corner walking towards positive x
    data: [PieceType; 26],
}

impl Default for Cube3x3 {
    fn default() -> Self {
        use Color::*;
        Self {
            data: [
                PieceType::Corner([White, Green, Orange]),  // 0
                PieceType::Edge([White, Green]),            // 1
                PieceType::Corner([White, Red, Green]),     // 2
                PieceType::Edge([White, Orange]),           // 3
                PieceType::Center(White),                   // 4
                PieceType::Edge([White, Red]),              // 5
                PieceType::Corner([White, Orange, Blue]),   // 6
                PieceType::Edge([White, Blue]),             // 7
                PieceType::Corner([White, Blue, Red]),      // 8
                PieceType::Edge([Green, Orange]),           // 9
                PieceType::Center(Green),                   // 10
                PieceType::Edge([Green, Red]),              // 11
                PieceType::Center(Orange),                  // 12
                PieceType::Center(Orange),                  // 13
                PieceType::Edge([Blue, Orange]),            // 14
                PieceType::Center(Blue),                    // 15
                PieceType::Edge([Blue, Red]),               // 16
                PieceType::Corner([Yellow, Orange, Green]), // 17
                PieceType::Edge([Yellow, Green]),           // 18
                PieceType::Corner([Yellow, Green, Red]),    // 19
                PieceType::Edge([Yellow, Orange]),          // 20
                PieceType::Center(Yellow),                  // 21
                PieceType::Edge([Yellow, Red]),             // 22
                PieceType::Corner([Yellow, Blue, Orange]),  // 23
                PieceType::Edge([Yellow, Blue]),            // 24
                PieceType::Corner([Yellow, Red, Blue]),     // 25
            ],
        }
    }
}

impl Cube3x3 {
    fn set_data(&mut self, new_positions: &[(usize, usize)]) {
        let mut data = self.data;
        for &(from, to) in new_positions {
            data[to] = self.data[from];
        }
        self.data = data;
    }

    pub fn perform_move(&mut self, r#move: Move) {
        match r#move {
            Move::R => self.set_data(&[
                (2, 8),
                (5, 16),
                (8, 25),
                (16, 21),
                (25, 19),
                (22, 11),
                (19, 2),
                (11, 6),
            ]),
            Move::RPrime => {
                self.perform_move(Move::R);
                self.perform_move(Move::R);
                self.perform_move(Move::R)
            }
            Move::L => {
                self.set_data(&[
                    (0, 6),
                    (3, 14),
                    (6, 23),
                    (14, 19),
                    (23, 17),
                    (20, 9),
                    (17, 0),
                    (9, 4),
                ]);
            }
            Move::LPrime => {
                self.perform_move(Move::L);
                self.perform_move(Move::L);
                self.perform_move(Move::L)
            }
            Move::U => self.set_data(&[
                (0, 6),
                (3, 7),
                (6, 8),
                (7, 5),
                (8, 2),
                (5, 1),
                (2, 0),
                (1, 3),
            ]),
            Move::UPrime => {
                self.perform_move(Move::U);
                self.perform_move(Move::U);
                self.perform_move(Move::U)
            }
            Move::D => todo!(),
            Move::DPrime => todo!(),
            Move::M => todo!(),
            Move::MPrime => todo!(),
            Move::S => todo!(),
            Move::SPrime => todo!(),
            Move::F => todo!(),
            Move::FPrime => todo!(),
            Move::B => todo!(),
            Move::BPrime => todo!(),
            Move::E => todo!(),
            Move::EPrime => todo!(),
        }
    }
}

pub enum MoveAxis {
    X,
    Y,
    Z,
}

pub enum Move {
    R,
    RPrime,
    L,
    LPrime,
    U,
    UPrime,
    D,
    DPrime,
    M,
    MPrime,
    S,
    SPrime,
    F,
    FPrime,
    B,
    BPrime,
    E,
    EPrime,
}

impl Move {
    pub fn inverse(&self) -> Self {
        match self {
            Self::R => Self::RPrime,
            Self::RPrime => Self::R,
            Self::L => Self::LPrime,
            Self::LPrime => Self::L,
            Self::U => Self::UPrime,
            Self::UPrime => Self::U,
            Self::D => Self::DPrime,
            Self::DPrime => Self::D,
            Self::F => Self::FPrime,
            Self::FPrime => Self::F,
            Self::B => Self::BPrime,
            Self::BPrime => Self::B,
            Self::M => Self::MPrime,
            Self::MPrime => Self::M,
            Self::S => Self::SPrime,
            Self::SPrime => Self::S,
            Self::E => Self::EPrime,
            Self::EPrime => Self::E,
        }
    }

    pub fn axis(&self) -> MoveAxis {
        match self {
            Self::R | Self::RPrime => MoveAxis::X,
            Self::L | Self::LPrime => MoveAxis::X,
            Self::M | Self::MPrime => MoveAxis::X,
            Self::U | Self::UPrime => MoveAxis::Y,
            Self::D | Self::DPrime => MoveAxis::Y,
            Self::E | Self::EPrime => MoveAxis::Y,
            Self::F | Self::FPrime => MoveAxis::Z,
            Self::B | Self::BPrime => MoveAxis::Z,
            Self::S | Self::SPrime => MoveAxis::Z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_order() {
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

    #[test]
    fn l_order() {
        let mut cube = Cube3x3::default();

        cube.perform_move(Move::L);
        cube.perform_move(Move::L);
        cube.perform_move(Move::L);
        cube.perform_move(Move::L);

        assert_eq!(cube, Cube3x3::default());
    }

    #[test]
    fn u_order() {
        let mut cube = Cube3x3::default();
        cube.perform_move(Move::U);
        cube.perform_move(Move::U);
        cube.perform_move(Move::U);

        assert_eq!(cube, Cube3x3::default());
    }

    #[test]
    fn sexy_r() {
        let mut cube = Cube3x3::default();
        for _ in 0..6 {
            cube.perform_move(Move::R);
            cube.perform_move(Move::U);
            cube.perform_move(Move::RPrime);
            cube.perform_move(Move::UPrime);
        }

        assert_eq!(cube, Cube3x3::default());
    }

    #[test]
    fn sexy_l() {
        let mut cube = Cube3x3::default();
        for _ in 0..5 {
            cube.perform_move(Move::L);
            cube.perform_move(Move::UPrime);
            cube.perform_move(Move::LPrime);
            cube.perform_move(Move::U);
        }

        assert_eq!(cube, Cube3x3::default());
    }
}
