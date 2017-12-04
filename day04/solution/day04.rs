use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashSet;

// --- Day 4: High-Entropy Passphrases ---
//
// A new system policy has been put in place that requires all accounts to
// use a passphrase instead of simply a password. A passphrase consists of
// a series of words (lowercase letters) separated by spaces.
//
// To ensure security, a valid passphrase must contain no duplicate words.
//
// For example:
//
// aa bb cc dd ee is valid.
// aa bb cc dd aa is not valid - the word aa appears more than once.
// aa bb cc dd aaa is valid - aa and aaa count as different words.
//
// The system's full passphrase list is available as your puzzle input. How
// many passphrases are valid?
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// For added security, yet another system policy has been put in place. Now,
// a valid passphrase must contain no two words that are anagrams of each
// other - that is, a passphrase is invalid if any word's letters can be
// rearranged to form any other word in the passphrase.
//
// For example:
//
// abcde fghij is a valid passphrase.
// abcde xyz ecdab is not valid - the letters from the third word can be
// rearranged to form the first word.
// a ab abc abd abf abj is a valid passphrase, because all letters need to
// be used when forming another word.
//
// iiii oiii ooii oooi oooo is valid.
// oiii ioii iioi iiio is not valid
//     - any of these words can be rearranged to form any other word.
//
// Under this new system policy, how many passphrases are valid?

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

