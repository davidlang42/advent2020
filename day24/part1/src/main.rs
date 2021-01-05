use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use std::ops::AddAssign;

const NEW_LINE: &str = "\r\n";

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Direction::East => Point(0, -1),
            Direction::SouthEast => Point(-1, 0),
            Direction::SouthWest => Point(-1, 1),
            Direction::West => Point(0, 1),
            Direction::NorthWest => Point(1, 0),
            Direction::NorthEast => Point(1, -1)
        }
    }
}

#[derive(PartialEq)]
enum Color {
    White,
    Black
}

struct Tile {
    color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Tile {
    fn new() -> Self {
        Tile {
            color: Color::White
        }
    }

    fn flip(&mut self) {
        self.color = match self.color {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

struct Floor(HashMap<Point,Tile>);

impl Floor {
    fn new() -> Self {
        Floor(HashMap::new())
    }

    fn count_black(&self) -> usize {
        self.0.values().filter(|t| t.color == Color::Black).count()
    }

    fn flip_tile(&mut self, point: Point) {
        match self.0.get_mut(&point) {
            Some(existing_tile) => existing_tile.flip(),
            None => {
                let mut new_tile = Tile::new();
                new_tile.flip();
                self.0.insert(point, new_tile);
            }
        }
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

impl Directions {
    fn point(&self) -> Point {
        let mut point = Point(0, 0);
        for direction in self.0.iter() {
            point += direction.offset();
        }
        point
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
            let point = directions.point();
            floor.flip_tile(point);
        }
        println!("Result: {}", floor.count_black());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}