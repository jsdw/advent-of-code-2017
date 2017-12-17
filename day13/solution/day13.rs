use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let connections: Vec<Layer> = content.lines().filter_map(parse_line).collect();

    let severity: usize = connections.iter()
        .map(|l| (l, step_position(&l, l.depth)))
        .map(|(l,p)| if p == 0 { l.depth * l.range } else { 0 })
        .sum();
    println!("Star 1: {}", severity);

    let delay: usize = (0..)
        .filter(|&step| connections.iter().map(|l| step_position(&l, l.depth + step)).all(|l| l != 0))
        .next()
        .unwrap();
    println!("Star 2: {}", delay);

}

fn parse_line(line: &str) -> Option<Layer> {
    let mut ns = line.split(": ");
    let depth = ns.next()?.parse().ok()?;
    let range = ns.next()?.parse().ok()?;
    Some(Layer{ depth, range, position: 0 })
}

fn step_position(layer: &Layer, step: usize) -> usize {
    let m = layer.range + if layer.range > 2 { layer.range - 2 } else { 0 };
    let d = (layer.position + step) % m;
    if d >= layer.range { m - d } else { d }
}

#[derive(Debug,Clone,Copy)]
struct Layer {
    depth: usize,
    range: usize,
    position: usize
}