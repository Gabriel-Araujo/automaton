use crate::automaton::Automaton;

mod automaton;

fn main() {
    let mut automaton = Automaton::new();
    automaton.process_input("program ".to_string());
}
