use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashSet;

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