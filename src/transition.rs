use std::fmt::{self, Display};
use std::ops::RangeInclusive;
use rand::{thread_rng, Rng};

use crate::utils::map_range_inclusive;

/// Container for action to perform given a binary symbol.
#[repr(transparent)]
#[derive(Debug)]
pub struct Transition {
    /// First action corresponds to the action executed when a 0 is read.
    /// Second action corresponds to the action executed when a 1 is read.
    actions: [Action; 2],
}

impl Transition {
    /// Creates a new transition with the given actions.
    #[inline]
    #[must_use]
    pub const fn new(action_on_0: Action, action_on_1: Action) -> Self {
        Self {
            actions: [action_on_0, action_on_1],
        }
    }

    /// Returns the action corresponding to the given symbol.
    #[inline]
    #[must_use]
    pub fn get_action_of(&self, symbol: u8) -> (u8, Direction, State) {
        self.actions[symbol as usize].unpack()
    }
}

impl Display for Transition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.actions[0], self.actions[1])
    }
}

/// Container for partially specified actions.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PartialTransition {
    actions: [Option<Action>; 2],
}

impl PartialTransition {
    /// Creates a new partial transition.
    #[inline]
    #[must_use]
    pub const fn new(action_on_0: Option<Action>, action_on_1: Option<Action>) -> Self {
        Self {
            actions: [action_on_0, action_on_1],
        }
    }

    /// Sets action to perform when the given symbol is read.
    #[inline]
    pub fn set_action_of(&mut self, symbol: u8, action: Option<Action>) {
        self.actions[symbol as usize] = action;
    }

    /// Returns action to perform when the given symbol is read.
    #[inline]
    #[must_use]
    pub fn get_action_of(&self, symbol: u8) -> Option<(u8, Direction, State)> {
        self.actions[symbol as usize].as_ref().map(|a| a.unpack())
    }
}

/// Encoded action to perform on a transition.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
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

    /// Unpacks the representation to return corresponding symbol to write, direction and state.
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
#[derive(Debug, PartialEq)]
pub enum Direction {
    /// Left direction
    Left,
    /// Right direction
    Right,
}

impl Direction {
    /// Returns a random direction.
    ///
    /// The distribution is uniform.
    #[inline]
    #[must_use]
    pub fn random() -> Self {
        let mut rng = thread_rng();

        Self::from(rng.gen_range(0..=1))
    }

    /// Returns the string representation of the direction.
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

/// State of Turing machine
///
/// Considering the complexity of the problem there is only 7 possible non-halting states.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    /// Halt state
    Halt,
    /// A state
    A,
    /// B state
    B,
    /// C state
    C,
    /// D state
    D,
    /// E state
    E,
    /// F state
    F,
    /// G state
    G,
}

impl State {
    /// Returns the starting state.
    /// As the order of the state does not matter we always start in state A by
    /// convention.
    #[inline]
    #[must_use]
    pub const fn start() -> Self {
        Self::A
    }

    /// Returns a random state in the given range.
    ///
    /// The distribution is uniform.
    #[inline]
    #[must_use]
    pub fn random(range: RangeInclusive<Self>) -> Self {
        let mut rng = thread_rng();

        Self::from(rng.gen_range(map_range_inclusive(range, |s| s as u8)))
    }

    /// Returns the string representation of the state.
    #[inline]
    fn to_str(&self) -> &str {
        match self {
            Self::Halt => "Halt",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
        }
    }
}

impl From<u8> for State {
    #[inline]
    #[must_use]
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
