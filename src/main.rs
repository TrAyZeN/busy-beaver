use busy_beaver::transition::{Action, Direction, State, Transition};
use busy_beaver::turing_machine::{generate_busy_beaver, TuringMachine};

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
    println!("{} {:?}", tm, tm.run(1000));

    let tm = generate_busy_beaver::<3>(1000);
    println!("{} {:?}", tm, tm.run(1000));
}
