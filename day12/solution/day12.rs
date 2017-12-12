use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashSet;

// --- Day 12: Digital Plumber ---
//
// Walking along the memory banks of the stream, you find a small village that is
// experiencing a little confusion: some programs can't communicate with each
// other.
//
// Programs in this village communicate using a fixed system of pipes. Messages are
// passed between programs using these pipes, but most programs aren't connected to
// each other directly. Instead, programs pass messages between each other until
// the message reaches the intended recipient.
//
// For some reason, though, some of these messages aren't ever reaching their
// intended recipient, and the programs suspect that some pipes are missing. They
// would like you to investigate.
//
// You walk through the village and record the ID of each program and the IDs with
// which it can communicate directly (your puzzle input). Each program has one or
// more programs with which it can communicate, and these pipes are bidirectional;
// if 8 says it can communicate with 11, then 11 will say it can communicate with
// 8.
//
// You need to figure out how many programs are in the group that contains program
// ID 0.
//
// For example, suppose you go door-to-door like a travelling salesman and record
// the following list:
//
// 0 <-> 2
// 1 <-> 1
// 2 <-> 0, 3, 4
// 3 <-> 2, 4
// 4 <-> 2, 3, 6
// 5 <-> 6
// 6 <-> 4, 5
//
// In this example, the following programs are in the group that contains program
// ID 0:
//
// Program 0 by definition.
// Program 2, directly connected to program 0.
// Program 3 via program 2.
// Program 4 via program 2.
// Program 5 via programs 6, then 4, then 2.
// Program 6 via programs 4, then 2.
//
// Therefore, a total of 6 programs are in this group; all but program 1, which has
// a pipe that connects it to itself.
//
// How many programs are in the group that contains program ID 0?
//
// Your puzzle answer was 115.
//
// --- Part Two ---
//
// There are more programs than just the ones in the group containing program ID 0.
// The rest of them have no way of reaching that group, and still might have no way
// of reaching each other.
//
// A group is a collection of programs that can all communicate via pipes either
// directly or indirectly. The programs you identified just a moment ago are all
// part of the same group. Now, they would like you to determine the total number
// of groups.
//
// In the example above, there were 2 groups: one consisting of programs
// 0,2,3,4,5,6, and the other consisting solely of program 1.
//
// How many groups are there in total?

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let connections: Connections = content.lines().map(parse_line).collect();

    println!("Star 1: {:?}", count_links(0,&connections,&mut HashSet::new()));
    println!("Star 2: {:?}", count_groups(&connections));

}

fn parse_line(line: &str) -> Vec<usize> {
    line.rsplit(" <-> ")
        .next().expect("connected indexes")
        .split(", ")
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn count_links(idx: usize, connections: &Connections, seen: &mut HashSet<usize>) -> usize {
    if seen.contains(&idx) { return 0 }
    seen.insert(idx);
    let child_count: usize = connections[idx].iter().map(|&idx| count_links(idx, connections, seen)).sum();
    child_count + 1
}

fn count_groups(connections: &Connections) -> usize {
    let mut seen = HashSet::new();
    let mut groups = 0;
    for idx in 0..connections.len() {
        if seen.contains(&idx) { continue; }
        groups += 1;
        count_links(idx, connections, &mut seen);
    }
    groups
}

type Connections = Vec<Vec<usize>>;