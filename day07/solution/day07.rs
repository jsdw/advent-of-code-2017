extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use regex::Regex;
use std::collections::{HashMap,HashSet};

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