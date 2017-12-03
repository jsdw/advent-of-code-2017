use std::env;

// --- Day 3: Spiral Memory ---
//
// You come across an experimental new kind of memory stored on an infinite
// two-dimensional grid.
//
// Each square on the grid is allocated in a spiral pattern starting at a location
// marked 1 and then counting up while spiraling outward. For example, the first
// few squares are allocated like this:
//
// 17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ...
//
// ... While this is very space-efficient (no squares are skipped),
// requested data must be carried back to square 1 (the location of the only access
// port for this memory system) by programs that can only move up, down, left, or
// right. They always take the shortest path: the Manhattan Distance between the
// location of the data and square 1.
//
// For example:
//
// Data from square 1 is carried 0 steps, since it's at the access port. Data from
// square 12 is carried 3 steps, such as: down, left, left. Data from square 23 is
// carried only 2 steps: up twice. Data from square 1024 must be carried 31 steps.
// How many steps are required to carry the data from the square identified in your
// puzzle input all the way to the access port?
//
// The first half of this puzzle is complete! It provides one gold star: *
//
fn main() {

    let input: i64 = env::args()
        .nth(1).expect("provide input number as first arg")
        .parse().expect("needs valid number as first arg");

    // Star 1:
    println!("Star 1: {}", distance_to_digit(input));

}

// To find the first digit S of a square that is distance N
// from the center, we can use:
//
// S = (2(N-1) + 1)^2
//
// We can flip this around quite easily to find N given some digit
// S that lives in the square N distance away:
//
// floor( (sqrt(S-1) - 1) / 2 + 1 ) = N
//
// Casting back to i64 after sqrt takes care of flooring for us
// as needed. Special case for 1
fn distance_to_square_containing_digit(s: i64) -> i64 {
    if s == 1 { return 0 };
    (((s - 1) as f64).sqrt() as i64 - 1) / 2 + 1
}

// To find the distance from some S to the center, we find which
// square N it lives in, subtract the starting number of that square
// so that each square starts at 0 eg
//
// 3 2 1
// 4   0
// 5 6 7
//
// And then, given this new number S', abs( (S' % (H - 1)) - (N - 1) )
// where H is the height of the square (2N+1) tells us the extra steps
// required to get to our S'. Add this to N to find manhatten distance
// from center.
fn distance_to_digit(s: i64) -> i64 {
    if s == 1 { return 0 };

    let n = distance_to_square_containing_digit(s);
    let s = s - starting_number_for_square(n);
    let height = 2 * n + 1;

    n + ((s % (height - 1)) - (n - 1) ).abs()
}

// given some square distance N from center, get the starting digit for it.
// (2N+1)^2 gives us the last S in the square, so tweak that to get first.
fn starting_number_for_square(n: i64) -> i64 {
    (2 * (n - 1) + 1).pow(2) + 1
}