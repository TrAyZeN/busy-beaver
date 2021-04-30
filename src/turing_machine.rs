use std::fmt::{self, Display};
use crate::tape::Tape;
use crate::transition::{State, Transition};

/// A binary-alphabet Turing Machine with N non-halting states.
pub struct TuringMachine<const N: usize> {
    current_state: State,
    tape: Tape,
    transitions: [Transition; N],
}

impl<const N: usize> TuringMachine<N> {
    pub fn new(transitions: [Transition; N]) -> Self {
        debug_assert!(N > 0);

        Self {
            current_state: State::start(),
            tape: Tape::new(),
            transitions,
        }
    }

    pub fn run(&mut self) {
        while self.current_state != State::Halt {
            let symbol = self.tape.read();
            let (symbol, direction, new_state) =
                self.transitions[self.current_state as usize - 1].action_of(symbol);

            self.tape.write(symbol);
            self.tape.move_head(direction);
            self.current_state = new_state;
        }
    }
}

impl<const N: usize> Display for TuringMachine<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut transitions = self.transitions.iter();
        write!(f, "{}", transitions.next().unwrap())?;

        for t in transitions {
            write!(f, "{}", t)?;
        }

        Ok(())
    }
}
