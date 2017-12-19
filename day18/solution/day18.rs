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

    // star 1:
    {
        use star1::*;
        let mut star1 = State::new(Registers::new(), &commands);
        let mut recieved = 0;
        loop {
            if let Res::Finished = star1.step() {
                recieved = star1.res();
                break;
            }
        }
        println!("Star 1: {}", recieved);
    }

    // star 2:
    {
        use star2::*;
        let mut a_sent = 0;
        let mut a = State::new(Registers::new().with_val('p',0), &commands);
        let mut b = State::new(Registers::new().with_val('p',1), &commands);

        loop {
            let step_a = a.step();
            if let Res::Send(val) = step_a {
                a_sent += 1;
                b.recieve(val);
            }

            let step_b = b.step();
            if let Res::Send(val) = step_b {
                a.recieve(val);
            }

            if let (Res::Stopped, Res::Stopped) = (step_a, step_b) {
                break;
            }
        }
    }

}

#[derive(Debug,Clone)]
pub struct Registers {
    registers: HashMap<Register,i64>
}

impl Registers {
    fn new() -> Registers {
        Registers { registers: HashMap::new() }
    }
    fn with_val(mut self, r: Register, v: i64) -> Registers {
        self.registers.insert(r, v);
        self
    }
    fn get_reg(&self, reg: &Register) -> i64 {
        *self.registers.get(reg).unwrap_or(&0)
    }
    fn get_val(&self, val: Val) -> i64 {
        match val {
            Val::Reg(ref r) => self.get_reg(r),
            Val::Num(n) => n
        }
    }
    fn update_register<F: Fn(i64) -> i64>(&mut self, reg: Register, func: F) {
        let entry = self.registers.entry(reg).or_insert(0);
        *entry = func(*entry);
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Command {
    Snd(Register),
    Set(Register, Val),
    Add(Register, Val),
    Mul(Register, Val),
    Mod(Register, Val),
    Rcv(Register),
    Jgz(Register, Val)
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Val {
    Reg(Register),
    Num(i64)
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

// **********
// * Star 1 *
// **********
mod star1 {

    use super::*;

    pub struct State<'a> {
        registers: super::Registers,
        commands: &'a [Command],
        pos: i64,
        last_played: Option<i64>
    }
    impl <'a> State<'a> {
        pub fn new(registers: Registers, commands: &[Command]) -> State {
            State {
                registers,
                commands,
                pos: 0,
                last_played: None
            }
        }
        pub fn step(&mut self) -> Res {
            let cmd = if self.pos < 0 || self.pos >= self.commands.len() as i64 {
                return Res::Finished;
            } else {
                &self.commands[self.pos as usize]
            };
            let mut next = 1;

            match *cmd {
                Snd(reg) => {
                    self.last_played = Some(self.registers.get_reg(&reg));
                },
                Set(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |_| v);
                },
                Add(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r+v);
                },
                Mul(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r*v);
                },
                Mod(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r%v);
                },
                Rcv(reg) => {
                    if self.registers.get_reg(&reg) > 0 {
                        return Res::Finished
                    }
                },
                Jgz(reg, val) => {
                    if self.registers.get_reg(&reg) > 0 {
                        let v = self.registers.get_val(val);
                        next = v;
                    }
                }
            };

            self.pos += next;
            Res::Continue
        }
        pub fn res(&self) -> i64 {
            self.last_played.unwrap_or(0)
        }
    }

    #[derive(Debug,Copy,Clone,Eq,PartialEq)]
    pub enum Res {
        Finished,
        Continue
    }

}

// **********
// * Star 2 *
// **********
pub mod star2 {

    use std::collections::vec_deque::VecDeque;

    use super::*;

    pub struct State<'a> {
        pub registers: super::Registers,
        recieved: VecDeque<i64>,
        commands: &'a [Command],
        pos: i64
    }
    impl <'a> State<'a> {
        pub fn new(registers: Registers, commands: &[Command]) -> State {
            State {
                registers,
                commands,
                recieved: VecDeque::new(),
                pos: 0
            }
        }
        pub fn recieve(&mut self, val: i64) {
            self.recieved.push_back(val);
        }
        pub fn step(&mut self) -> Res {
            let cmd = if self.pos < 0 || self.pos >= self.commands.len() as i64 {
                return Res::Stopped;
            } else {
                &self.commands[self.pos as usize]
            };
            let mut next = 1;

            match *cmd {
                Snd(reg) => {
                    let val = self.registers.get_reg(&reg);
                    // increment position and send the value at reg:
                    self.pos += 1;
                    return Res::Send(val);
                },
                Set(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |_| v);
                },
                Add(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r+v);
                },
                Mul(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r*v);
                },
                Mod(reg, val) => {
                    let v = self.registers.get_val(val);
                    self.registers.update_register(reg, |r| r%v);
                },
                Rcv(reg) => {
                    if let Some(v) = self.recieved.pop_front() {
                        // set register to first recieved value if there is one:
                        self.registers.update_register(reg, |_| v);
                    } else {
                        // else, don't advance and block until value exists:
                        return Res::Stopped;
                    }
                },
                Jgz(reg, val) => {
                    if self.registers.get_reg(&reg) > 0 {
                        let v = self.registers.get_val(val);
                        next = v;
                    }
                }
            };

            self.pos += next;
            Res::Continue
        }
    }

    #[derive(Debug,Copy,Clone,Eq,PartialEq)]
    pub enum Res {
        Send(i64),
        Continue,
        Stopped
    }

}