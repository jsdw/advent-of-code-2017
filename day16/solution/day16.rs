use std::env;
use std::fs;
use std::io::Read;
use self::Move::*;

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

