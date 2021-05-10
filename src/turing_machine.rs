use crate::tape::Tape;
use crate::transition::{Action, Direction, PartialTransition, State, Transition};
use std::fmt::{self, Display};

/// A binary-alphabet Turing Machine with N non-halting states.
#[derive(Debug)]
pub struct TuringMachine<const N: usize> {
    transitions: [Transition; N],
}

impl<const N: usize> TuringMachine<N> {
    /// Creates a new binary-alphabet Turing Machine with the given transitions.
    ///
    /// # Panic
    /// Panics if `N < 2`.
    #[inline]
    #[must_use]
    pub fn new(transitions: [Transition; N]) -> Self {
        assert!(N >= 2);

        Self { transitions }
    }

    pub fn run(&self) {
        let mut current_state = State::start();
        let mut tape = Tape::new();

        while current_state != State::Halt {
            let (symbol, direction, new_state) =
                self.transitions[current_state as usize - 1].get_action_of(tape.read());

            tape.write(symbol);
            tape.move_head(direction);
            current_state = new_state;
        }
    }
}

impl<const N: usize> Display for TuringMachine<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut transitions = self.transitions.iter();
        write!(f, "{}", transitions.next().unwrap())?;

        for t in transitions {
            write!(f, " {}", t)?;
        }

        Ok(())
    }
}

/// A binary-alphabet partial Turing Machine with N non-halting states
#[derive(Debug)]
pub struct PartialTuringMachine<const N: usize> {
    transitions: [PartialTransition; N],
}

impl<const N: usize> PartialTuringMachine<N> {
    /// Creates a new partial Turing Machine
    ///
    /// # Panic
    /// Panics if `N < 2`.
    #[inline]
    #[must_use]
    pub fn new(transitions: [PartialTransition; N]) -> Self {
        assert!(N >= 2);

        Self { transitions }
    }

    #[inline]
    pub fn add_action(&mut self, state: State, symbol: u8, action: Action) {
        self.transitions[state as usize].set_action_of(symbol, Some(action));
    }

    pub fn run(&self) {
        let mut current_state = State::start();
        let mut tape = Tape::new();

        while current_state != State::Halt {
            let action = self.transitions[current_state as usize - 1].get_action_of(tape.read());
            if action.is_none() {
                break;
            }

            let (symbol, direction, new_state) = action.unwrap();
            tape.write(symbol);
            tape.move_head(direction);
            current_state = new_state;
        }
    }
}
