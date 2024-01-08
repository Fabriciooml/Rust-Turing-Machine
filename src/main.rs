mod state;
mod tape;
mod direction;
mod transition;
mod machine;

fn main() {
    let mut machine : machine::TuringMachine = add("$|||&|||||||#");
    machine.process();
}

fn add(word: &str) -> machine::TuringMachine {
    let tape = tape::Tape::new("$|&#", word);
    let states = vec![
        state::State::new('0', state::StateType::Start),
        state::State::new('1', state::StateType::Empty),
        state::State::new('2', state::StateType::Empty),
        state::State::new('3', state::StateType::Empty),
        state::State::new('4', state::StateType::Empty),
        state::State::new('f', state::StateType::Final)
    ];

    let transitions = vec![
        transition::Transition::new('0', '$', '1', '$', direction::Direction::Right),
        transition::Transition::new('1', '#', 'f', '#', direction::Direction::None),
        transition::Transition::new('1', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '&', '2', '|', direction::Direction::Right),
        transition::Transition::new('2', '|', '2', '|', direction::Direction::Right),
        transition::Transition::new('2', '#', '3', '#', direction::Direction::Left),
        transition::Transition::new('3', '|', '4', '#', direction::Direction::Left),
        transition::Transition::new('4', '|', '4', '|', direction::Direction::Left),
        transition::Transition::new('4', '$', 'f', '$', direction::Direction::None)
    ];

    machine::TuringMachine::new(states, transitions, tape)
}