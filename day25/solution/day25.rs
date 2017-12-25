mod input;

use std::collections::HashSet;
use input::*;

fn main() {

    let blueprint = input();

    let mut machine = TuringMachine::new(&blueprint);
    let mut tape = HashSet::new();
    for _ in 0..blueprint.perform_checksum_after {
        machine.step(&mut tape);
    }

    println!("Star 1: {}", tape.iter().count());

}

struct TuringMachine<'a> {
    position: isize,
    state: char,
    commands: &'a Commands
}

impl <'a> TuringMachine<'a> {
    fn new(blueprint: &Blueprint) -> TuringMachine {
        TuringMachine {
            position: 0,
            state: blueprint.begin_in,
            commands: &blueprint.commands
        }
    }
    fn step(&mut self, tape: &mut Tape) {
        let state = &self.commands[&self.state];
        let cmd = if tape.contains(&self.position) { &state.if_true } else { &state.if_false };

        if cmd.write {
            tape.insert(self.position);
        } else {
            tape.remove(&self.position);
        }
        if cmd.move_to == Direction::Right {
            self.position += 1;
        } else {
            self.position -= 1
        }
        self.state = cmd.continue_with;
    }
}

type Tape = HashSet<isize>;