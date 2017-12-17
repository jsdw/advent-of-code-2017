extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use self::Comparison::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let instructions: Vec<Instruction> = content.lines().filter_map(parse_instruction).collect();
    let mut registers: Registers = HashMap::new();

    let mut max: isize = 0;
    let mut last: isize = 0;
    for inst in &instructions {
        eval_step(inst, &mut registers);
        last = registers.iter().map(|v| *v.1).max().unwrap_or(0);
        if max < last { max = last };
    }

    println!("Star 1: {}", last);
    println!("Star 2: {}", max);

}

fn eval_step(inst: &Instruction, registers: &mut Registers) {
    if eval_condition(&inst.condition, registers) {
        let val = match inst.command {
            Cmd::Inc => inst.value,
            Cmd::Dec => -inst.value
        };
        *registers.entry(inst.register.to_owned()).or_insert(0) += val;
    }
}

fn eval_condition(condition: &Condition, registers: &mut Registers) -> bool {
    let val: isize = registers.get(&condition.register).unwrap_or(&0).clone();
    let target = condition.value;
    match condition.comparison {
        GT => val > target,
        LT => val < target,
        GE => val >= target,
        LE => val <= target,
        EQ => val == target,
        NE => val != target
    }
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) ([>=<!]+) (-?[0-9]+)$").unwrap();
    }
    let caps = RE.captures(line)?;

    Some(Instruction{
        register: caps[1].to_owned(),
        command: if &caps[2] == "inc" { Cmd::Inc } else { Cmd::Dec },
        value: caps[3].parse().ok()?,
        condition: Condition {
            register: caps[4].to_owned(),
            comparison: parse_comparison(&caps[5])?,
            value: caps[6].parse().ok()?
        }
    })
}

fn parse_comparison(cond: &str) -> Option<Comparison> {
    Some(match cond {
        ">" => GT,
        "<" => LT,
        ">=" => GE,
        "<=" => LE,
        "==" => EQ,
        "!=" => NE,
        _ => { return None }
    })
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct Instruction {
    register: Register,
    command: Cmd,
    value: isize,
    condition: Condition
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Cmd {
    Inc,
    Dec
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct Condition {
    register: Register,
    comparison: Comparison,
    value: isize
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Comparison {
    GT,
    LT,
    GE,
    LE,
    EQ,
    NE
}

type Register = String;
type Registers = HashMap<String,isize>;