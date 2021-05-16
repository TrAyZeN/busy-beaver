use rand::{thread_rng, Rng};
use std::fmt::{self, Display};
use std::ops::RangeInclusive;

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

impl From<PartialTransition> for Transition {
    #[inline]
    fn from(transition: PartialTransition) -> Self {
        Self::new(
            transition.actions[0].unwrap_or(Action::new(0, Direction::Right, State::Halt)),
            transition.actions[1].unwrap_or(Action::new(1, Direction::Right, State::Halt)),
        )
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
    pub fn get_action_of(self, symbol: u8) -> Option<(u8, Direction, State)> {
        self.actions[symbol as usize].map(Action::unpack)
    }

    /// Counts the number of specified actions.
    #[inline]
    #[must_use]
    pub fn count_specified_actions(self) -> usize {
        let mut c = 0;

        if self.actions[0].is_some() {
            c += 1;
        }

        if self.actions[1].is_some() {
            c += 1;
        }

        c
    }
}

impl Display for PartialTransition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(a) = self.actions[0] {
            write!(f, "{}", a)
        } else {
            write!(f, "---")
        }?;

        if let Some(a) = self.actions[1] {
            write!(f, " {}", a)
        } else {
            write!(f, " ---")
        }
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
    ///
    /// # Panics
    /// Panics in debug mode if `symbol >= 2`.
    #[inline]
    #[must_use]
    pub fn new(symbol: u8, direction: Direction, next_state: State) -> Self {
        debug_assert!(symbol < 2);

        Self {
            representation: ((next_state as u8) << 2) | ((direction as u8) << 1) | symbol,
        }
    }

    /// Returns the direction in which to move the head.
    #[inline]
    #[must_use]
    pub fn get_direction(self) -> Direction {
        Direction::from(self.representation >> 1 & 1)
    }

    /// Unpacks the representation to return corresponding symbol to write, direction and state.
    #[inline]
    #[must_use]
    fn unpack(self) -> (u8, Direction, State) {
        (
            self.representation & 1,
            self.get_direction(),
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
    const fn to_str(self) -> &'static str {
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

    /// Checks if the state is halting.
    #[inline(always)]
    #[must_use]
    pub fn is_halting(self) -> bool {
        self == Self::Halt
    }

    /// Returns the string representation of the state.
    #[inline]
    pub const fn to_str(self) -> &'static str {
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
        debug_assert!(state <= 7);

        match state {
            0 => Self::Halt,
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            _ => unreachable!(),
        }
    }
}
