use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    let numbers = fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .bytes()
        .map(|b| b.unwrap() - 48)
        .collect::<Vec<u8>>();

    // star 1:
    println!("Star 1: {}", solve(&numbers, 1));

    // star 2:
    println!("Star 2: {}", solve(&numbers, numbers.len() / 2));

}

fn solve(input: &[u8], offset: usize) -> u64 {
    input.iter()
        .zip(input.iter().cycle().skip(offset))
        .map(|(&a,&b)| if a == b { a as u64 } else { 0 })
        .sum()
}