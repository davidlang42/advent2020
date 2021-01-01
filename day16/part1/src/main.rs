use std::num::ParseIntError;
use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

struct Rule {
    _field: String,
    min1: usize,
    max1: usize,
    min2: usize,
    max2: usize
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_REGEX: Regex = Regex::new("^(.*): (\\d*)-(\\d*) or (\\d*)-(\\d*)$").unwrap();
        }
        match RULE_REGEX.captures(line) {
            Some(rule_match) => Ok(Rule {
                _field: rule_match.get(1).unwrap().as_str().to_string(),
                min1: rule_match.get(2).unwrap().as_str().parse().expect("This regex should return a number"),
                max1: rule_match.get(3).unwrap().as_str().parse().expect("This regex should return a number"),
                min2: rule_match.get(4).unwrap().as_str().parse().expect("This regex should return a number"),
                max2: rule_match.get(5).unwrap().as_str().parse().expect("This regex should return a number"),
            }),
            None => Err(format!("Did not match regex: {}", line))
        }
    }
}

impl Rule {
    fn verify(&self, value: usize) -> bool {
        value >= self.min1 && value <= self.max1 || value >= self.min2 && value <= self.max2
    }
}

struct Ticket {
    values: Vec<usize>
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: line.split(",").map(|s| s.parse().unwrap()).collect()
        })
    }
}

impl Ticket {
    fn find_errors(&self, rules: &Vec<Rule>) -> Option<usize> {
        let mut errors: usize = 0;
        for value in self.values.iter() {
            if !rules.iter().any(|r| r.verify(*value)) {
                errors += value;
            }
        }
        if errors > 0 {
            Some(errors)
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split(DOUBLE_NEW_LINE).collect();
        let rules: Vec<Rule> = sections[0].split(NEW_LINE).map(|line| line.parse().unwrap()).collect();
        let _my_ticket: Ticket = sections[1].split(NEW_LINE).skip(1).next().unwrap().parse().unwrap();
        let tickets: Vec<Ticket> = sections[2].split(NEW_LINE).skip(1).map(|line| line.parse().unwrap()).collect();
        let sum: usize = tickets.iter().map(|ticket| ticket.find_errors(&rules)).filter(|o| o.is_some()).map(|o| o.unwrap()).sum();
        println!("Result: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
