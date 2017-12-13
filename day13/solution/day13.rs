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

    let stepped: Vec<Layer> = connections.iter().map(|l| step_position(&l, l.depth)).collect();
    let severity: usize = stepped.iter().map(|l| if l.position == 0 { l.depth * l.range } else { 0 }).sum();
    println!("Star 1: {}", severity);

    let delay: usize = (0..)
        .filter(|&step| stepped.iter().map(|l| step_position(&l,step)).all(|l| l.position == 0))
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

fn step_position(layer: &Layer, step: usize) -> Layer {
    let m = layer.range + if layer.range > 2 { layer.range - 2 } else { 0 };
    let d = (layer.position + step) % m;
    let pos = if d >= layer.range { m - d } else { d };

    Layer {
        depth: layer.depth,
        range: layer.range,
        position: pos
    }
}

#[derive(Debug,Clone,Copy)]
struct Layer {
    depth: usize,
    range: usize,
    position: usize
}