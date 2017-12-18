use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use self::Command::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let commands: Vec<Command> = content.lines().filter_map(parse_command).collect();

    let state = State::new(commands);


    let mut recieved = 0;
    for res in state {
        if let Some(val) = res {
            recieved = val;
            break;
        }
    }
    println!("{:?}", recieved);

}

struct State {
    commands: Vec<Command>,
    registers: HashMap<Register,isize>,
    last_played: Option<isize>,
    pos: isize
}
impl State {
    fn step(&mut self) -> Res {
        let cmd = if let Some(cmd) = self.get_cmd() { cmd } else { return Res::Finished; };
        let mut next = 1;

        match cmd {
            Snd(reg) => {
                self.last_played = Some(self.get_reg(&reg));
            },
            Set(reg, val) => {
                let v = self.get_val(val);
                self.registers.insert(reg, v);
            },
            Add(reg, val) => {
                let v = self.get_val(val);
                *self.registers.entry(reg).or_insert(0) += v;
            },
            Mul(reg, val) => {
                let v = self.get_val(val);
                let curr = self.registers.entry(reg).or_insert(0);
                *curr = *curr * v;
            },
            Mod(reg, val) => {
                let v = self.get_val(val);
                let curr = self.registers.entry(reg).or_insert(0);
                *curr = *curr % v;
            },
            Rcv(reg) => {
                if self.get_reg(&reg) > 0 {
                    return Res::Recieved(self.last_played.unwrap_or(0))
                }
            },
            Jgz(reg, val) => {
                if self.get_reg(&reg) > 0 {
                    let v = self.get_val(val);
                    next = v;
                }
            }
        };

        self.pos += next;
        Res::Continue
    }
    fn get_cmd(&self) -> Option<Command> {
        if self.pos < 0 || self.pos >= self.commands.len() as isize {
            None
        } else {
            Some(self.commands[self.pos as usize])
        }
    }
    fn get_reg(&self, reg: &Register) -> isize {
        *self.registers.get(reg).unwrap_or(&0)
    }
    fn get_val(&self, val: Val) -> isize {
        match val {
            Val::Reg(ref r) => self.get_reg(r),
            Val::Num(n) => n
        }
    }
    fn new(cmds: Vec<Command>) -> State {
        State {
            commands: cmds,
            registers: HashMap::new(),
            last_played: None,
            pos: 0
        }
    }
    fn reset(&mut self) {
        self.registers = HashMap::new();
        self.last_played = None;
        self.pos = 0;
    }
}
impl Iterator for State {
    type Item = Option<isize>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.step() {
            Res::Finished => None,
            Res::Continue => Some(None),
            Res::Recieved(n) => Some(Some(n))
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Res {
    Finished,
    Recieved(isize),
    Continue
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Command {
    Snd(Register),
    Set(Register, Val),
    Add(Register, Val),
    Mul(Register, Val),
    Mod(Register, Val),
    Rcv(Register),
    Jgz(Register, Val)
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Val {
    Reg(Register),
    Num(isize)
}

type Register = char;

fn parse_command(s: &str) -> Option<Command> {
    let mut bits = s.split_whitespace();
    let cmd = bits.next()?;
    let reg = bits.next()?.chars().next()?;
    let val = bits.next().and_then(parse_val);

    Some(match cmd {
        "snd" => Snd(reg),
        "set" => Set(reg, val?),
        "add" => Add(reg, val?),
        "mul" => Mul(reg, val?),
        "mod" => Mod(reg, val?),
        "rcv" => Rcv(reg),
        "jgz" => Jgz(reg, val?),
        _ => return None
    })
}

fn parse_val(s: &str) -> Option<Val> {
    let c = s.chars().next()?;
    Some(if c.is_alphabetic() {
        Val::Reg(c)
    } else {
        Val::Num(s.parse().ok()?)
    })
}