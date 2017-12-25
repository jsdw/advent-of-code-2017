use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let pieces: Vec<Piece> = content.lines().filter_map(parse_piece).collect();

    // iterate over possible bridges as they are found and summarise:
    let stats: Vec<(usize,isize)> = DFS::new(&pieces)
        .map(|bridge| (bridge.len(), bridge.iter().map(|&(a,b)| a+b).sum()))
        .collect();

    // Which bridge is the strongest?
    let strongest: isize = stats.iter().map(|&(_,strength)| strength).max().unwrap();
    println!("Star 1: {}", strongest);

    // What is the highest strength of the longest bridge?
    let mut stats = stats;
    stats.sort_unstable_by(|&(a_l,a_s),&(b_l,b_s)| a_l.cmp(&b_l).then(a_s.cmp(&b_s)).reverse());
    println!("Star 1: {}", stats[0].1);

}

// Our depth-first searcher allows us to stream 
// search results as they become available. Iteration
// pulls from the finished list, and if it becomes empty
// runs search_steps until it's populated again, until our
// search_space is exhausted.
struct DFS {
    search_space: Vec<Vec<Piece>>,
    finished: Vec<Vec<Piece>>,
    pieces: PieceHash
}
impl DFS {
    fn new(pieces: &Vec<Piece>) -> DFS {
        // we start with any bridges with a 0 in, putting the 0 first.
        let search_space: Vec<Vec<Piece>> = pieces.iter()
            .filter(|&&(a,b)| a == 0 || b == 0)
            .map(|&(a,b)| vec![if a == 0 { (a,b) } else { (b,a) }])
            .collect();

        // create a table to easily find available bridges given some port:
        let mut pieces_hash = PieceHash::new();
        pieces.iter().for_each(|piece| pieces_hash.insert_piece(piece));

        DFS {
            pieces: pieces_hash,
            finished: vec![],
            search_space
        }
    }
    fn search_step(&mut self) {
        let bridge = if let Some(b) = self.search_space.pop() { b } else { return };

        // The last port number is the one we need to match:
        let &(_,b) = bridge.last().unwrap();

        // find all valid next pieces; remove any for which their
        // available count has already been used:
        let matches: Vec<Piece> = self.pieces
            .matches_for(b).into_iter()
            .filter(|&(p,c)| bridge.iter().filter(|&&(a,b)| (a,b) == p || (b,a) == p).count() < c)
            .map(|(piece,_)| piece)
            .collect();

        // bridge finished if no matches, else push new bridges
        // into our search space.
        if matches.len() == 0 {
            self.finished.push(bridge);
        } else {
            for piece in matches {
                let mut new = bridge.clone();
                new.push(piece);
                self.search_space.push(new);
            }
        }
    }
}
impl Iterator for DFS {
    type Item = Vec<Piece>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.search_space.len() > 0 && self.finished.len() == 0 {
            self.search_step();
        }
        self.finished.pop()
    }
}

// A basic struct for storing knowledge about a tupple of two ports which
// can be used either way around. We use it to find pieces that match some
// port (on either side)
struct PieceHash {
    map: HashMap<isize,HashMap<isize,usize>>
}
impl PieceHash {
    fn new() -> PieceHash {
        PieceHash{ map: HashMap::new() }
    }
    fn insert_piece(&mut self, &(a,b): &Piece) {
        *self.map.entry(a).or_insert_with(|| HashMap::new()).entry(b).or_insert(0) += 1;
        if a != b {
            *self.map.entry(b).or_insert_with(|| HashMap::new()).entry(a).or_insert(0) += 1;
        }
    }
    fn matches_for(&self, a: isize) -> Vec<(Piece,usize)> {
        self.map.get(&a)
            .map(|inner| inner.iter().map(|(&b,&count)| ((a,b),count)).collect())
            .unwrap_or(vec![])
    }
}

// Parse our pieces: parse a string like 
// "18/22" into a tuple of (18,22):
fn parse_piece(s: &str) -> Option<Piece> {
    let mut bits = s.split('/');
    let a = bits.next()?.parse().ok()?;
    let b = bits.next()?.parse().ok()?;
    Some((a,b))    
}
type Piece = (isize,isize);