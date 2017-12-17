use std::env;

fn main() {

    let step: usize = env::args().nth(1).expect("need puzzle input").parse().expect("expects number as input");

    let mut ring = vec![0];
    let mut position: usize = 0;
    for n in 1..2017+1 {
        position = (position + step) % ring.len() + 1;
        ring.insert(position, n);
    }
    println!("Star 1: {}", ring[(position + 1) % ring.len()]);

    let mut after_0 = 0;
    let mut position: usize = 0;
    for n in 1..50_000_000+1 {
        position = (position + step) % n + 1;
        if position == 1 {
            after_0 = n;
        }
    }
    println!("Star 2: {}", after_0);

}

