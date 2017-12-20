extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

fn main() {

    let filename = env::args().nth(1).expect("need puzzle input");
    let mut content = String::new();
    fs::File::open(filename)
        .map_err(|e| format!("{}", e))
        .expect("can't open file")
        .read_to_string(&mut content)
        .expect("can't read to string");

    let particles: Vec<Particle> = content.lines().map(Particle::new_from_str).collect();

    // Star 1: which particle will stay closest to 0 is basically which has the smallest acceleration.
    let slowest = particles.iter().enumerate().min_by_key(|&(_,p)| p.acceleration.manhatten()).unwrap().0;
    println!("Star 1: {}", slowest);

    // Star 2: run the simulation for some number of iterations, checking for collisions each step,
    // until we're happy that nothing else could collide. From experimentation, it doesn't take long
    // for all collisions to occur.
    let mut particles = particles;
    for _ in 0..2000 {

        let mut positions = HashMap::new();

        for p in &mut particles {
            p.step();
            *positions.entry(p.position).or_insert(0) += 1;
        }

        particles = particles
            .into_iter()
            .filter(|p| positions.get(&p.position).map(|&c| c).unwrap_or(0) <= 1)
            .collect();

    }
    println!("Star 2: {}", particles.len())

}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Particle {
    position: Coords,
    velocity: Coords,
    acceleration: Coords
}
impl Particle {
    fn new() -> Particle {
        Particle {
            position: Coords::new(),
            velocity: Coords::new(),
            acceleration: Coords::new()
        }
    }
    fn step(&mut self) {
        let v = self.velocity;
        let a = self.acceleration;
        let p = self.position;

        let v2 = Coords{ x:v.x + a.x, y:v.y + a.y, z:v.z + a.z };
        let p2 = Coords{ x:p.x + v2.x, y:p.y + v2.y, z:p.z + v2.z };

        self.position = p2;
        self.velocity = v2;
    }
    fn new_from_str(s: &str) -> Particle {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(p|v|a)=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>$").unwrap();
        }
        s.split(", ").filter_map(|part| {
            let caps = RE.captures(part)?;
            let l = caps[1].to_owned();
            let x = caps[2].parse().ok()?;
            let y = caps[3].parse().ok()?;
            let z = caps[4].parse().ok()?;
            Some((l, Coords{x,y,z}))
        }).fold(Particle::new(), |mut p, (l,c)| {
            match &*l {
                "p" => p.position = c,
                "v" => p.velocity = c,
                _  => p.acceleration = c
            }
            p
        })
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Coords {
    x: isize,
    y: isize,
    z: isize
}
impl Coords {
    fn new() -> Coords {
        Coords { x: 0, y: 0, z: 0 }
    }
    fn manhatten(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}