use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

const MONSTER_DATA: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

struct EdgeMap {
    edges: HashMap<Vec<bool>, Vec<Edge>>
}

impl EdgeMap {
    fn insert_reversible(&mut self, edge: Edge) {
        self.insert_inner(edge.data.clone(), edge.clone());
        self.insert_inner(edge.reversed_data(), edge);
    }

    fn insert_inner(&mut self, data: Vec<bool>, edge: Edge) {
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

#[derive(Clone)]
struct Edge {
    tile: usize,
    location: EdgeLocation,
    data: Vec<bool>,
}

impl Edge {
    fn reversed_data(&self) -> Vec<bool> {
        let mut rev = Vec::new();
        for value in self.data.iter().rev() {
            rev.push(*value);
        }
        rev
    }
}

struct Tile {
    number: usize,
    image: Image,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TILE_NUMBER: Regex = Regex::new("^Tile (\\d+):$").unwrap();
        }
        let mut lines: Vec<&str> = text.split(NEW_LINE).collect();
        let number: usize = match TILE_NUMBER.captures(lines.remove(0)) {
            Some(number_match) => number_match.get(1).unwrap().as_str().parse().expect("This regex should only return a number"),
            None => return Err("Tile number not found".to_string())
        };
        Ok(Tile { number, image: lines.join(NEW_LINE).parse()? })
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let lines = text.split(NEW_LINE);
        let mut data = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '#' => true,
                    _ => false,
                });
            }
            data.push(row);
        }
        Ok(Image { data })
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:\r\n{}", self.number, self.image)
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            let mut s = String::new();
            for value in row {
                if *value {
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
        let top = self.image.data[0].clone();
        let bottom = self.image.data[self.image.data.len()-1].clone();
        let mut left = Vec::new();
        let mut right = Vec::new();
        for row in self.image.data.iter() {
            left.push(row[0]);
            right.push(row[row.len()-1]);
        }
        vec![
            Edge { tile: self.number, location: EdgeLocation::Top, data: top},
            Edge { tile: self.number, location: EdgeLocation::Bottom, data: bottom},
            Edge { tile: self.number, location: EdgeLocation::Left, data: left},
            Edge { tile: self.number, location: EdgeLocation::Right, data: right},
        ]
    }

    fn get_edge(&self, location: &EdgeLocation) -> Edge {
        self.get_edges().into_iter().filter(|e| e.location == *location).nth(0).unwrap()
    }

    fn transform(&self, required_left_edge: EdgeLocation, required_top_edge: EdgeLocation) -> Result<Tile, String> {
        let image = match (required_left_edge, required_top_edge) {
            (EdgeLocation::Left, EdgeLocation::Top) => Image { data: self.image.data.clone() },
            (EdgeLocation::Bottom, EdgeLocation::Left) => self.image.rotate_clockwise(),
            (EdgeLocation::Right, EdgeLocation::Bottom) => self.image.rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Top, EdgeLocation::Right) => self.image.rotate_clockwise().rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Top, EdgeLocation::Left) => self.image.flip_vertical().rotate_clockwise(),
            (EdgeLocation::Right, EdgeLocation::Top) => self.image.flip_vertical().rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Bottom, EdgeLocation::Right) => self.image.flip_vertical().rotate_clockwise().rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Left, EdgeLocation::Bottom) => self.image.flip_vertical(),
            _ => return Err(format!("Cannot transform to have {:?} at the top and {:?} on the left", required_top_edge, required_left_edge))
        };
        Ok(Tile {
            number: self.number,
            image
        })
    }

    fn find_unmatched_edges(&self, edges: &EdgeMap) -> Vec<Edge> {
        let mut unmatched: Vec<Edge> = Vec::new();
        for edge in self.get_edges() {
            if edges.get_match(&edge).is_none() {
                unmatched.push(edge);
            }
        }
        unmatched
    }

    fn find_unmatched_edge(&self, edges: &EdgeMap) -> Edge {
        self.find_unmatched_edges(&edges).into_iter().nth(0).unwrap()
    }
}

struct Image {
    data: Vec<Vec<bool>>
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize
}

