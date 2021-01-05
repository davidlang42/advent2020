use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

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

    fn options() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast
        ]
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Color {
    White,
    Black
}

struct Tile {
    color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl Point {
    fn move_in_direction(&self, direction: &Direction) -> Self {
        let offset = direction.offset();
        Point(self.0 + offset.0, self.1 + offset.1)
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

    fn get_tile_color(&self, point: &Point) -> Color {
        match self.0.get(point) {
            Some(existing_tile) => existing_tile.color,
            None => Color::White
        }
    }

    fn count_adjacent_black(&self, point: &Point) -> usize {
        let mut count: usize = 0;
        for direction in Direction::options() {
            if self.get_tile_color(&point.move_in_direction(&direction)) == Color::Black {
                count += 1;
            }
        }
        count
    }

    fn bounds(&self) -> (Point, Point) {
        let mut min = Point(0,0);
        let mut max = Point(0,0);
        for point in self.0.keys() {
            if point.0 < min.0 {
                min.0 = point.0;
            }
            if point.1 < min.1 {
                min.1 = point.1;
            }
            if point.0 > max.0 {
                max.0 = point.0;
            }
            if point.1 > max.1 {
                max.1 = point.1
            }
        }
        (min, max)
    }

    fn run_daily(&mut self) {
        let mut new_tiles = HashMap::new();
        let (min, max) = self.bounds();
        for x in (min.0-1)..=(max.0+1) {
            for y in (min.1-1)..=(max.1+1) {
                let point = Point(x, y);
                let adjacent_black = self.count_adjacent_black(&point);
                let mut color = self.get_tile_color(&point);
                match color {
                    Color::Black => {
                        if adjacent_black == 0 || adjacent_black > 2 {
                            color = Color::White;
                        }
                    },
                    Color::White => {
                        if adjacent_black == 2 {
                            color = Color::Black;
                        }
                    }
                }
                new_tiles.insert(point, Tile { color });
            }
        }
        self.0 = new_tiles;
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
            point = point.move_in_direction(direction);
        }
        point
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let tiles_to_flip: Vec<Directions> = text.split(NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing Directions: {}", s))).collect();
        let days: usize = args[2].parse().expect("Days should be a number");
        // initial state
        let mut floor = Floor::new();
        for directions in tiles_to_flip {
            let point = directions.point();
            floor.flip_tile(point);
        }
        // day simulation
        for _ in 0..days {
            floor.run_daily();
        }
        println!("Day {}: {}", days, floor.count_black());
    } else {
        println!("Please provide 2 arguments: Filename, Days");
    }
}