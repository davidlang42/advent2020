use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";
const SIZE: usize = 10;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum EdgeLocation {
    Top, Bottom, Left, Right
}

impl EdgeLocation {
    fn opposite(&self) -> EdgeLocation {
        match self {
            EdgeLocation::Top => EdgeLocation::Bottom,
            EdgeLocation::Bottom => EdgeLocation::Top,
            EdgeLocation::Left => EdgeLocation::Right,
            EdgeLocation::Right => EdgeLocation::Left,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct EdgeMatch {
    tile: usize,
    location: EdgeLocation
}

struct Edge {
    location: EdgeLocation,
    data: [bool; SIZE],
}

struct Tile {
    number: usize,
    data: [[bool; SIZE]; SIZE],
    edges: [Edge; 4]
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
            edges: [
                Edge { location: EdgeLocation::Top, data: top},
                Edge { location: EdgeLocation::Bottom, data: bottom},
                Edge { location: EdgeLocation::Left, data: left},
                Edge { location: EdgeLocation::Right, data: right},
            ]
        }
    }

    fn transform(&self, required_left_edge: EdgeLocation, required_top_edge: EdgeLocation) -> Tile {

    }
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let tiles: Vec<Tile> = text.split(DOUBLE_NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing tile {}", s))).collect();
        let edges = match_edges(&tiles);
        let placed = place_tiles(&tiles, &edges);






        let corners: Vec<&Tile> = tiles.iter().filter(|t| edges.iter().filter(|(em,_)| em.tile == t.number).count() == 2).collect();
        println!("Found {} corners: {:?}", corners.len(), corners.iter().map(|t| t.number).collect::<Vec<usize>>());
        println!("Result: {}", corners.iter().map(|t| t.number).product::<usize>());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn place_tiles(tiles: &Vec<Tile>, edges: &HashMap<EdgeMatch, EdgeMatch>) -> HashMap<Point,Tile> {
    let mut remaining: Vec<&Tile> = tiles.iter().collect();
    let grid_length: usize = (tiles.len() as f64).sqrt() as usize;
    let last_index: usize = grid_length - 1;
    let mut placed: HashMap<Point,Tile> = HashMap::new();
    for row in 0..grid_length {
        for col in 0..grid_length {
            let edge_left = match placed.get(&Point { row, col: col - 1 }) {
                Some(tile_left) => edges.get(&EdgeMatch { tile: tile_left.number, location: EdgeLocation::Right }),
                None => None
            };
            let edge_top = match placed.get(&Point { row: row - 1, col }) {
                Some(tile_top) => edges.get(&EdgeMatch { tile: tile_top.number, location: EdgeLocation::Bottom }),
                None => None
            };
            let new_tile: Tile = match (edge_left, edge_top) {
                (None, None) => {
                    // place any corner first, the choice doesn't matter
                    let corner: Option<Tile> = None;
                    for existing_tile in tiles.iter() {
                        let matched_edges: Vec<EdgeLocation> = existing_tile.edges.iter().map(|e| edges.get(&EdgeMatch { tile: existing_tile.number, location: e.location })).filter(|o| o.is_some()).map(|em| em.unwrap().location).collect();
                        if matched_edges.len() == 2 {
                            corner = Some(existing_tile.transform(matched_edges[0].opposite(), matched_edges[1].opposite()));
                            break;
                        }
                    }
                    corner.unwrap()
                },
                (Some(left), None) => {
                    // placing edge in top row, not first corner
                    let existing_tile = tiles.iter().filter(|t| t.number == left.tile).nth(0).unwrap();
                    //TODO need to take into account that the top right corner will have 2 non-matching edges and it needs to be the one which isn't opposite left.location
                    let non_matched_edge = existing_tile.edges.iter().filter(|e| !edges.contains_key(&EdgeMatch { tile: existing_tile.number, location: e.location })).nth(0).unwrap().location;
                    existing_tile.transform(left.location, non_matched_edge)
                },
                (None, Some(top)) => {
                    // placing edge in first col, not top row
                    let existing_tile = tiles.iter().filter(|t| t.number == top.tile).nth(0).unwrap();
                    //TODO need to take into account that the bottom left corner will have 2 non-matching edges and it needs to be the one which isn't opposite top.location
                    let non_matched_edge = existing_tile.edges.iter().filter(|e| !edges.contains_key(&EdgeMatch { tile: existing_tile.number, location: e.location })).nth(0).unwrap().location;
                    existing_tile.transform(non_matched_edge, top.location)
                },
                (Some(left), Some(top)) => {
                    // placing middles
                    assert_eq!(left.tile, top.tile);
                    let existing_tile = tiles.iter().filter(|t| t.number == left.tile).nth(0).unwrap();
                    existing_tile.transform(left.location, top.location)
                }
            };
            placed.insert(Point { row, col }, new_tile);
        }
    }
    placed
}

fn could_match(a: &[bool; SIZE], b: &[bool; SIZE],)-> bool {
    itertools::equal(a, b) || itertools::equal(a.iter().rev(), b)
}

fn find_match(all_tiles: &Vec<Tile>, this_tile: &Tile, this_edge: &Edge) -> Option<EdgeMatch> {
    for other_tile in all_tiles.iter().filter(|t| t.number != this_tile.number) {
        for other_edge in other_tile.edges.iter() {
            if could_match(&this_edge.data, &other_edge.data) {
                return Some(EdgeMatch {
                    tile: other_tile.number,
                    location: other_edge.location
                });
            }
        }
    }
    None
}

fn match_edges(tiles: &Vec<Tile>) -> HashMap<EdgeMatch,EdgeMatch> {
    let mut matches: HashMap<EdgeMatch,EdgeMatch> = HashMap::new();
    for tile in tiles.iter() {
        for edge in tile.edges.iter() {
            if let Some(other_edge_match) = find_match(tiles, tile, edge) {
                let this_edge_match = EdgeMatch {
                    tile: tile.number,
                    location: edge.location
                };
                matches.insert(this_edge_match, other_edge_match);
                matches.insert(other_edge_match, this_edge_match);
            }
        }
    }
    matches
}