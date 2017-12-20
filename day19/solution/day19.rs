use std::env;
use std::fs;
use std::io::Read;
use Piece::*;
use Direction::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let map = Map::new(content.lines().map(parse_line).collect());

    // Star 1:
    let letters: String = Runner::new(&map).filter_map(|p| p.letter()).collect();
    println!("Star 1: {}", letters);

    // Star 2:
    let steps: usize = Runner::new(&map).count();
    println!("Star 2: {}", steps);
}

// ********************************
// * A Runner to navigate the map *
// ********************************

#[derive(Clone,Debug)]
struct Runner<'a> {
    map: &'a Map,
    pos: (isize,isize),
    direction: Direction,
    finished: bool
}

impl <'a> Runner<'a> {
    fn new(map: &Map) -> Runner {
        let x: usize = map.row(0).unwrap().iter().take_while(|&&p| if p == Empty { true } else { false }).count();
        Runner { map, pos: (x as isize, 0), direction: Down, finished: false }
    }
    fn step(&mut self) {
        if self.finished { return }

        for &d in &[self.direction, self.direction.r(), self.direction.r().r().r()] {
            let next_pos = d.next_coords(self.pos);
            if !self.map.get(next_pos).unwrap_or(Empty).is_empty() {
                self.pos = next_pos;
                self.direction = d;
                return
            }
        }

        self.finished = true;
    }
}
impl <'a> Iterator for Runner<'a> {
    type Item = Piece;
    fn next(&mut self) -> Option<Piece> {
        if self.finished {
            None
        } else {
            let curr = self.map.get(self.pos);
            self.step();
            curr
        }
    }
}

// **************
// * Directions *
// **************

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}
impl Direction {
    fn r(&self) -> Direction {
        match *self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right
        }
    }
    fn next_coords(&self, (x,y): (isize,isize)) -> (isize,isize) {
        match *self {
            Right => (x + 1, y),
            Down  => (x, y + 1),
            Left  => (x - 1, y),
            Up    => (x, y - 1)
        }
    }
}

// ***********
// * The Map *
// ***********

#[derive(Clone,Debug)]
struct Map(Vec<Vec<Piece>>);

impl Map {
    fn new(input: Vec<Vec<Piece>>) -> Map {
        Map(input)
    }
    fn row(&self, y: isize) -> Option<&Vec<Piece>> {
        if y < 0 { None } else { self.0.get(y as usize) }
    }
    fn get(&self, (x,y): (isize,isize)) -> Option<Piece> {
        if x < 0 { None } else { self.row(y).and_then(|row| row.get(x as usize)).map(|p| *p) }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Piece {
    Letter(char),
    Road,
    Empty
}

impl Piece {
    fn letter(&self) -> Option<char> {
        if let Letter(c) = *self { Some(c) } else { None }
    }
    fn is_empty(&self) -> bool {
        if let Empty = *self { true } else { false }
    }
}

fn parse_line(line: &str) -> Vec<Piece> {
    line.chars().map(parse_char).collect()
}

fn parse_char(c: char) -> Piece {
    if c == '+' || c == '-' || c == '|' {
        Piece::Road
    } else if char::is_whitespace(c) {
        Piece::Empty
    } else {
        Piece::Letter(c)
    }
}