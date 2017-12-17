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

    let starts: Vec<usize> = content.lines().map(|l| {
        let n_str: String = l.matches(char::is_numeric).collect();
        n_str.parse().unwrap()
    }).collect();

    let a_start = *starts.get(0).expect("need gen A value");
    let b_start = *starts.get(1).expect("need gen B value");

    let a = Generator::new(a_start, 16807);
    let b = Generator::new(b_start, 48271);
    let star1 = a.zip(b).take(40_000_000).filter(|&(av,bv)| av & 0xFFFF == bv & 0xFFFF).count();
    println!("Star 1: {}", star1);

    let a = Generator::new(a_start, 16807).filter(|&val| val % 4 == 0);
    let b = Generator::new(b_start, 48271).filter(|&val| val % 8 == 0);
    let star2 = a.zip(b).take(5_000_000).filter(|&(av,bv)| av & 0xFFFF == bv & 0xFFFF).count();
    println!("Star 2: {}", star2);
}

struct Generator {
    val: usize,
    multiplier: usize
}

impl Generator {
    fn new(start: usize, multiplier: usize) -> Generator {
        Generator {
            val: start,
            multiplier: multiplier
        }
    }
}

impl Iterator for Generator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let val = self.val;
        self.val = (val * self.multiplier) % 2147483647;
        Some(val)
    }
}
