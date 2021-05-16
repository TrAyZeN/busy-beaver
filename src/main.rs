use busy_beaver::transition::{Action, Direction, State, Transition};
use busy_beaver::turing_machine::TuringMachine;

fn main() {
    let tm = TuringMachine::new([
        Transition::new(
            Action::new(1, Direction::Left, State::A),
            Action::new(0, Direction::Left, State::A),
        ),
        Transition::new(
            Action::new(1, Direction::Left, State::A),
            Action::new(0, Direction::Left, State::A),
        ),
    ]);
    println!("{}", tm);
    tm.run();
}
