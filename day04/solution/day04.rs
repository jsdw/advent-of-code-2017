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

    // Get vector of vectors of Strings for each line:
    let lines: Vec<Vec<String>> = content
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_owned()).collect())
        .collect();

    // Star 1: filter phrases with dupe words; count remaining:
    let star1 = lines.iter().filter(|&line| {
        let mut used = HashSet::new();
        line.iter().filter(|&word| !used.insert(word)).count() == 0
    }).count();
    println!("Star 1: {}", star1);

    // Star 2: filter phrases with anagram words; count remaining:
    let star2 = lines.iter().filter(|&line| {
        let mut used = HashSet::new();
        for word in line {
            let mut bytes = word.clone().into_bytes();
            bytes.sort();
            if !used.insert(bytes) {
                return false
            }
        }
        true
    }).count();
    println!("Star 2: {}", star2);

}

