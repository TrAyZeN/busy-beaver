use std::fmt::{self, Display};

/// Container for action to perform given a binary symbol
#[repr(transparent)]
#[derive(Debug)]
pub struct Transition {
    /// First action corresponds to the action executed when a 0 is read.
    /// Second action corresponds to the action exectued when a 1 is read.
    actions: [Action; 2],
}

impl Transition {
    /// Creates a new transition with the given actions
    #[inline]
    #[must_use]
    pub const fn new(action_on_0: Action, action_on_1: Action) -> Self {
        Self {
            actions: [action_on_0, action_on_1],
        }
    }

    /// Returns the action corresponding to the given symbol
    #[inline]
    #[must_use]
    pub fn action_of(&self, symbol: u8) -> (u8, Direction, State) {
        self.actions[symbol as usize].unpack()
    }
}

impl Display for Transition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.actions[0], self.actions[1])
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Action {
    /// Packed representation of the action:
    /// Bit 0: Symbol to write on the tape
    /// Bit 1: Direction in which to move the head (0 = Left, 1 = Right)
    /// Bit 2-7: Next state
    representation: u8,
}

impl Action {
    /// Creates a new action from the given direction and next state.
    #[inline]
    #[must_use]
    pub fn new(symbol: u8, direction: Direction, next_state: State) -> Self {
        debug_assert!(symbol < 2);

        Self {
            representation: ((next_state as u8) << 2) | ((direction as u8) << 1) | symbol,
        }
    }

    /// Unpacks the representation to return corresponding symbol to write, direction and state
    #[inline]
    #[must_use]
    fn unpack(&self) -> (u8, Direction, State) {
        (
            self.representation & 1,
            Direction::from(self.representation >> 1 & 1),
            State::from(self.representation >> 2),
        )
    }
}

impl Display for Action {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (symbol, direction, state) = self.unpack();
        write!(
            f,
            "{}{}{}",
            symbol,
            direction.to_str().chars().next().unwrap(),
            state.to_str().chars().next().unwrap()
        )
    }
}

/// Direction in which to move the head
#[repr(u8)]
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Converts the given variant to a `&str`
    #[inline]
    fn to_str(&self) -> &str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }
}

impl From<u8> for Direction {
    #[inline]
    #[must_use]
    fn from(direction: u8) -> Self {
        debug_assert!(direction < 2);

        match direction {
            0 => Self::Left,
            1 => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Halt,
    A,
    B,
    C,
}

impl State {
    /// Always starts in A state
    #[inline]
    #[must_use]
    pub const fn start() -> Self {
        Self::A
    }

    /// Converts the given variant to a `&str`
    #[inline]
    fn to_str(&self) -> &str {
        match self {
            Self::Halt => "Halt",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

impl From<u8> for State {
    #[must_use]
    #[inline]
    fn from(state: u8) -> Self {
        debug_assert!(state < 4);

        match state {
            0 => Self::Halt,
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            _ => unreachable!(),
        }
    }
}

