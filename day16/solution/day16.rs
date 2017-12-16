use std::env;
use std::fs;
use std::io::Read;
use self::Move::*;

// --- Day 16: Permutation Promenade ---
//
// You come upon a very unusual sight; a group of programs here appear to be dancing.
//
// There are sixteen programs in total, named a through p. They start by standing
// in a line: a stands in position 0, b stands in position 1, and so on until p,
// which stands in position 15.
//
// The programs' dance consists of a sequence of dance moves:
//
// - Spin, written sX, makes X programs move from the end to the front, but maintain
// their order otherwise. (For example, s3 on abcde produces cdeab).
// - Exchange, written xA/B, makes the programs at positions A and B swap places.
// - Partner, written pA/B, makes the programs named A and B swap places.
//
// For example, with only five programs standing in a line (abcde), they could do
// the following dance:
//
// s1, a spin of size 1: eabcd.
// x3/4, swapping the last two programs: eabdc.
// pe/b, swapping programs e and b: baedc.
// After finishing their dance, the programs end up in order baedc.
//
// You watch the dance for a while and record their dance moves (your puzzle
// input). In what order are the programs standing after their dance?
//
// --- Part Two ---
//
// Now that you're starting to get a feel for the dance moves, you turn your
// attention to the dance as a whole.
//
// Keeping the positions they ended up in from their previous dance, the programs
// perform it again and again: including the first dance, a total of one billion
// (1000000000) times.
//
// In the example above, their second dance would begin with the order baedc, and
// use the same dance moves:
//
// s1, a spin of size 1: cbaed.
// x3/4, swapping the last two programs: cbade.
// pe/b, swapping programs e and b: ceadb.
//
// In what order are the programs standing after their billion dances?

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let moves: Vec<Move> = content.split(',').map(parse_move).collect();
    let original: Vec<char> = (0..16).map(|c| char::from(c + 97)).collect();

    let mut dancers = original.clone();
    for &m in &moves {
        step(m,&mut dancers);
    }
    println!("Star 1: {}", dancers.iter().collect::<String>());

    // For star 2, running 1billion rounds takes far too long,
    // but through a little experimentation I find that the
    // sequence repeats. So, find the number of steps taken
    // to repeat, and we know how many steps we need to actually
    // perform to equal the billionth result.
    let mut next_equal = 0;
    for idx in 1.. {
        if dancers == original {
            next_equal = idx;
            break;
        }
        for &m in &moves {
            step(m,&mut dancers);
        }
    }

    // use fresh dancers as the others have grown tired after
    // all of this dancing.
    let mut dancers = original.clone();
    for _ in 0..(1_000_000_000 % next_equal) {
        for &m in &moves {
            step(m,&mut dancers);
        }
    }
    println!("Star 2: {}", dancers.iter().collect::<String>());

}

fn step(m: Move, dancers: &mut Vec<char>) {
    match m {
        Spin(n) => {
            let last = dancers.len() - n;
            let mut rest = dancers.split_off(last);
            rest.extend_from_slice(&dancers);
            *dancers = rest;
        },
        Exchange(a, b) => {
            dancers.swap(a,b);
        },
        Partner(a, b) => {
            if let Some(a_idx) = find_idx(a, &dancers) {
            if let Some(b_idx) = find_idx(b, &dancers) {
                dancers.swap(a_idx, b_idx);
            }}
        }
    }
}

fn find_idx<T: PartialEq + Copy>(needle: T, input: &[T]) -> Option<usize> {
    input.iter().enumerate().find(|&(_,&c)| c == needle).map(|t| t.0)
}

fn parse_move(input: &str) -> Move {
    let rest = &input[1..];
    match input.chars().next().unwrap() {
        's' => {
            Spin( rest.parse().expect("Spin expects number") )
        },
        'x' => {
            let (a,b) = rest.split_at( rest.find('/').expect("Exchange /") );
            Exchange( a.parse().expect("Exchange a val"), b[1..].parse().expect("Exchange b val") )
        },
        'p' => {
            let (a,b) = rest.split_at( rest.find('/').expect("Exchange /") );
            Partner( a.chars().next().unwrap(), b[1..].chars().next().unwrap() )
        },
        _ => {
            panic!("Invalid input: {}", input);
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Move {
    Spin(usize),
    Exchange(usize,usize),
    Partner(char,char)
}

