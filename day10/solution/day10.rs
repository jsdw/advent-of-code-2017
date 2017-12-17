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

    let input1: Vec<usize> = content.split(',').filter_map(|n| n.parse().ok()).collect();
    println!("Star 1: {}", star1(&input1));

    let input2: Vec<usize> = content.into_bytes().into_iter().map(|b| b as usize).chain(vec![17,31,73,47,23]).collect();
    println!("Star 2: {}", star2(&input2));
}

fn star1(lengths: &Vec<usize>) -> usize {
    let mut knot: Vec<usize> = (0..256).collect();
    let mut curr_idx = 0;
    let mut skip_size = 0;

    for &len in lengths {
        knot_round(curr_idx, len, &mut knot);
        curr_idx += len + skip_size;
        skip_size += 1;
    }

    knot[0] * knot[1]
}

fn star2(lengths: &Vec<usize>) -> String {
    let mut knot: Vec<usize> = (0..256).collect();
    let mut curr_idx = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for &len in lengths {
            knot_round(curr_idx, len, &mut knot);
            curr_idx += len + skip_size;
            skip_size += 1;
        }
    }

    knot.chunks(16)
        .map(|c| c.iter().fold(0u8,|acc,&n| n as u8 ^ acc))
        .fold(String::new(), |mut s,n| { s.push_str(&format!("{:02x}", n)); s })

}

fn knot_round(start: usize, len: usize, knot: &mut Vec<usize>) {
    let knot_len     = knot.len();
    let to_alter     = (start..start+len).map(|i| i % knot_len);
    let to_alter_rev = (start..start+len).map(|i| i % knot_len).rev();

    for (a,b) in to_alter.zip(to_alter_rev).take(len / 2) {
        knot.swap(a,b);
    }
}