use itertools::iproduct;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Turn {
    axis: TurnAxis,
    direction: TurnDirection,
    layer: usize,
}

impl Turn {
    /// Returns an iterator over all possible turns for a cube with a given amount of layers.
    pub fn possible_turn_iter(&self, layers: usize) -> impl Iterator<Item = Turn> {
        iproduct!(0..layers, TurnAxis::VALUES, TurnDirection::VALUES).map(
            |(layer, &axis, &direction)| Turn {
                layer,
                axis,
                direction,
            },
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnAxis {
    X,
    Y,
    Z,
}

impl TurnAxis {
    pub const VALUES: &[Self] = &[Self::X, Self::Y, Self::Z];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnDirection {
    /// Clockwise rotation
    Cw,
    /// Counter clockwise rotation
    Ccw,
}

impl TurnDirection {
    pub const VALUES: &[Self] = &[Self::Cw, Self::Ccw];

    pub fn inverse(&self) -> Self {
        match *self {
            Self::Cw => Self::Ccw,
            Self::Ccw => Self::Cw,
        }
    }
}
