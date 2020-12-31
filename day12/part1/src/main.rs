use std::env;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::char::ParseCharError;

const NEW_LINE: &str = "\r\n";

#[derive(Debug, Copy, Clone)]
enum Direction {
    North, South, East, West
}

impl Direction {
    fn turn_right_once(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_right_many(&self, turns: usize) -> Self {
        let mut new_direction = *self;
        for _ in 0..turns {
            new_direction = new_direction.turn_right_once();
        }
        new_direction
    }
}

#[derive(Debug)]
enum Instruction {
    Move(Direction,usize), // direction, distance
    Rotate(isize), // clockwise rotation in degrees
    Forward(usize), // distance
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
            'F' => Ok(Instruction::Forward(n)),
            _ => Err(ParseError::Other(format!("Incorrect char: {}",c)))
        }
    }
}

#[derive(Debug)]
struct Navigation {
    facing: Direction,
    northings: isize,
    eastings: isize
}

impl Navigation {
    fn follow(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Move(direction, distance) => self.move_location(direction, distance),
                Instruction::Rotate(degrees) => self.rotate(degrees),
                Instruction::Forward(distance) => self.move_location(&self.facing.clone(), distance)
            }
            //println!("{:?}: {:?}", instruction, self);
        }
    }

    fn move_location(&mut self, direction: &Direction, distance: &usize) {
        match direction {
            Direction::North => self.northings += *distance as isize,
            Direction::East => self.eastings += *distance as isize,
            Direction::South => self.northings -= *distance as isize,
            Direction::West => self.eastings -= *distance as isize,
        }
    }

    fn rotate(&mut self, degrees: &isize) {
        match ((degrees % 360) + 360) % 360 {
            0 => (),
            90 => self.facing = self.facing.turn_right_once(),
            180 => self.facing = self.facing.turn_right_many(2),
            270 => self.facing = self.facing.turn_right_many(3),
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
            facing: Direction::East,
            northings: 0,
            eastings: 0
        };
        nav.follow(&instructions);
        println!("Final state: {} north, {} east, facing {:?}", nav.northings, nav.eastings, nav.facing);
        println!("Result: {}", nav.northings.abs() + nav.eastings.abs());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}