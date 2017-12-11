use std::env;
use std::fs;
use std::io::Read;
use self::Direction::*;

// --- Day 11: Hex Ed ---
//
// Crossing the bridge, you've barely reached the other side of the stream when a
// program comes up to you, clearly in distress. "It's my child process," she says,
// "he's gotten lost in an infinite grid!"
//
// Fortunately for her, you have plenty of experience with infinite grids.
//
// Unfortunately for you, it's a hex grid.
//
// The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be
// found to the north, northeast, southeast, south, southwest, and northwest:
//
//   \ n  /
// nw +--+ ne
//   /    \
// -+      +-
//   \    /
// sw +--+ se
//   / s  \
//
// You have the path the child process took. Starting where he started, you need to
// determine the fewest number of steps required to reach him. (A "step" means to
// move from the hex you are in to any adjacent hex.)
//
// For example:
//
// ne,ne,ne is 3 steps away.
// ne,ne,sw,sw is 0 steps away (back where you started).
// ne,ne,s,s is 2 steps away (se,se).
// se,sw,se,sw,sw is 3 steps away (s,s,sw).
// Your puzzle answer was 796.
//
// --- Part Two ---
//
// How many steps away is the furthest he ever got from his starting position?

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