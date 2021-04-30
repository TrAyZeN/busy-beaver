mod tape;
mod transition;
mod turing_machine;

use transition::{Action, Direction, State, Transition};
use turing_machine::TuringMachine;

fn main() {
    let mut tm = TuringMachine::new([Transition::new(
        Action::new(1, Direction::Left, State::A),
        Action::new(0, Direction::Left, State::A),
    )]);
    println!("{}", tm);
    tm.run();
}
