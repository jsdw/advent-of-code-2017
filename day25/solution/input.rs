use std::collections::HashMap;
use self::Direction::*;

pub fn input() -> Blueprint {
    Blueprint {
        begin_in: 'A',
        perform_checksum_after: 12994925,
        commands: vec![
            ('A', State {
                if_false: Command {
                    write: true,
                    move_to: Right,
                    continue_with: 'B'
                },
                if_true: Command {
                    write: false,
                    move_to: Left,
                    continue_with: 'F'
                }
            }),
            ('B', State {
                if_false: Command {
                    write: false,
                    move_to: Right,
                    continue_with: 'C'
                },
                if_true: Command {
                    write: false,
                    move_to: Right,
                    continue_with: 'D'
                }
            }),
            ('C', State {
                if_false: Command {
                    write: true,
                    move_to: Left,
                    continue_with: 'D'
                },
                if_true: Command {
                    write: true,
                    move_to: Right,
                    continue_with: 'E'
                }
            }),
            ('D', State {
                if_false: Command {
                    write: false,
                    move_to: Left,
                    continue_with: 'E'
                },
                if_true: Command {
                    write: false,
                    move_to: Left,
                    continue_with: 'D'
                }
            }),
            ('E', State {
                if_false: Command {
                    write: false,
                    move_to: Right,
                    continue_with: 'A'
                },
                if_true: Command {
                    write: true,
                    move_to: Right,
                    continue_with: 'C'
                }
            }),
            ('F', State {
                if_false: Command {
                    write: true,
                    move_to: Left,
                    continue_with: 'A'
                },
                if_true: Command {
                    write: true,
                    move_to: Right,
                    continue_with: 'A'
                }
            })
        ].into_iter().collect()
    } 
}

#[derive(Debug,Clone)]
pub struct Blueprint {
    pub begin_in: char,
    pub perform_checksum_after: usize,
    pub commands: Commands
}

pub type Commands = HashMap<char,State>;

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
pub struct State {
    pub if_false: Command,
    pub if_true: Command
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub struct Command {
    pub write: bool,
    pub move_to: Direction,
    pub continue_with: char
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub enum Direction {
    Left,
    Right
}
