use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

struct EdgeMap {
    edges: HashMap<Vec<bool>, Vec<Edge>>
}

impl EdgeMap {
    fn insert_reversible(&mut self, edge: Edge) {
        self.insert_inner(edge.data, edge);
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

    fn transform(&self, required_left_edge: EdgeLocation, required_top_edge: EdgeLocation) -> Result<Tile, String> {
        //TODO actually transform (by calling image.rotate_clockwise() and image.flip())
        let image = match (required_left_edge, required_top_edge) {
            (EdgeLocation::Left, EdgeLocation::Top) => Image { data: self.image.data.clone() },
            (EdgeLocation::Bottom, EdgeLocation::Left) => self.image.rotate_clockwise(),
            (EdgeLocation::Right, EdgeLocation::Bottom) => self.image.rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Top, EdgeLocation::Right) => self.image.rotate_clockwise().rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Top, EdgeLocation::Left) => self.image.flip(),
            (EdgeLocation::Right, EdgeLocation::Top) => self.image.flip().rotate_clockwise(),
            (EdgeLocation::Bottom, EdgeLocation::Right) => self.image.flip().rotate_clockwise().rotate_clockwise(),
            (EdgeLocation::Left, EdgeLocation::Bottom) => self.image.flip().rotate_clockwise().rotate_clockwise().rotate_clockwise(),
            _ => return Err("Cannot transform like that".to_string())
        };
        Ok(Tile {
            number: self.number,
            image
        })
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
    let image_data = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    image_data.parse().unwrap()
}