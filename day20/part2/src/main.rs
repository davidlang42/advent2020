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

struct EdgeMap {
    edges: HashMap<[bool; SIZE], Vec<Edge>>
}

impl EdgeMap {
    fn insert_reversible(&mut self, edge: Edge) {
        self.insert_inner(edge.data, edge);
        self.insert_inner(edge.reversed_data(), edge);
    }

    fn insert_inner(&mut self, data: [bool; SIZE], edge: Edge) {
        match self.edges.remove(&data) {
            Some(mut vec) => {
                vec.push(edge);
                self.edges.insert(data, vec);
            },
            None => {
                self.edges.insert(data, vec![edge]);
            }
        }
    }

    fn get_match(&self, edge: &Edge) -> Option<&Edge> {
        self.edges.get(&edge.data)?.iter().filter(|e| e.tile != edge.tile).nth(0)
    }

    fn from_tiles(tiles: &Vec<Tile>) -> Self {
        let mut map = EdgeMap {
            edges: HashMap::new()
        };
        for tile in tiles.iter() {
            for edge in tile.get_edges() {
                map.insert_reversible(edge);
            }
        }
        map
    }
}

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

#[derive(Clone, Copy)]
struct Edge {
    tile: usize,
    location: EdgeLocation,
    data: [bool; SIZE],
}

impl Edge {
    fn reversed_data(&self) -> [bool; SIZE] {
        let mut rev = [false; SIZE];
        for i in 0..SIZE {
            rev[i] = self.data[SIZE-i-1];
        }
        rev
    }
}

struct Tile {
    number: usize,
    data: [[bool; SIZE]; SIZE],
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
        Ok(Tile { number, data })
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
    fn get_edges(&self) -> Vec<Edge> {
        let top: [bool; SIZE] = self.data[0];
        let bottom: [bool; SIZE] = self.data[SIZE-1];
        let mut left: [bool; SIZE] = [false; SIZE];
        let mut right: [bool; SIZE] = [false; SIZE];
        for row in 0..SIZE {
            left[row] = self.data[row][0];
            right[row] = self.data[row][SIZE-1];
        }
        vec![
            Edge { tile: self.number, location: EdgeLocation::Top, data: top},
            Edge { tile: self.number, location: EdgeLocation::Bottom, data: bottom},
            Edge { tile: self.number, location: EdgeLocation::Left, data: left},
            Edge { tile: self.number, location: EdgeLocation::Right, data: right},
        ]
    }

    fn transform(&self, required_left_edge: EdgeLocation, required_top_edge: EdgeLocation) -> Tile {
        //TODO actually transform
        Tile {
            number: self.number,
            data: self.data.clone()
        }
    }
}

struct Image {
    data: Vec<Vec<bool>>
}

struct Point {
    row: isize,
    col: isize
}

impl Image {
    fn find_pattern(&self, pattern: &Image) -> Vec<Point> {
        //TODO
        Vec::new()
    }

    fn count_active_pixels(&self) -> usize {
        let mut count: usize = 0;
        for row in self.data.iter() {
            for value in row.iter() {
                if *value {
                    count += 1;
                }
            }
        }
        count
    }

    fn from_placed_tiles(tiles: &Vec<Vec<Tile>>) -> Self {
        //TODO
        Image { data: Vec::new() }
    }

    fn flip(&self) -> Image {
        let mut flipped_data = Vec::new();
        for row in self.data.iter() {
            flipped_data.insert(0, row.clone())
        }
        Image { data: flipped_data }
    }

    fn rotate_clockwise(&self) -> Image {
        //TODO
        Image { data: Vec::new() }
    }

