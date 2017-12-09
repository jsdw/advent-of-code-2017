extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use self::Comparison::*;

// --- Day 8: I Heard You Like Registers ---
//
// You receive a signal directly from the CPU. Because of your recent assistance
// with jump instructions, it would like you to compute the result of a series of
// unusual register instructions.
//
// Each instruction consists of several parts: the register to modify, whether to
// increase or decrease that register's value, the amount by which to increase or
// decrease it, and a condition. If the condition fails, skip the instruction
// without modifying the register. The registers all start at 0. The instructions
// look like this:
//
// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10
// These instructions would be processed as follows:
//
// Because a starts at 0, it is not greater than 1, and so b is not modified.
// a is increased by 1 (to 1) because b is less than 5 (it is 0).
// c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
// c is increased by -20 (to -10) because c is equal to 10.
// After this process, the largest value in any register is 1.
//
// You might also encounter <= (less than or equal to) or != (not equal to).
// However, the CPU doesn't have the bandwidth to tell you what all the registers
// are named, and leaves that to you to determine.
//
// What is the largest value in any register after completing the instructions in
// your puzzle input?
//
// --- Part Two ---
//
// To be safe, the CPU also needs to know the highest value held in any register
// during this process so that it can decide how much memory to allocate to these
// operations. For example, in the above instructions, the highest value ever held
// was 10 (in register c after the third instruction was evaluated).

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