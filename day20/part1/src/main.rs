use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";
const SIZE: usize = 10;

struct Tile {
    number: usize,
    data: [[bool; SIZE]; SIZE],
    // cached for frequent access:
    top: [bool; SIZE],
    bottom: [bool; SIZE],
    left: [bool; SIZE],
    right: [bool; SIZE],
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TILE_NUMBER: Regex = Regex::new("^Tile (\\d+):$").unwrap();
        }
        let lines: Vec<&str> = text.split(NEW_LINE).collect();
        assert_eq!(lines.len(), SIZE+1);
        let number: usize = match TILE_NUMBER.captures(lines[0]) {
            Some(number_match) => number_match.get(1).unwrap().as_str().parse().expect("This regex should only return a number"),
            None => return Err("Tile number not found".to_string())
        };
        let mut data = [[false; SIZE]; SIZE];
        for row in 0..SIZE {
            let mut chars = lines[row+1].chars();
            for col in 0..SIZE {
                data[row][col] = match chars.next() {
                    Some('#') => true,
                    Some(_) => false,
                    None => return Err("Row was too short".to_string())
                }
            }
        }
        Ok(Tile::new(number, data))
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:", self.number)?;
        for row in 0..SIZE {
            let mut s = String::new();
            for col in 0..SIZE {
                if self.data[row][col] {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Tile {
    fn edges(&self) -> [&[bool; SIZE]; 4] {
        [&self.top, &self.bottom, &self.left, &self.right]
    }

    fn new(number: usize, data: [[bool; SIZE]; SIZE]) -> Self {
        let top: [bool; SIZE] = data[0];
        let bottom: [bool; SIZE] = data[SIZE-1];
        let mut left: [bool; SIZE] = [false; SIZE];
        let mut right: [bool; SIZE] = [false; SIZE];
        for row in 0..SIZE {
            left[row] = data[row][0];
            right[row] = data[row][SIZE-1];
        }
        Tile {
            number,
            data,
            top,
            bottom,
            left,
            right,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let tiles: Vec<Tile> = text.split(DOUBLE_NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing tile {}", s))).collect();
        
        let mut corners: Vec<&Tile> = Vec::new();
        for tile in tiles.iter() {
            let mut matching_edges: usize = 0;
            for edge in tile.edges().iter() {
                let other_matching_edges = tiles.iter().filter(|t| t.number != tile.number).map(|t| t.edges().iter().filter(|e| could_match(e,edge)).count()).sum();
                match other_matching_edges {
                    0 => (),
                    1 => matching_edges += 1,
                    _ => println!("PROBLEM: Tile {} edge could match {} other edges", tile.number, other_matching_edges)
                }
            }
            println!("Tile {} matches on {} edges", tile.number, matching_edges);
            if matching_edges == 2 {
                corners.push(tile);
            }
        }

        println!("");
        println!("Found {} corners: {:?}", corners.len(), corners.iter().map(|t| t.number).collect::<Vec<usize>>());
        println!("Result: {}", corners.iter().map(|t| t.number).product::<usize>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn could_match(a: &[bool; SIZE], b: &[bool; SIZE])-> bool {
    itertools::equal(a, b) || itertools::equal(a.iter().rev(), b)
}