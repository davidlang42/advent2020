use std::env;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::char::ParseCharError;

const NEW_LINE: &str = "\r\n";

#[derive(Debug)]
enum Direction {
    North, South, East, West
}

#[derive(Debug)]
enum Instruction {
    Move(Direction,usize), // direction, distance
    Rotate(isize), // clockwise rotation in degrees
    Waypoint(usize), // multiplier
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Char(ParseCharError),
    Other(String)
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl From<ParseCharError> for ParseError {
    fn from(e: ParseCharError) -> Self {
        ParseError::Char(e)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let c: char = line[0..1].parse()?;
        let n: usize = line[1..].parse()?;
        match c {
            'N' => Ok(Instruction::Move(Direction::North, n)),
            'E' => Ok(Instruction::Move(Direction::East, n)),
            'S' => Ok(Instruction::Move(Direction::South, n)),
            'W' => Ok(Instruction::Move(Direction::West, n)),
            'L' => Ok(Instruction::Rotate(n as isize * -1)),
            'R' => Ok(Instruction::Rotate(n as isize)),
            'F' => Ok(Instruction::Waypoint(n)),
            _ => Err(ParseError::Other(format!("Incorrect char: {}",c)))
        }
    }
}

#[derive(Debug)]
struct Location {
    northings: isize,
    eastings: isize
}

impl Location {
    fn move_location(&mut self, direction: &Direction, distance: &isize) {
        match direction {
            Direction::North => self.northings += *distance as isize,
            Direction::East => self.eastings += *distance as isize,
            Direction::South => self.northings -= *distance as isize,
            Direction::West => self.eastings -= *distance as isize,
        }
    }

    fn rotate_right_once(&mut self) {
        let new_eastings: isize = self.northings;
        let new_northings: isize = self.eastings * -1;
        self.northings = new_northings;
        self.eastings = new_eastings;
    }

    fn rotate_right_many(&mut self, turns: usize) {
        for _ in 0..turns {
            self.rotate_right_once();
        }
    }
}

#[derive(Debug)]
struct Navigation {
    ship: Location,
    waypoint: Location // relative to ship
}

impl Navigation {
    fn follow(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Move(direction, distance) => self.waypoint.move_location(direction, &(*distance as isize)),
                Instruction::Rotate(degrees) => self.rotate_waypoint(degrees),
                Instruction::Waypoint(multiplier) => {
                    self.ship.move_location(&Direction::North, &(self.waypoint.northings * *multiplier as isize));
                    self.ship.move_location(&Direction::East, &(self.waypoint.eastings * *multiplier as isize));
                }
            }
            //println!("{:?}: {:?}", instruction, self);
        }
    }

    fn rotate_waypoint(&mut self, degrees: &isize) {
        match ((degrees % 360) + 360) % 360 {
            0 => (),
            90 => self.waypoint.rotate_right_once(),
            180 => self.waypoint.rotate_right_many(2),
            270 => self.waypoint.rotate_right_many(3),
            _ => () // should throw an error if none of these values
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split(NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing instruction {}",s))).collect();
        let mut nav = Navigation {
            ship: Location {
                northings: 0,
                eastings: 0
            },
            waypoint: Location {
                northings: 1,
                eastings: 10
            }
        };
        nav.follow(&instructions);
        println!("Result: {}", nav.ship.northings.abs() + nav.ship.eastings.abs());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}