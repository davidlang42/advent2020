use std::env;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::char::ParseCharError;

struct PasswordCheck {
    min: u8,
    max: u8,
    letter: char,
    password: String
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Char(ParseCharError),
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

impl FromStr for PasswordCheck {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        let p0: Vec<&str> = parts[0].split("-").collect();
        let p1: Vec<&str> = parts[1].split(":").collect();
        Ok(PasswordCheck {
            min: p0[0].parse()?,
            max: p0[1].parse()?,
            letter: p1[0].parse()?,
            password: parts[2].to_string()
        })
    }
}

impl PasswordCheck {
    fn verify(&self) -> bool {
        let count: usize = self.password.chars().filter(|c| *c == self.letter).count();
        return count >= self.min.into() && count <= self.max.into();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let checks: Vec<PasswordCheck> = text.split("\r\n").map(|s| s.parse()
            .expect(&format!("Error parsing password check {}", s))).collect();
        let result = process(checks);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<PasswordCheck>) -> usize {
    list.iter().filter(|check| check.verify()).count()
}
