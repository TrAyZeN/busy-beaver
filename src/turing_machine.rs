use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::fmt::{self, Display};

use crate::tape::Tape;
use crate::transition::{Action, Direction, PartialTransition, State, Transition};

/// A binary-alphabet Turing Machine with N non-halting states.
#[derive(Debug)]
pub struct TuringMachine<const N: usize> {
    transitions: [Transition; N],
}

impl<const N: usize> TuringMachine<N> {
    /// Creates a new binary-alphabet Turing Machine with the given transitions.
    ///
    /// # Panics
    /// Panics if `N < 2`.
    #[inline]
    #[must_use]
    pub fn new(transitions: [Transition; N]) -> Self {
        assert!(N >= 2);

        Self { transitions }
    }

    /// Runs the turing machine on the blank input for a maximum number of steps.
    /// Returns `Some(productivity)` if the machine did halt and `None` otherwise.
    pub fn run(&self, max_steps: usize) -> Option<usize> {
        let mut current_state = State::start();
        let mut tape = Tape::new();

        let mut i = 0;
        while !current_state.is_halting() && i < max_steps {
            let (symbol, direction, new_state) =
                self.transitions[current_state as usize - 1].get_action_of(tape.read());

            tape.write(symbol);
            tape.move_head(direction);
            current_state = new_state;
            i += 1;
        }

        current_state.is_halting().then(|| tape.count_ones())
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

impl<const N: usize> From<PartialTuringMachine<N>> for TuringMachine<N> {
    fn from(machine: PartialTuringMachine<N>) -> Self {
        Self::new(
            machine
                .transitions
                .iter()
                .map(|&transition| transition.into())
                .collect::<Vec<Transition>>()
                .try_into()
                .unwrap(),
        )
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
    /// # Panics
    /// Panics if `N < 2`.
    #[inline]
    #[must_use]
    pub fn new(transitions: [PartialTransition; N]) -> Self {
        assert!(N >= 2);

        Self { transitions }
    }

    /// Adds the given transition to the `PartialTuringMachine`
    #[inline]
    pub fn add_transition(&mut self, state: State, symbol: u8, action: Action) {
        self.transitions[state as usize - 1].set_action_of(symbol, Some(action));
    }

    /// Checks if the machine is N-state full
    #[inline]
    #[must_use]
    fn is_n_state_full(&self) -> bool {
        self.transitions
            .iter()
            .map(|t| t.get_action_of(0).or(t.get_action_of(1)).is_some())
            .count()
            .eq(&N)
    }

    /// Checks if the machine contains `N` transitions that moves the head to
    /// the right when a zero is read with the given transition
    #[must_use]
    fn is_0_dextrous_with(&self, state: State, symbol: u8, action: Action) -> bool {
        self.transitions
            .iter()
            .enumerate()
            .filter(|(s, t)| {
                if *s + 1 == state as usize && symbol == 0 {
                    action.get_direction() == Direction::Right
                } else {
                    t.get_action_of(0)
                        .map_or(false, |(_, direction, _)| direction == Direction::Right)
                }
            })
            .count()
            .eq(&N)
    }

    /// Counts the number of specified transitions of the machine.
    #[must_use]
    fn count_specified_transitions(&self) -> usize {
        self.transitions
            .iter()
            .map(|t| t.count_specified_actions())
            .sum()
    }

    /// Gets the first state with unspecified transitions or if none the last state.
    #[must_use]
    fn state_choice_limit(&self) -> State {
        let mut s = N;

        while s > 0 && self.transitions[s - 1].count_specified_actions() == 0 {
            s -= 1;
        }

        State::from(s as u8)
    }

    /// Runs the `PartialTuringMachine` on a blank tape
    pub fn run(&self, max_steps: usize) -> Result<Option<usize>, (State, u8)> {
        let mut current_state = State::start();
        let mut tape = Tape::new();

        let mut i = 0;
        while !current_state.is_halting() && i < max_steps {
            let (symbol, direction, new_state) = self.transitions[current_state as usize - 1]
                .get_action_of(tape.read())
                .ok_or((current_state, tape.read()))?;

            tape.write(symbol);
            tape.move_head(direction);
            current_state = new_state;
            i += 1;
        }

        Ok(current_state.is_halting().then(|| tape.count_ones()))
    }
}

impl<const N: usize> Display for PartialTuringMachine<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut transitions = self.transitions.iter();
        write!(f, "{}", transitions.next().unwrap())?;

        for t in transitions {
            write!(f, " {}", t)?;
        }

        Ok(())
    }
}

pub fn generate_busy_beaver<const N: usize>(max_steps: usize) -> TuringMachine<N> {
    let halting_action = Action::new(1, Direction::Right, State::Halt);
    let mut rng = thread_rng();
    let mut m = PartialTuringMachine::new([PartialTransition::default(); N]);

    m.add_transition(State::A, 0, Action::new(1, Direction::Right, State::B));

    if N >= 3 {
        m.add_transition(
            State::B,
            0,
            Action::new(rng.gen_range(0..=1), Direction::random(), State::C),
        );
    } else {
        m.add_transition(
            State::B,
            0,
            Action::new(
                rng.gen_range(0..=1),
                Direction::random(),
                State::random(State::A..=State::B),
            ),
        );
    }

    loop {
        println!("{}", m);
        match m.run(max_steps) {
            Ok(o) => {
                println!("{:?}", o);
                break;
            }
            Err((state, symbol)) => {
                println!("yeah");
                dbg!(m.is_n_state_full());
                if m.is_n_state_full() && !m.is_0_dextrous_with(state, symbol, halting_action) {
                    println!("endo");
                    m.add_transition(state, symbol, halting_action);
                    break;
                }

                let action = Action::new(
                    rng.gen_range(0..=1),
                    Direction::random(),
                    State::random(State::A..=m.state_choice_limit()),
                );
                if !m.is_0_dextrous_with(state, symbol, action) {
                    m.add_transition(state, symbol, action);
                }

                if m.count_specified_transitions() == 2 * N - 1 {
                    for s in 0..N {
                        if m.transitions[s].count_specified_actions() == 1 {
                            if m.transitions[s].get_action_of(0).is_none() {
                                m.add_transition(State::from(s as u8 + 1), 0, halting_action);
                            } else {
                                m.add_transition(State::from(s as u8 + 1), 1, halting_action);
                            }
                        }
                    }

                    break;
                }
            }
        }
    }

    m.into()
}
