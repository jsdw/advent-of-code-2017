use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashSet;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    // Get memory banks:
    let mut banks: Vec<u8> = content
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Star 1; loop until we see a bank twice.
    let mut seen = HashSet::new();
    let mut count = 0;
    loop {
        step(&mut banks);
        count += 1;
        if !seen.insert(banks.clone()) {
            break;
        }
    }
    println!("Star 1: {}", count);

    // Star 2; keep going until we see the same bank a third time.
    let last = banks.clone();
    let mut count = 0;
    loop {
        step(&mut banks);
        count += 1;
        if last == banks {
            break;
        }
    }
    println!("Star 2: {}", count);

}

// Perform one step of updating the memory banks.
fn step(banks: &mut [u8]) {
    let max_index = banks.iter().enumerate().rev().max_by_key(|v| v.1).unwrap().0;
    let max_val = banks[max_index] as usize;
    banks[max_index] = 0;

    for idx in (0..banks.len()).cycle().skip(max_index+1).take(max_val) {
        banks[idx] += 1
    }
}

