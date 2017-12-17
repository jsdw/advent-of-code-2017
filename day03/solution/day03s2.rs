use std::env;
use std::collections::HashMap;

fn main() {

    let input: u64 = env::args()
        .nth(1).expect("provide input number as first arg")
        .parse().expect("needs valid number as first arg");


    // Star 2:
    println!("Star 2: {}", get_value(input));

}

// We are going to brute force star 2. We'll use an iterator that
// churns out coordinates by iterating around in a spiral fashion,
// and we'll just set values in a map based on the values that exist
// at surrounding coordinates.
fn get_value(input: u64) -> u64 {

    let positions = coords::Spiral::new();
    let mut spiral: HashMap<coords::Coords,u64> = HashMap::new();

    // seed the spiral with a starting value,
    // and skip overwriting it below.
    spiral.insert(coords::at(0,0), 1);

    for coord in positions.skip(1) {

        let around = coords::surrounding(coord);
        let value = around.iter().filter_map(|coord| spiral.get(coord)).sum();

        if value > input {
            return value;
        }

        spiral.insert(coord, value);
    }

    unreachable!("Iterator should never end")
}

// our spiral coordinate iterator, and function to get
// surrounding coordinates, live in here.
mod coords {

    pub fn surrounding(pos: Coords) -> [Coords; 8] {
        [
            at(pos.x + 1, pos.y + 0),
            at(pos.x + 1, pos.y + 1),
            at(pos.x + 0, pos.y + 1),
            at(pos.x - 1, pos.y + 1),
            at(pos.x - 1, pos.y + 0),
            at(pos.x - 1, pos.y - 1),
            at(pos.x + 0, pos.y - 1),
            at(pos.x + 1, pos.y - 1)
        ]
    }

    pub struct Spiral {
        direction: Direction,
        n: i64,
        val: Coords
    }

    impl Spiral {
        pub fn new() -> Spiral {
            Spiral {
                direction: Direction::Right,
                n: 0,
                val: at(0,0)
            }
        }
        fn inc(&mut self) {
            match self.direction {
                Direction::Up => {
                    self.val.y += 1;
                    if self.val.y == self.n {
                        self.direction = Direction::Left;
                    };
                },
                Direction::Left => {
                    self.val.x -= 1;
                    if self.val.x == -self.n {
                        self.direction = Direction::Down;
                    };
                },
                Direction::Down => {
                    self.val.y -= 1;
                    if self.val.y == -self.n {
                        self.direction = Direction::Right;
                    };
                },
                Direction::Right => {
                    self.val.x += 1;
                    if self.val.x == self.n + 1 {
                        self.direction = Direction::Up;
                        self.n += 1;
                    }
                }
            }
        }
    }

    impl Iterator for Spiral {
        type Item = Coords;
        fn next(&mut self) -> Option<Self::Item> {
            let cur = self.val;
            self.inc();
            Some(cur)
        }
    }

    #[derive(Debug,Copy,Clone,PartialEq)]
    enum Direction {
        Up, Down, Left, Right
    }

    #[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
    pub struct Coords {
        x: i64,
        y: i64
    }

    pub fn at(x: i64, y: i64) -> Coords {
        Coords { x, y }
    }

}