    fn all_orientations(&self) -> [Image; 8] {
        let original = Image { data: self.data.clone() };
        let flipped = original.flip();
        let original90 = original.rotate_clockwise();
        let flipped90 = flipped.rotate_clockwise();
        let original180 = original90.rotate_clockwise();
        let flipped180 = flipped90.rotate_clockwise();
        let original270 = original180.rotate_clockwise();
        let flipped270 = flipped180.rotate_clockwise();
        [
            original, flipped,
            original90, flipped90,
            original180, flipped180,
            original270, flipped270
        ]
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
        let edges = EdgeMap::from_tiles(&tiles);
        let placed = arrange_tiles(&tiles, &edges);
        
        // confirm still works for part1
        let corners = get_corners(&placed);
        println!("Found {} corners: {:?}", corners.len(), corners.iter().map(|t| t.number).collect::<Vec<usize>>());
        println!("Part1 result: {}", corners.iter().map(|t| t.number).product::<usize>());

        // find result for part2
        let image = Image::from_placed_tiles(&placed);
        let monster = generate_monster();
        let monsters = monster.all_orientations();
        let found_monsters: Vec<Point> = monsters.iter().flat_map(|m| image.find_pattern(m)).collect();
        let image_pixels: usize = image.count_active_pixels();
        let monster_pixels: usize = found_monsters.len() * monster.count_active_pixels();
        let remaining_pixels = image_pixels - monster_pixels;
        println!("Part2 result: {}", remaining_pixels);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn arrange_tiles(tiles: &Vec<Tile>, edges: &EdgeMap) -> Vec<Vec<Tile>> {
    //TODO
    Vec::new()
}

fn get_corners(placed_tiles: &Vec<Vec<Tile>>) -> [&Tile; 4] {
    let top_row = &placed_tiles[0];
    let bottom_row = &placed_tiles[placed_tiles.len()-1];
    [
        &top_row[0],
        &top_row[top_row.len()-1],
        &bottom_row[0],
        &bottom_row[bottom_row.len()-1],
    ]
}

fn generate_monster() -> Image {
    //TODO
    Image { data: Vec::new() }
}
















// #[derive(Hash, Eq, PartialEq)]
// struct Point {
//     row: isize,
//     col: isize
// }
//
// fn place_tiles_old(tiles: &Vec<Tile>, edges: &HashMap<EdgeMatch, EdgeMatch>) -> HashMap<Point,Tile> {
//     let grid_length: usize = (tiles.len() as f64).sqrt() as usize;
//     let mut placed: HashMap<Point,Tile> = HashMap::new();
//     for row in 0..grid_length {
//         for col in 0..grid_length {
//             let edge_left = match placed.get(&Point { row: row as isize, col: col as isize - 1 }) {
//                 Some(tile_left) => edges.get(&EdgeMatch { tile: tile_left.number, location: EdgeLocation::Right }),
//                 None => None
//             };
//             let edge_top = match placed.get(&Point { row: row as isize - 1, col: col as isize }) {
//                 Some(tile_top) => edges.get(&EdgeMatch { tile: tile_top.number, location: EdgeLocation::Bottom }),
//                 None => None
//             };
//             let new_tile: Tile = match (edge_left, edge_top) {
//                 (None, None) => {
//                     // place any corner first, the choice doesn't matter
//                     let mut corner: Option<Tile> = None;
//                     for existing_tile in tiles.iter() {
//                         let matched_edges: Vec<EdgeLocation> = existing_tile.edges().iter().map(|e| edges.get(&EdgeMatch { tile: existing_tile.number, location: e.location })).filter(|o| o.is_some()).map(|em| em.unwrap().location).collect();
//                         if matched_edges.len() == 2 {
//                             corner = Some(existing_tile.transform(matched_edges[0].opposite(), matched_edges[1].opposite()));
//                             break;
//                         }
//                     }
//                     corner.unwrap()
//                 },
//                 (Some(left), None) => {
//                     // placing edge in top row, not first corner
//                     let existing_tile = tiles.iter().filter(|t| t.number == left.tile).nth(0).unwrap();
//                     let non_matched_edges = existing_tile.get_unmatched_edges(&edges);
//                     let non_matched_edge = non_matched_edges.iter().filter(|el| **el != left.location.opposite()).nth(0).unwrap();
//                     existing_tile.transform(left.location, *non_matched_edge)
//                 },
//                 (None, Some(top)) => {
//                     // placing edge in first col, not top row
//                     let existing_tile = tiles.iter().filter(|t| t.number == top.tile).nth(0).unwrap();
//                     let non_matched_edges = existing_tile.get_unmatched_edges(&edges);
//                     let non_matched_edge = non_matched_edges.iter().filter(|el| **el != top.location.opposite()).nth(0).unwrap();
//                     existing_tile.transform(*non_matched_edge, top.location)
//                 },
//                 (Some(left), Some(top)) => {
//                     // placing middles
//                     assert_eq!(left.tile, top.tile);
//                     let existing_tile = tiles.iter().filter(|t| t.number == left.tile).nth(0).unwrap();
//                     existing_tile.transform(left.location, top.location)
//                 }
//             };
//             placed.insert(Point { row: row as isize, col: col as isize }, new_tile);
//         }
//     }
//     placed
// }

// fn could_match(a: &[bool; SIZE], b: &[bool; SIZE],)-> bool {
//     itertools::equal(a, b) || itertools::equal(a.iter().rev(), b)
// }

// fn find_match(all_tiles: &Vec<Tile>, this_tile: &Tile, this_edge: &Edge) -> Option<EdgeMatch> {
//     for other_tile in all_tiles.iter().filter(|t| t.number != this_tile.number) {
//         for other_edge in other_tile.edges().iter() {
//             if could_match(&this_edge.data, &other_edge.data) {
//                 return Some(EdgeMatch {
//                     tile: other_tile.number,
//                     location: other_edge.location
//                 });
//             }
//         }
//     }
//     None
// }

// fn match_edges(tiles: &Vec<Tile>) -> HashMap<EdgeMatch,EdgeMatch> {
//     let mut matches: HashMap<EdgeMatch,EdgeMatch> = HashMap::new();
//     for tile in tiles.iter() {
//         for edge in tile.edges().iter() {
//             if let Some(other_edge_match) = find_match(tiles, tile, edge) {
//                 let this_edge_match = EdgeMatch {
//                     tile: tile.number,
//                     location: edge.location
//                 };
//                 matches.insert(this_edge_match, other_edge_match);
//                 matches.insert(other_edge_match, this_edge_match);
//             }
//         }
//     }
//     matches
// }