extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::{HashMap,HashSet};

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let nodes: HashMap<String,RawNode> = content
        .lines()
        .filter_map(parse_raw_node)
        .map(|node| (node.name.clone(), node))
        .collect();

    // Star 1: find the root node of the tree:
    let root_node: String = find_root(&nodes);
    println!("Star 1: {}", root_node);

    // Star 2: build the tree, tracking total weight and balanced state of
    // each node, then traverse down to the last unabalanced node we find
    // and find the balanced weight for it:
    let tree = build_tree(&nodes, &root_node);
    let mut curr = &tree;
    while let Some(child) = curr.children.iter().find(|&c| !c.balanced) { curr = child }
    let correct_weight = find_balanced_weight(&curr.children).unwrap();
    println!("Star 2: {}", correct_weight);
}

// turn a hash of RawNodes into a TreeNode, for easy traversal.
fn build_tree(nodes: &HashMap<String,RawNode>, root_name: &str) -> TreeNode {
    let root = &nodes[root_name];
    let children: Vec<TreeNode> = root.children.iter().map(|c| build_tree(nodes, &**c)).collect();

    TreeNode {
        name: root.name.clone(),
        weight: root.weight,
        total_weight: children.iter().map(|c| c.total_weight).sum::<usize>() + root.weight,
        balanced: children.iter().map(|c| c.balanced).all(|b| b) && find_balanced_weight(&children).is_none(),
        children: children
    }
}

// find the root node by finding the node that is not in any children list:
fn find_root(nodes: &HashMap<String,RawNode>) -> String {
    let children: HashSet<&str> = nodes.values().flat_map(|n| n.children.iter()).map(|c| &**c).collect();
    nodes.keys().find(|&name| !children.contains(&**name)).unwrap().to_owned()
}

// given a list of nodes, find any total weight which differs from the rest
// and return the weight of the node that would compensate for this difference.
// if nodes are all the same weight, returns None, as no change is needed.
fn find_balanced_weight(vals: &[TreeNode]) -> Option<usize> {
    let mut it = vals.iter();
    let a = it.next()?;
    let b = it.next()?;

    let a_total = a.total_weight;
    let b_total = b.total_weight;

    if a_total == b_total {
        for v in it {
            let v_total = v.total_weight;
            if v_total != a_total {
                return Some(a_total - (v_total - v.weight));
            }
        }
        return None;
    }
    if let Some(c) = it.next() {
        if a_total == c.total_weight {
            return Some(a_total - (b_total - b.weight));
        }
    }
    return Some(b_total - (a_total - a.weight));
}

// turn a single line of input into a single RawNode:
fn parse_raw_node(line: &str) -> Option<RawNode> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) \(([0-9]+)\)(?: -> ([a-z]+(, [a-z]+)*))?$").unwrap();
    }
    let caps = RE.captures(line)?;
    let name: String          = caps[1].to_owned();
    let weight: usize         = caps[2].parse().ok()?;
    let children: Vec<String> = caps.get(3).map(|m| m.as_str().split(", ").map(|s| s.to_owned()).collect()).unwrap_or(vec![]);

    Some(RawNode { name, weight, children })
}

// a node in tree form, owning its children for each traversal
// and keeping track of its total weight and balanced state:
#[derive(Debug,Clone,PartialEq,Eq)]
struct TreeNode {
    name: String,
    weight: usize,
    children: Vec<TreeNode>,
    total_weight: usize,
    balanced: bool
}

// a single Node:
#[derive(Debug,Clone,PartialEq,Eq)]
struct RawNode {
    name: String,
    weight: usize,
    children: Vec<String>
}