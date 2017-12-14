use std::env;
use self::drive::Drive;

// --- Day 14: Disk Defragmentation ---
//
// Suddenly, a scheduled job activates the system's disk defragmenter. Were the
// situation different, you might sit and watch it for a while, but today, you just
// don't have that kind of time. It's soaking up valuable system resources that are
// needed elsewhere, and so the only option is to help it finish its task as soon
// as possible.
//
// The disk in question consists of a 128x128 grid; each square of the grid is
// either free or used. On this disk, the state of the grid is tracked by the bits
// in a sequence of knot hashes.
//
// A total of 128 knot hashes are calculated, each corresponding to a single row in
// the grid; each hash contains 128 bits which correspond to individual grid
// squares. Each bit of a hash indicates whether that square is free (0) or used
// (1).
//
// The hash inputs are a key string (your puzzle input), a dash, and a number from
// 0 to 127 corresponding to the row. For example, if your key string were
// flqrgnkx, then the first row would be given by the bits of the knot hash of
// flqrgnkx-0, the second row from the bits of the knot hash of flqrgnkx-1, and so
// on until the last row, flqrgnkx-127.
//
// The output of a knot hash is traditionally represented by 32 hexadecimal digits;
// each of these digits correspond to 4 bits, for a total of 4 * 32 = 128 bits. To
// convert to bits, turn each hexadecimal digit to its equivalent binary value,
// high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes 1110, f becomes 1111,
// and so on; a hash that begins with a0c2017... in hexadecimal would begin with
// 10100000110000100000000101110000... in binary.
//
// Continuing this process, the first 8 rows and columns for key flqrgnkx appear as
// follows, using # to denote used squares, and . to denote free ones:
//
// ##.#.#..-->
// .#.#.#.#
// ....#.#.
// #.#.##.#
// .##.#...
// ##..#..#
// .#...#..
// ##.#.##.-->
// |      |
// V      V
//
// In this example, 8108 squares are used across the entire 128x128 grid.
//
// Given your actual key string, how many squares are used?
//
// --- Part Two ---
//
// Now, all the defragmenter needs to know is the number of regions. A region is a
// group of used squares that are all adjacent, not including diagonals. Every used
// square is in exactly one region: lone used squares form their own isolated
// regions, while several adjacent squares all count as a single region.
//
// In the example above, the following nine regions are visible, each marked with a
// distinct digit:
//
// 11.2.3..-->
// .1.2.3.4
// ....5.6.
// 7.8.55.9
// .88.5...
// 88..5..8
// .8...8..
// 88.8.88.-->
// |      |
// V      V
//
// Of particular interest is the region marked 8; while it does not appear
// contiguous in this small view, all of the squares marked 8 are connected when
// considering the whole 128x128 grid. In total, in this example, 1242 regions are
// present.
//
// How many regions are present given your key string?
//
// Your puzzle input is vbqugkhl.

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