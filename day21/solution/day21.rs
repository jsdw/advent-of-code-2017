use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use self::square::{Square};

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let rules: HashMap<Grid,Grid> = content.lines().map(parse_rule).flat_map(expand_rule).collect();
    let enhance = |sub| if let Some(s) = rules.get(&sub) { s.clone() } else { sub };

    // Our starting square:
    let mut grid = Square::new(vec![
        false, true,  false,
        false, false, true,
        true,  true,  true
    ]);

    // Star 1: 5 iterations of our grid, mapping subsquares based on our rules:
    for _ in 0..5 {
        if grid.size() % 2 == 0 {
            grid = grid.map_subsquares(2, &enhance);
        } else if grid.size() % 3 == 0 {
            grid = grid.map_subsquares(3, &enhance);
        }
    }
    println!("Star 1: {}", grid.data().iter().filter(|&&b| b).count());

    // Star 2: go up to 18 iterations:
    for _ in 5..18 {
        if grid.size() % 2 == 0 {
            grid = grid.map_subsquares(2, &enhance);
        } else if grid.size() % 3 == 0 {
            grid = grid.map_subsquares(3, &enhance);
        }
    }
    println!("Star 2: {}", grid.data().iter().filter(|&&b| b).count());

}

// Give back all of the permutations of rotating and flipping
// some grid (boolean square):
fn expand_rule((input,output): (Grid,Grid)) -> Vec<(Grid,Grid)> {
    let input_90  = input.rot();
    let input_180 = input_90.rot();
    let input_270 = input_180.rot();

    let input_f     = input.flip_h();
    let input_f_90  = input_f.rot();
    let input_f_180 = input_f_90.rot();
    let input_f_270 = input_f_180.rot();

    vec![ input,   input_90,   input_180,   input_270
        , input_f, input_f_90, input_f_180, input_f_270 ]
        .into_iter()
        .map(|i| (i,output.clone()))
        .collect()
}

// turn our input text into boolean square mappings from input to output.
// panics if the input is not as expected.
fn parse_rule(s: &str) -> (Grid,Grid) {
    let mut bits = s.split(" => ");
    let input: Grid = parse_square(bits.next().unwrap());
    let output: Grid = parse_square(bits.next().unwrap());
    (input,output)
}
fn parse_square(s: &str) -> Grid {
    Square::new(s.chars().filter(|&c| c == '.' || c == '#').map(|c| c == '#').collect())
}

type Grid = Square<bool>;

// A bunch of generic square related bits live here, including functions to work with
// subsquares, rotate and flip them. With this stuff in place, it's pretty easy to
// make ourselves some boolean squares and manipulate them as necessary.
mod square {

    use std::ops::{Index,IndexMut};

    // A square of data. This exposes useful methods to work with
    // squares and map them into other squares.
    #[derive(Debug,Clone,Eq,PartialEq,Hash)]
    pub struct Square<T>{
        data: Vec<T>,
        size: usize
    }

    impl <T> Square<T> {
        pub fn new(data: Vec<T>) -> Square<T> {
            let size = (data.len() as f64).sqrt().round() as usize;
            if size.pow(2) != data.len() {
                panic!("Square input needs to be have a square length, but has length of {} {}", size, data.len());
            }
            Square{ data, size }
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn data(&self) -> &Vec<T> {
            &self.data
        }
    }

    impl <T: Copy> Square<T> {
        // Flip a square horizontally:
        pub fn flip_h(&self) -> Square<T> {
            let mut flipped = self.clone();
            for y in 0..self.size {
                for x in 0..self.size/2 {
                    flipped[(self.size-x-1,y)] = self[(x,y)];
                    flipped[(x,y)] = self[(self.size-x-1,y)];
                }
            }
            flipped
        }
        // Rotate a square right 90 degrees:
        pub fn rot(&self) -> Square<T> {
            let mut rotated = self.clone();
            for y in 0..self.size {
                for x in 0..self.size {
                    let new_x = self.size - y - 1;
                    let new_y = x;
                    rotated[(new_x,new_y)] = self[(x,y)];
                }
            }
            rotated
        }
    }

    impl <T: Copy + Default + ::std::fmt::Debug> Square<T> {
        pub fn map_subsquares<F: Fn(Square<T>) -> Square<T>>(&self, size: usize, map: F) -> Square<T> {
            let steps = self.size / size;

            // cut out each of the subsquares, cols then rows, from the main square, avoiding
            // any that may lead to overflow
            let mut subsquares = vec![];
            for (x1,y1) in Coords::new(steps).jump(size) {
                let coords = Coords::new(size).offset(x1, y1);
                let s = Square::new(coords.map(|c| self[c]).collect());
                subsquares.push(map(s));
            }

            if subsquares.len() == 0 {
                return Square::new(vec![]);
            }

            // ensure all subsquares are the same size. panic if this does not hold.
            let new_size = subsquares[0].size;
            if subsquares[1..].iter().any(|s| s.size != new_size) {
                panic!("map_subsquares expects all map result squares to be the same size");
            }

            // piece them back together into one square:
            let mut out = Square::new(vec![T::default(); new_size * new_size * subsquares.len()]);
            for ((x1,y1),sub) in Coords::new(steps).jump(new_size).zip(subsquares) {
                let coords = Coords::new(new_size).offset(x1,y1);
                for (coords,val) in coords.zip(sub.data) {
                    out[coords] = val;
                }
            }
            out
        }
    }

    // convenience helpers to index into squares using (x,y) coords:
    impl <T> Index<(usize,usize)> for Square<T> {
        type Output = T;
        fn index(&self, (x,y): (usize,usize)) -> &T {
            &self.data[ y * self.size + x ]
        }
    }
    impl <T> IndexMut<(usize,usize)> for Square<T> {
        fn index_mut(&mut self, (x,y): (usize,usize)) -> &mut T {
            &mut self.data[ y * self.size + x ]
        }
    }

    // Coords is an iterator over a set of coordinates in some square.
    // we can set the initial coord offset, and the jump amount. This
    // makes it useful for everything to do with subsquares.
    #[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
    struct Coords {
        x: usize,
        y: usize,
        size: usize,
        offset_x: usize,
        offset_y: usize,
        jump: usize
    }

    impl Coords {
        fn new(size: usize) -> Coords {
            Coords { x:0, y:0, jump:1, offset_x:0, offset_y:0, size }
        }
        fn offset(mut self, offset_x: usize, offset_y: usize) -> Coords {
            self.offset_x = offset_x;
            self.offset_y = offset_y;
            self
        }
        fn jump(mut self, j: usize) -> Coords {
            self.jump = j;
            self
        }
    }

    impl Iterator for Coords {
        type Item = (usize,usize);
        fn next(&mut self) -> Option<Self::Item> {
            let (x,y) = (self.x, self.y);

            if self.x < self.size - 1 {
                self.x += 1;
            } else if self.y < self.size {
                self.y += 1;
                self.x = 0;
            }

            if y < self.size {
                Some( (x*self.jump+self.offset_x, y*self.jump+self.offset_y) )
            } else {
                None
            }
        }
    }

}