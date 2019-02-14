use std::env;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");

    // read input to string:
    let lines = std::fs::read_to_string(filename)
        .expect("Can't open file")
        .lines()
        .map(|line| line.split_whitespace().filter_map(|s| s.parse().ok()).collect())
        .collect::<Vec<Vec<u64>>>();

    // Star 1: diff between max and min for each line:
    let sum: u64 = lines
        .iter()
        .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())
        .sum();
    println!("Star 1: {}", sum);

    // Star 2: find perfect dividers and sum result for each line:
    let sum: u64 = lines
        .iter()
        .filter_map(|l| find_even_divides(&l))
        .sum();
    println!("Star 2: {}", sum);

}

fn find_even_divides(input: &[u64]) -> Option<u64> {
    for (n1,n2) in tuple_combinations(input) {
        if n2 % n1 == 0 { return Some(n2 / n1) }
        else if n1 % n2 == 0 { return Some(n1 / n2) }
    }
    None
}

// Gives back a reference to each combination of values in the slice provided.
// We could just use iter-tools to provide a similar thing:
fn tuple_combinations<'a, T: 'a>(input: &'a [T]) -> impl Iterator<Item=(&T,&T)> + 'a {
    input.iter().enumerate().flat_map(move |(idx,n1)| {
        input[idx+1..].iter().map(move |n2| {
            (n1, n2)
        })
    })
}