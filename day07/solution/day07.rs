extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    // turn raw input into something structured, ready to work with:
    let nodes: Vec<Node> = content.lines().filter_map(parse_node).collect();

    let root_node = star1(&nodes);
    println!("Star 1: {}", root_node);
}

fn star1(nodes: &[Node]) -> String {

    let mut child_to_parent: HashMap<&str,&str> = HashMap::new();
    for node in nodes {
        for child in &node.children {
            child_to_parent.insert(child, &node.name);
        }
    }

    let mut curr: &str = &nodes[0].name;
    loop {
        if let Some(next) = child_to_parent.get(curr) {
            curr = next;
        } else {
            break;
        }
    }

    curr.to_owned()
}

// turn a single line of input into a single Node:
fn parse_node(line: &str) -> Option<Node> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) \(([0-9]+)\)(?: -> ([a-z]+(, [a-z]+)*))?$").unwrap();
    }
    let caps = RE.captures(line)?;
    let name: String          = caps[1].to_owned();
    let weight: usize         = caps[2].parse().ok()?;
    let children: Vec<String> = caps.get(3).map(|m| m.as_str().split(", ").map(|s| s.to_owned()).collect()).unwrap_or(vec![]);

    Some(Node { name, weight, children })
}

// a single Node:
#[derive(Debug,Clone,PartialEq,Eq)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<String>
}