use std::env;
use self::drive::Drive;

fn main() {

    let input = env::args().nth(1).expect("need puzzle input");
    let drive = Drive::new(&input);

    let total: usize = drive.iter().map(|&b| if b { 1 } else { 0 }).sum();
    println!("Star 1: {}", total);

    let regions = count_regions(drive);
    println!("Star 2: {}", regions);

}

fn count_regions(mut drive: Drive) -> usize {
    let mut sum = 0;
    for i in 0..drive.size() {
        for j in 0..drive.size() {
            if drive[(i,j)] {
                remove_region(&mut drive, (i,j));
                sum += 1
            }
        }
    }
    sum
}

fn remove_region(drive: &mut Drive, idx: (usize,usize)) {
    if !drive[idx] { return }
    drive[idx] = false;
    if idx.0 > 0 { remove_region(drive, (idx.0 - 1, idx.1)) };
    if idx.0 < drive.size() - 1 { remove_region(drive, (idx.0 + 1, idx.1)) };
    if idx.1 > 0 { remove_region(drive, (idx.0, idx.1 - 1)) };
    if idx.1 < drive.size() - 1 { remove_region(drive, (idx.0, idx.1 + 1)) };
}

// This is a bit overkill, but how we might encapsulate the behaviour of a drive
// behind some basic interfaces:
mod drive {

    use std::ops::{Index,IndexMut,Deref};

    const SIZE: usize = 128;

    pub struct Drive(Vec<bool>);

    impl Drive {
        pub fn new(input: &str) -> Drive {
            Drive( (0..128).flat_map(|i| knot_hash(&format!("{}-{}", input, i))).collect() )
        }
        pub fn size(&self) -> usize {
            SIZE
        }
    }
    impl Deref for Drive {
        type Target = [bool];
        fn deref(&self) -> &[bool] {
            &*self.0
        }
    }
    impl Index<(usize,usize)> for Drive {
        type Output = bool;
        fn index(&self, (row,col): (usize,usize)) -> &bool {
            &self.0[ row * SIZE + col ]
        }
    }
    impl IndexMut<(usize,usize)> for Drive {
        fn index_mut(&mut self, (row,col): (usize,usize)) -> &mut bool {
            &mut self.0[ row * SIZE + col ]
        }
    }

    fn knot_hash(input: &str) -> Vec<bool> {
        let input: Vec<u8> = input.bytes().chain(vec![17,31,73,47,23]).collect();
        let mut knot: Vec<usize> = (0..256).collect();
        let mut curr_idx = 0;
        let mut skip_size = 0;

        for _ in 0..64 {
            for &len in &input {
                knot_round(curr_idx, len as usize, &mut knot);
                curr_idx += len as usize + skip_size;
                skip_size += 1;
            }
        }

        knot.chunks(16)
            .map(|c| c.iter().fold(0u8,|acc,&n| n as u8 ^ acc))
            .flat_map(|n| (0..8).map(move |i| (n >> (7-i)) & 1 == 1))
            .collect()
    }

    fn knot_round(start: usize, len: usize, knot: &mut Vec<usize>) {
        let knot_len     = knot.len();
        let to_alter     = (start..start+len).map(|i| i % knot_len);
        let to_alter_rev = (start..start+len).map(|i| i % knot_len).rev();

        for (a,b) in to_alter.zip(to_alter_rev).take(len / 2) {
            knot.swap(a,b);
        }
    }

}