extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use std::cell::Cell;

// --- Day 7: Recursive Circus ---
//
// Wandering further through the circuits of the computer, you come upon a tower of
// programs that have gotten themselves into a bit of trouble. A recursive
// algorithm has gotten out of hand, and now they're balanced precariously in a
// large tower.
//
// One program at the bottom supports the entire tower. It's holding a large disc,
// and on the disc are balanced several more sub-towers. At the bottom of these
// sub-towers, standing on the bottom disc, are other programs, each holding their
// own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many
// programs stand simply keeping the disc below them balanced but with no disc of
// their own.
//
// You offer to help, but first you need to understand the structure of these
// towers. You ask each program to yell out their name, their weight, and (if
// they're holding a disc) the names of the programs immediately above them
// balancing on that disc. You write this information down (your puzzle input).
// Unfortunately, in their panic, they don't do this in an orderly fashion; by the
// time you're done, you're not sure which program gave which information.
//
// For example, if your list is the following:
//
// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)
//
// ...then you would be able to recreate the structure of the towers that looks
// like this:
//
//                 gyxo
//               /
//          ugml - ebii
//        /      \
//       |         jptl
//       |
//       |         pbga
//      /        /
// tknk --- padx - havc
//      \        \
//       |         qoyq
//       |
//       |         ktlj
//        \      /
//          fwft - cntj
//               \
//                 xhth
//
// In this example, tknk is at the bottom of the tower (the bottom program), and is
// holding up ugml, padx, and fwft. Those programs are, in turn, holding up other
// programs; in this example, none of those programs are holding up any other
// programs, and are all the tops of their own towers. (The actual tower balancing
// in front of you is much larger.)
//
// Before you're ready to help them, you need to make sure your information is
// correct. What is the name of the bottom program?
//
// Your puzzle answer was svugo.
//
// --- Part Two ---
//
// The programs explain the situation: they can't get down. Rather, they could get
// down, if they weren't expending all of their energy trying to keep the tower
// balanced. Apparently, one program has the wrong weight, and until it's fixed,
// they're stuck here.
//
// For any program holding a disc, each program standing on that disc forms a
// sub-tower. Each of those sub-towers are supposed to be the same weight, or the
// disc itself isn't balanced. The weight of a tower is the sum of the weights of
// the programs in that tower.
//
// In the example above, this means that for ugml's disc to be balanced, gyxo,
// ebii, and jptl must all have the same weight, and they do: 61.
//
// However, for tknk to be balanced, each of the programs standing on its disc and
// all programs above it must each match. This means that the following sums must
// all be the same:
//
// ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
// padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
// fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
//
// As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the
// other two. Even though the nodes above ugml are balanced, ugml itself is too
// heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the
// towers balanced. If this change were made, its weight would be 60.
//
// Given that exactly one program is the wrong weight, what would its weight need
// to be to balance the entire tower?

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

    let desired_weight = star2(&nodes, &root_node);
    println!("Star 2: {}", desired_weight.expect("All weights look ok"));

}

// create map from child name to parent name. traverse the map from some arbitrary
// name to find the root parent of the tree.
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

fn star2(nodes: &[Node], root_node: &str) -> Option<isize> {

    let mut by_name: HashMap<&str,&Node> = HashMap::new();
    for node in nodes {
        by_name.insert(&node.name, node);
    }

    // work out the total weights and balanced states for each node:
    sum_weights(root_node, &by_name);

    // follow the !balanced states down the tree from the root until we get to
    // a node whose children are balanced. This node is itself unbalanced.
    let mut curr = by_name[root_node];
    while let Some(name) = curr.children.iter().find(|&c| !by_name[&**c].balanced.get()) {
        curr = by_name[&**name];
    }

    // find the differing weights of the children of this unbalanced node,
    // and return the desired weight of the bad node.
    let children: Vec<&Node> = curr.children.iter().map(|c| by_name[&**c]).collect();
    find_balancing_weight(&children)
}

// recursively sum up the node weights and balanced statuses.
// - a nodes total_weight is the weight of it and every node below it.
// - a node is balanced if its children are all balanced *and* they all
//   have the same weights.
fn sum_weights<'a>(name: &str, by_name: &HashMap<&str,&'a Node>) -> &'a Node {
    let node = &by_name[name];
    let children: Vec<&Node> = node.children.iter().map(|c| sum_weights(&c, by_name)).collect();

    let mut child_weights: isize = 0;
    let mut balanced: bool = true;
    for child in &children {
        child_weights += child.total_weight.get();
        if balanced && !child.balanced.get() { balanced = false }
        else if balanced && find_balancing_weight(&children).is_some() { balanced = false };
    }

    node.total_weight.set(child_weights + node.weight);
    node.balanced.set(balanced);
    node
}

// given a list of nodes, find any total weight which differs from the rest
// and return the weight of the node that would compensate for this difference.
// if nodes are all the same weight, returns None, as no change is needed.
fn find_balancing_weight(vals: &[&Node]) -> Option<isize> {

    let mut it = vals.iter();

    let a = it.next()?;
    let b = it.next()?;

    let a_total = a.total_weight.get();
    let b_total = b.total_weight.get();

    if a_total == b_total {
        for v in it {
            let v_total = v.total_weight.get();
            if v_total != a_total {
                return Some(a_total - (v_total - v.weight));
            }
        }
        return None;
    } else {
        if let Some(c) = it.next() {
            let c_total = c.total_weight.get();
            if a_total == c_total {
                return Some(a_total - (b_total - b.weight));
            }
        }
        return Some(b_total - (a_total - a.weight));
    }

}

// turn a single line of input into a single Node:
fn parse_node(line: &str) -> Option<Node> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) \(([0-9]+)\)(?: -> ([a-z]+(, [a-z]+)*))?$").unwrap();
    }
    let caps = RE.captures(line)?;
    let name: String          = caps[1].to_owned();
    let weight: isize         = caps[2].parse().ok()?;
    let children: Vec<String> = caps.get(3).map(|m| m.as_str().split(", ").map(|s| s.to_owned()).collect()).unwrap_or(vec![]);

    Some(Node { name, weight, children, total_weight: Cell::new(0), balanced: Cell::new(true) })
}

// a single Node:
#[derive(Debug,Clone,PartialEq,Eq)]
struct Node {
    name: String,
    weight: isize,
    children: Vec<String>,
    total_weight: Cell<isize>,
    balanced: Cell<bool>
}