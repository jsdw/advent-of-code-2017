use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    // Get vector of ints from input string:
    let lines: Vec<isize> = content.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    // Run the instructions, deciding how to modify each instruciton based on its curr value:
    println!("Star 1: {}", solve(lines.clone(), |_| 1));
    println!("Star 2: {}", solve(lines.clone(), |j| if j >= 3 { -1 } else { 1 }));

}

fn solve<F: Fn(isize) -> isize>(mut lines: Vec<isize>, modify_given_jump: F) -> isize {
    let mut pos: isize = 0;
    let mut steps: isize = 0;
    while pos >= 0 && pos < lines.len() as isize {
        let idx = pos as usize;
        let jump = lines[idx];
        lines[idx] +=  modify_given_jump(jump);
        pos += jump;
        steps += 1;
    }
    steps
}