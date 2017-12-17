use std::env;
use std::fs;
use std::io::Read;
use std::iter::Peekable;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let mut chars = content.chars().peekable();
    let group: Item = parse_item(&mut chars);

    println!("Star 1: {}", score_item(&group, 1));
    println!("Star 2: {}", count_garbage(&group));

}

fn score_item(group: &Item, score: usize) -> usize {
    match *group {
        Item::Group(ref children) => {
            score + children.iter().map(|c| score_item(c, score + 1)).sum::<usize>()
        },
        _ => 0
    }
}

fn count_garbage(group: &Item) -> usize {
    match *group {
        Item::Group(ref children) => {
            children.iter().map(|c| count_garbage(c)).sum::<usize>()
        },
        Item::Garbage(ref garbage) => {
            garbage.len()
        }
    }
}

fn parse_item<It: Iterator<Item=char>>(chars: &mut Peekable<It>) -> Item {
    let next = chars.next().unwrap().clone();
    if next == '{' {
        Item::Group(parse_group(chars))
    } else {
        Item::Garbage(parse_garbage(chars))
    }
}

fn parse_group<It: Iterator<Item=char>>(chars: &mut Peekable<It>) -> Stream {
    let mut items = vec![];
    loop {
        let peeked = chars.peek().unwrap().clone();
        if peeked == '}' {
            chars.next().unwrap();
            return items;
        } else if peeked == ',' {
            chars.next().unwrap();
        }
        items.push(parse_item(chars));
    }
}

fn parse_garbage<It: Iterator<Item=char>>(chars: &mut Peekable<It>) -> String {
    let mut garbage = String::new();
    let mut ignore_next = false;
    loop {
        let next = chars.next().unwrap();
        if !ignore_next {
            if next == '>' {
                return garbage;
            }
            if next == '!' {
                ignore_next = true;
            } else {
                garbage.push(next);
            }
        } else {
            ignore_next = false;
        }
    }
}

type Stream = Vec<Item>;

#[derive(Debug)]
enum Item {
    Garbage(String),
    Group(Stream)
}