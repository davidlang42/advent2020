use std::str::Chars;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::char::ParseCharError;

const NEW_LINE: &str = "\r\n";

enum Rule {
    SingleChar(char),
    Series(Vec<usize>), // series of rule indicies in order
    Options(Vec<usize>,Vec<usize>) // two options of Series
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.len() == 3 && line.starts_with('"') && line.ends_with('"') {
            Ok(Rule::SingleChar(line.chars().nth(1).unwrap()))
        } else if line.contains("|") {
            let options: Vec<&str> = line.split("|").collect();
            Ok(Rule::Options(split_series(options[0]), split_series(options[1])))
        } else {
            Ok(Rule::Series(split_series(line)))
        }
    }
}

fn split_series(list: &str) -> Vec<usize> {
    list.split(' ').filter(|s| s.len() != 0).map(|s| s.parse().unwrap()).collect()
}

struct Rules {
    rules: HashMap<usize,Rule> // index, Rule
}

impl FromStr for Rules {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for line in text.split(NEW_LINE) {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                let index: usize = parts[0].parse()?;
                let rule: Rule = parts[1].parse()?;
                rules.insert(index, rule);
            } else {
                return Err(ParseError::Other("Invalid rule.".to_string()));
            }
        }
        Ok(Rules {
            rules
        })
    }
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

enum VerifyResult {
    Success(String), // remaining string
    Failure
}

impl Rules {
    fn verify_root(&self, message: &str) -> bool {
        match self.verify_rule(message, 0) {
            VerifyResult::Success(remaining) => remaining.as_str().len() == 0,
            VerifyResult::Failure => false
        }
    }

    fn verify_rule(&self, message: &str, rule_index: usize) -> VerifyResult {
        let mut chars = message.chars();
        match self.rules.get(&rule_index).expect(&format!("Rule index {} doesn't exist", rule_index)) {
            Rule::SingleChar(required_char) => match chars.next() {
                Some(next_char) => {
                    if *required_char == next_char {
                        VerifyResult::Success(chars.as_str().to_string())
                    } else {
                        VerifyResult::Failure
                    }
                } ,
                None => VerifyResult::Failure
            },
            Rule::Series(series) => self.verify_series(message, series),
            Rule::Options(series1,series2) => {
                let chars2 = chars.clone();
                match self.verify_series(chars, series1) {
                    VerifyResult::Success(remaining) => VerifyResult::Success(remaining),
                    VerifyResult::Failure => self.verify_series(chars2,series2)
                }
            }
        }
    }

    fn verify_series(&self, message: &str, series: &Vec<usize>) -> VerifyResult {
        let remaining = message;
        for i in series {
            match self.verify_rule(remaining, *i) {
                VerifyResult::Success(new_remaining) => remaining = new_remaining,
                VerifyResult::Failure => return VerifyResult::Failure
            }
        }
        VerifyResult::Success(remaining)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        let rules: Rules = sections[0].parse().unwrap();
        let messages: Vec<&str> = sections[1].split(NEW_LINE).collect();
        
        let mut count: usize = 0;
        for message in messages {
            let result = rules.verify_root(message);
            println!("Message: {} = {}", message, result);
            if result {
                count += 1;
            }
        }

        //let count = messages.iter().filter(|m| rules.verify_root(m)).count();
        println!("Results: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}