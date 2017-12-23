extern crate primal;

use std::env;
use std::fs;
use std::io::Read;
use std::ops::{Index,IndexMut};
use self::Command::*;
use self::Val::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let commands: Vec<Command> = content.lines().filter_map(parse_command).collect();

    let mut registers = Registers::new();
    let mut runner = Runner::new(&commands);
    let mut mul_seen = 0;
    while let Some(cmd) = runner.step(&mut registers) {
        if let Mul{..} = cmd {
            mul_seen += 1
        }
    }
    println!("Star 1: {}", mul_seen);

    let max = 125400;
    let mut n = 108400;
    let mut non_primes = 0;
    while n <= max {
        if !primal::is_prime(n) {
            non_primes += 1;
        }
        n += 17;
    }
    println!("Star 2: {}", non_primes);

}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Runner<'a> {
    commands: &'a [Command],
    position: i64
}
impl <'a> Runner<'a> {
    fn new(cmds: &[Command]) -> Runner {
        Runner { commands: cmds, position: 0}
    }
    fn step(&mut self, registers: &mut Registers) -> Option<Command> {

        let cmd = if self.position < 0 || self.position as usize >= self.commands.len() {
            return None;
        } else {
            self.commands[self.position as usize]
        };

        let mut jump = 1;
        match cmd {
            Set(idx, val) => {
                let v = registers.get_val(val);
                registers[idx] = v;
            },
            Sub(idx, val) => {
                let v = registers.get_val(val);
                registers[idx] -= v;
            },
            Mul(idx, val) => {
                let v = registers.get_val(val);
                registers[idx] *= v;
            },
            Jgz(v1, v2) => {
                if registers.get_val(v1) != 0 {
                    jump = registers.get_val(v2);
                }
            }
        }

        self.position += jump;
        Some(cmd)
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Registers {
    data: Vec<i64>
}
impl Registers {
    fn new() -> Registers {
        Registers { data: vec![0;8] }
    }
    fn get_val(&self, val: Val) -> i64 {
        match val {
            Reg(idx) => self[idx],
            Num(n) => n
        }
    }
}
impl Index<u8> for Registers {
    type Output = i64;
    fn index(&self, idx: u8) -> &i64 {
        self.data.get(idx as usize).unwrap_or(&0)
    }
}
impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, idx: u8) -> &mut i64 {
        self.data.get_mut(idx as usize).unwrap()
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Command {
    Set(Register, Val),
    Sub(Register, Val),
    Mul(Register, Val),
    Jgz(Val, Val)
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Val {
    Reg(Register),
    Num(i64)
}

type Register = u8;

fn parse_command(s: &str) -> Option<Command> {
    let mut bits = s.split_whitespace();
    let cmd = bits.next()?;
    let snd = bits.next()?;
    let reg = snd.chars().next()?;
    let val = bits.next().and_then(parse_val);

    Some(match cmd {
        "set" => Set(parse_register_char(reg), val?),
        "sub" => Sub(parse_register_char(reg), val?),
        "mul" => Mul(parse_register_char(reg), val?),
        "jnz" => Jgz(parse_val(snd)?, val?),
        _ => return None
    })
}

fn parse_val(s: &str) -> Option<Val> {
    let c = s.chars().next()?;
    Some(if c.is_alphabetic() {
        Val::Reg(parse_register_char(c))
    } else {
        Val::Num(s.parse().ok()?)
    })
}

fn parse_register_char(c: char) -> u8 {
    c as u8 - 97
}