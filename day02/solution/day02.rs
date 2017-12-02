use std::env;
use std::fs;
use std::io::Read;

// --- Day 2: Corruption Checksum ---
//
// As you walk through the door, a glowing humanoid shape yells in your direction.
// "You there! Your state appears to be idle. Come help us repair the corruption in
// this spreadsheet - if we take another millisecond, we'll have to display an
// hourglass cursor!"
//
// The spreadsheet consists of rows of apparently-random numbers. To make sure the
// recovery process is on the right track, they need you to calculate the
// spreadsheet's checksum. For each row, determine the difference between the
// largest value and the smallest value; the checksum is the sum of all of these
// differences.
//
// For example, given the following spreadsheet:
//
// 5 1 9 5
// 7 5 3 2
// 4 6 8
//
// The first row's largest and smallest values are 9 and 1,
// and their difference is 8. The second row's largest and smallest values are 7
// and 3, and their difference is 4. The third row's difference is 6. In this
// example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
//
// What is the checksum for the spreadsheet in your puzzle input?
//
// --- Part Two ---
//
// "Great work; looks like we're on the right track after all. Here's a star for
// your effort." However, the program seems a little worried. Can programs be
// worried?
//
// "Based on what we're seeing, it looks like all the User wanted is some
// information about the evenly divisible values in the spreadsheet. Unfortunately,
// none of us are equipped for that kind of calculation - most of us specialize in
// bitwise operations."
//
// It sounds like the goal is to find the only two numbers in each row where one
// evenly divides the other - that is, where the result of the division operation
// is a whole number. They would like you to find those numbers on each line,
// divide them, and add up each line's result.
//
// For example, given the following spreadsheet:
//
// 5 9 2 8
// 9 4 7 3
// 3 8 6 5
//
// In the first row, the only two numbers that evenly
// divide are 8 and 2; the result of this division is 4. In the second row, the two
// numbers are 9 and 3; the result is 3. In the third row, the result is 2. In this
// example, the sum of the results would be 4 + 3 + 2 = 9.
//
// What is the sum of each row's result in your puzzle input?

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    let mut content = String::new();

    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    // Star 1:
    let mut sum = 0;
    for line in content.lines() {

        let (min,max) = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .fold((std::u64::MAX, std::u64::MIN), |(min,max), n| {
                let max2 = if n > max { n } else { max };
                let min2 = if n < min { n } else { min };
                (min2, max2)
            });

        sum += max - min;
    }
    println!("Star 1: {}", sum);

    // Star 2:
    let mut sum = 0;
    for line in content.lines() {

        let row: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        sum += find_even_divides(&row);

    }
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
        if n >= first && (n / first) * first == n { return n / first };
        if n < first && (first / n) * n == first { return first / n };

    }

    // otherwise, try again with the rest:
    return find_even_divides(&input[1..]);

}