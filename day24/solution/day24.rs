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

    // build all possible bridges and summarise some stats:
    let bridges = build_bridges(&pieces);
    let stats: Vec<(usize,isize)> = bridges
        .into_iter()
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

fn build_bridges(pieces: &[Piece]) -> Vec<Vec<Piece>> {

    // we start with any bridges with a 0 in, putting the 0 first.
    let mut search_space: Vec<Vec<Piece>> = pieces.iter()
        .filter(|&&(a,b)| a == 0 || b == 0)
        .map(|&(a,b)| vec![if a == 0 { (a,b) } else { (b,a) }])
        .collect();

    // create a table to easily find available bridges given some port:
    let mut pieces_hash = PieceHash::new();
    pieces.iter().for_each(|piece| pieces_hash.insert_piece(piece));

    // searching expands the search space until bridges are built, then putting
    // them into finished instead. keep going until nothing left to search.
    let mut finished = vec![];
    while search_space.len() > 0 {
        search_step(&mut search_space, &mut finished, &pieces_hash);
    }
    finished
}

fn search_step(search_space: &mut Vec<Vec<Piece>>, finished: &mut Vec<Vec<Piece>>, pieces: &PieceHash) {
    let mut current = vec![];
    ::std::mem::swap(search_space, &mut current);

    for bridge in current {

        // The last port number is the one we need to match:
        let &(_,b) = bridge.last().unwrap();

        // find all valid next pieces; remove any for which their
        // available count has already been used:
        let matches: Vec<Piece> = pieces
            .matches_for(b)
            .into_iter()
            .filter(|&(piece,count)|{
                bridge.iter().filter(|&&(a,b)| (a,b) == piece || (b,a) == piece).count() < count
            })
            .map(|(piece,_)| piece)
            .collect();

        // no matches? the bridge is done.
        if matches.len() == 0 {
            finished.push(bridge);
            continue;
        }

        // matches? expand our search space.
        for piece in matches {
            let mut new = bridge.clone();
            new.push(piece);
            search_space.push(new);
        }
    }
}

// parse a string like "18/22" into a tuple of (18,22):
fn parse_piece(s: &str) -> Option<Piece> {
    let mut bits = s.split('/');
    let a = bits.next()?.parse().ok()?;
    let b = bits.next()?.parse().ok()?;
    Some((a,b))    
}

type Piece = (isize,isize);

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
