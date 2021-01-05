use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

const NEW_LINE: &str = "\r\n";

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
            Direction::NorthEast => Direction::SouthWest
        }
    }

    fn adjacent_left(&self) -> Self {
        match self {
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
            Direction::NorthEast => Direction::East
        }
    }

    fn adjacent_left2(&self) -> Self {
        self.adjacent_left().adjacent_left()
    }

    fn adjacent_right(&self) -> Self {
        match self {
            Direction::East => Direction::NorthEast,
            Direction::SouthEast => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::SouthWest,
            Direction::NorthWest => Direction::West,
            Direction::NorthEast => Direction::NorthWest
        }
    }

    fn adjacent_right2(&self) -> Self {
        self.adjacent_right().adjacent_right()
    }
}

#[derive(PartialEq)]
enum Color {
    White,
    Black
}

struct Tile<'a> {
    color: Color,
    adjacent_tiles: HashMap<Direction,&'a Tile<'a>>
}

impl Tile<'_> {
    fn new() -> Self {
        Tile {
            color: Color::White,
            adjacent_tiles: HashMap::new()
        }
    }

    fn flip(&mut self) {
        self.color = match self.color {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

struct Floor<'a> {
    tiles: Vec<Tile<'a>>,
}

impl Floor<'_> {
    fn new() -> Self {
        Floor {
            tiles: vec![Tile::new()]
        }
    }

    fn count_black(&self) -> usize {
        self.tiles.iter().filter(|t| t.color == Color::Black).count()
    }

    fn flip_tile(&mut self, directions: &Directions) {
        let mut tile: &Tile = &self.tiles[0];
        for direction in directions.0.iter() {
            let mut next_tile = match tile.adjacent_tiles.get_mut(direction) {
                Some(existing_tile) => existing_tile,
                None => {
                    let mut new_tile = Tile::new();
                    // link direction
                    tile.adjacent_tiles.insert(*direction, &new_tile);
                    new_tile.adjacent_tiles.insert(direction.opposite(), &tile);
                    // link adjacent_left
                    if let Some(a_left) = tile.
                    tile.adjacent_tiles.insert(direction.adjacent_left(), &new_tile);
                    new_tile.adjacent_tiles.insert(direction.opposite(), &tile);
                    // link adjacent_left2
                    // link adjacent_right
                    // link adjacent_right2
                    // link opposite
                    // move tile into floor
                    self.tiles.push(new_tile);
                }
            }
            tile = next_tile;
        }
        tile.flip();
    }
}

#[derive(Debug)]
struct Directions(Vec<Direction>);

impl FromStr for Directions {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut directions = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            directions.push(match c {
                'e' => Direction::East,
                'w' => Direction::West,
                'n' => match chars.next() {
                    Some(c2) => match c2 {
                        'e' => Direction::NorthEast,
                        'w' => Direction::NorthWest,
                        _ => return Err(format!("Invalid 2nd character after '{}': {}", c, c2))
                    },
                    None => return Err(format!("Missing 2nd character after '{}'", c))
                },
                's' => match chars.next() {
                    Some(c2) => match c2 {
                        'e' => Direction::SouthEast,
                        'w' => Direction::SouthWest,
                        _ => return Err(format!("Invalid 2nd character after '{}': {}", c, c2))
                    },
                    None => return Err(format!("Missing 2nd character after '{}'", c))
                },
                _ => return Err(format!("Invalid 1st character: {}", c))
            });
        }
        Ok(Directions(directions))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let tiles_to_flip: Vec<Directions> = text.split(NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing Directions: {}", s))).collect();
        let mut floor = Floor::new();
        for directions in tiles_to_flip {
            floor.flip_tile(&directions);
        }
        println!("Result: {}", floor.count_black());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}