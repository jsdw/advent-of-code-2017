use std::env;
use std::fs;
use std::io::Read;
use self::Direction::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let directions: Vec<Direction> = content.split(',').map(parse_direction).collect();

    println!("Star 1: {}", star1(&directions));
    println!("Star 2: {}", star2(&directions));

}

fn star1(directions: &Vec<Direction>) -> usize {

    let mut counts: [usize;6] = [0;6];
    for &d in directions {
        counts[d as usize] += 1
    }

    distance(counts)
}

fn star2(directions: &Vec<Direction>) -> usize {

    let mut max_distance = 0;
    let mut counts: [usize;6] = [0;6];
    for &d in directions {
        counts[d as usize] += 1;
        let d = distance(counts);
        if d > max_distance { max_distance = d }
    }

    max_distance
}

fn distance(mut counts: [usize;6]) -> usize {

     // cancel out opposite directions:
    for idx in 0..6 {
        let opp = (idx + 3) % 6;
        if counts[idx] <= counts[opp] {
            counts[opp] -= counts[idx];
            counts[idx] = 0;
        }
    }
    // merge diagonals:
    for idx in 0..6 {
        let next = if idx == 5 { 0 } else { idx + 1 };
        let prev = if idx == 0 { 5 } else { idx - 1 };
        let adj = counts[next].min(counts[prev]);
        counts[next] -= adj;
        counts[idx] += adj;
        counts[prev] -= adj;
    }

    counts.into_iter().sum()
}

fn parse_direction(dir: &str) -> Direction {
    match dir {
        "n"  => N,
        "ne" => NE,
        "se" => SE,
        "s"  => S,
        "sw" => SW,
        "nw" => NW,
        _ => panic!("unexpected direction")
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Direction {
    N, NE, SE, S, SW, NW
}