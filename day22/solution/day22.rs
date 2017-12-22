use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use self::Direction::*;
use self::State::*;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let original_grid = map_grid(&parse_grid(&content));

    // print the middle area of our grid to screen to
    // show what we're working with.
    for y in -20..20 {
        let s: String = (-20..20)
            .map(|x| *original_grid.get(&(x,y)).unwrap_or(&Clean))
            .map(|b| if b == Infected { '#' } else { '.' })
            .collect();
        println!("{}", s);
    }

    // Star 1:
    let mut grid = original_grid.clone();
    let mut dir = Up;
    let mut coords = (0,0);
    let mut infected = 0;
    for _ in 0..10_000 {
        let state = *grid.get(&coords).unwrap_or(&Clean);
        let new_state;
        if state == Infected {
            new_state = Clean;
            dir.turn_right();
        } else {
            new_state = Infected;
            // oddly, the puzzle states that nodes beginning infected should
            // be ignored, but if we do ignore those (looking at original_grid
            // and only incrementing count if node didn't start infected), we
            // get the wrong answer. Same for part 2.
            infected += 1;
            dir.turn_left();
        }
        grid.insert(coords, new_state);
        coords = dir.step_coords(coords);
    }
    println!("Star 1: {}", infected);

    // Star 2:
    let mut grid = original_grid.clone();
    let mut dir = Up;
    let mut coords = (0,0);
    let mut infected = 0;
    for _ in 0..10_000_000 {
        let state = *grid.get(&coords).unwrap_or(&Clean);
        match state {
            Clean => dir.turn_left(),
            Infected => dir.turn_right(),
            Flagged => dir.reverse(),
            Weakened => {}
        };
        let new_state = state.rotate();
        if new_state == Infected { infected += 1 }
        grid.insert(coords, new_state);
        coords = dir.step_coords(coords);
    }
    println!("Star 2: {}", infected);

}

fn parse_grid(grid: &str) -> Vec<Vec<bool>> {
    grid.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect()
}

fn map_grid(grid: &Vec<Vec<bool>>) -> HashMap<Coord,State> {
    let mut map = HashMap::new();
    let offset_y = grid.len() as isize / 2;
    for (y,row) in grid.iter().enumerate() {
        let offset_x = row.len() as isize / 2;
        for (x,&b) in row.iter().enumerate() {
            map.insert((x as isize - offset_x,y as isize - offset_y), if b { Infected } else { Clean });
        }
    }
    map
}

type Coord = (isize,isize);

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
enum State {
    Clean, Weakened, Infected, Flagged
}
impl State {
    fn rotate(&self) -> State {
        match *self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean
        }
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
enum Direction {
    Up, Right, Down, Left
}
impl Direction {
    fn turn_right(&mut self) {
        *self = match *self {
            Up    => Right,
            Right => Down,
            Down  => Left,
            Left  => Up
        }
    }
    fn turn_left(&mut self) {
        self.turn_right();
        self.turn_right();
        self.turn_right();
    }
    fn reverse(&mut self) {
        self.turn_right();
        self.turn_right();
    }
    fn step_coords(&self, (x,y): Coord) -> Coord {
        match *self {
            Up    => (x, y-1),
            Right => (x+1, y),
            Down  => (x, y+1),
            Left  => (x-1, y)
        }
    }
}