impl Image {
    fn find_pattern(&self, _pattern: &Image) -> Vec<Point> {
        //TODO (LATER)
        panic!();
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

    fn from_placed_tiles(_tiles: &Vec<Vec<Tile>>) -> Self {
        //TODO (LATER)
        panic!();
    }

    fn flip_vertical(&self) -> Image {
        let mut flipped_data = Vec::new();
        for row in self.data.iter() {
            flipped_data.insert(0, row.clone())
        }
        Image { data: flipped_data }
    }

    fn rotate_clockwise(&self) -> Image {
        let transposed = transpose(&self.data);
        let mut new_data: Vec<Vec<bool>> = Vec::new();
        for row in transposed.iter() {
            new_data.push(row.iter().rev().map(|b| *b).collect());
        }
        Image { data: new_data }
    }

    fn all_orientations(&self) -> [Image; 8] {
        let original = Image { data: self.data.clone() };
        let flipped = original.flip_vertical();
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


        let tile = tiles.into_iter().nth(0).unwrap();
        println!("Original {}", tile);
        let options = vec![EdgeLocation::Left, EdgeLocation::Right, EdgeLocation::Top, EdgeLocation::Bottom];
        for left in options.iter() {
            for top in options.iter() {
                match tile.transform(*left, *top) {
                    Ok(transformed) => println!("\r\nLEFT={:?}, TOP={:?}:\r\n{}", left, top, transformed.image),
                    Err(error) => println!("\r\nError: LEFT={:?}, TOP={:?}:\r\n{}", left, top, error),
                };
            }
        }
        

        panic!();


        let edges = EdgeMap::from_tiles(&tiles);
        let placed = arrange_tiles(&tiles, &edges);
        
        // confirm still works for part1
        let corners = get_corners(&placed);
        println!("Found {} corners: {:?}", corners.len(), corners.iter().map(|t| t.number).collect::<Vec<usize>>());
        println!("Part1 result: {}", corners.iter().map(|t| t.number).product::<usize>());

        //TODO
        // // find result for part2
        // let image = Image::from_placed_tiles(&placed);
        // let monster: Image = MONSTER_DATA.parse().unwrap();
        // let monsters = monster.all_orientations();
        // let found_monsters: Vec<Point> = monsters.iter().flat_map(|m| image.find_pattern(m)).collect();
        // let image_pixels: usize = image.count_active_pixels();
        // let monster_pixels: usize = found_monsters.len() * monster.count_active_pixels();
        // let remaining_pixels = image_pixels - monster_pixels;
        // println!("Part2 result: {}", remaining_pixels);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn arrange_tiles(tiles: &Vec<Tile>, edges: &EdgeMap) -> Vec<Vec<Tile>> {
    let grid = (tiles.len() as f64).sqrt() as usize;
    let mut placed: HashMap<Point,Tile> = HashMap::new();
    for row in 0..grid {
        for col in 0..grid {
            let edge_left = find_matching_edge(&placed, &edges, row as isize, col as isize - 1, &EdgeLocation::Right);
            let edge_top = find_matching_edge(&placed, &edges, row as isize - 1, col as isize, &EdgeLocation::Top);
            let this_tile: Tile = match (edge_left, edge_top) {
                (Some(left), Some(top)) => { // not first row or column
                    assert_eq!(left.tile, top.tile);
                    let existing_tile = find_tile(&tiles, left.tile);
                    existing_tile.transform(left.location, top.location).unwrap()
                },
                (Some(left), None) => { // first row, not first column
                    let existing_tile = find_tile(&tiles, left.tile);
                    let unmatched_edge = existing_tile.find_unmatched_edge(&edges);
                    existing_tile.transform(left.location, unmatched_edge.location).unwrap()
                },
                (None, Some(top)) => { // first column, not first row
                    let existing_tile = find_tile(&tiles, top.tile);
                    let unmatched_edge = existing_tile.find_unmatched_edge(&edges);
                    existing_tile.transform(unmatched_edge.location, top.location).unwrap()
                },
                (None, None) => { // only at (0,0)
                    let (existing_tile, unmatched_edges) = tiles.iter()
                        .map(|t| (t, t.find_unmatched_edges(&edges))) // calc unmatched edges of tile
                        .filter(|(_t, u)| u.len() == 2) // find corners
                        .nth(0).unwrap(); // pick any corner to start
                    existing_tile.transform(unmatched_edges[1].location, unmatched_edges[0].location).unwrap() // 2 possible orientations here, but doesn't matter, chosen only to match example for debugging
                }
            };
            println!("Placing tile {} at ({},{})",this_tile.number, row, col);
            placed.insert(Point { row, col }, this_tile);
        }
    }
    // convert to vec
    let mut data_vec: Vec<Vec<Tile>> = Vec::new();
    for row in 0..grid {
        let mut row_vec: Vec<Tile> = Vec::new();
        for col in 0..grid {
            row_vec.push(placed.remove(&Point { row, col }).unwrap())
        }
        data_vec.push(row_vec);
    }
    data_vec
}

fn find_tile(tiles: &Vec<Tile>, number: usize) -> &Tile {
    tiles.iter().filter(|t| t.number == number).nth(0).unwrap()
}

fn find_matching_edge(placed: &HashMap<Point,Tile>, edges: &EdgeMap, row: isize, col: isize, location: &EdgeLocation) -> Option<Edge> {
    if row < 0 || col < 0 {
        None
    } else {
        match placed.get(&Point { row: row as usize, col: col as usize }) {
            Some(tile) => match edges.get_match(&tile.get_edge(location)) {
                Some(matched_edge) => Some(matched_edge.clone()),
                None => None
            },
            None => None
        }
    }
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

//source: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}