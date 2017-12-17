use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    // Get vector of vectors of u64s for each line:
    let lines = content
        .lines()
        .map(|line| line.split_whitespace().filter_map(|s| s.parse().ok()).collect())
        .collect::<Vec<Vec<u64>>>();

    // Star 1: diff between max and min for each line:
    let sum: u64 = lines.iter().map(|l| l.iter().max().unwrap() - l.iter().min().unwrap()).sum();
    println!("Star 1: {}", sum);

    // Star 2: find perfect dividers and sum result for each line:
    let sum: u64 = lines.iter().map(|l| find_even_divides(&l)).sum();
    println!("Star 2: {}", sum);

}

fn find_even_divides(input: &[u64]) -> u64 {

    if input.len() == 0 {
        panic!("Did not find even divides");
    }

    let first = input[0];
    for &n in &input[1..] {

        // if we can cleanly divide one number by the other, return the
        // result of that division..
        if n >= first && n % first == 0 { return n / first };
        if n < first && first % n == 0 { return first / n };

    }

    // otherwise, try again with the rest:
    return find_even_divides(&input[1..]);

}