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
    Success(Vec<String>), // remaining strings after successful matches
    Failure,
    Error(String)
}

impl Rules {
    fn verify_root(&self, message: &str) -> Result<bool,String> {
        match self.verify_rule(message, &0) {
            VerifyResult::Success(remainings) => Ok(remainings.iter().any(|r| r.len() == 0)),
            VerifyResult::Failure => Ok(false),
            VerifyResult::Error(error) => Err(error)
        }
    }

    fn verify_rule(&self, message: &str, rule_index: &usize) -> VerifyResult {
        match self.rules.get(rule_index) {
            Some(rule) => match rule {
                Rule::SingleChar(required_char) => {
                    let mut chars = message.chars();
                    if chars.next() == Some(*required_char) {
                        VerifyResult::Success(vec![chars.as_str().to_string()])
                    } else {
                        VerifyResult::Failure
                    }
                },
                Rule::Series(series) => self.verify_rules(message, series),
                Rule::Options(series1, series2) => match (self.verify_rules(message, series1), self.verify_rules(message, series2)) {
                    (VerifyResult::Error(e1), _) => VerifyResult::Error(e1),
                    (_, VerifyResult::Error(e2)) => VerifyResult::Error(e2),
                    (VerifyResult::Success(s1), VerifyResult::Success(s2)) => VerifyResult::Success([s1, s2].concat()),
                    (VerifyResult::Success(s1), _) => VerifyResult::Success(s1),
                    (_, VerifyResult::Success(s2)) => VerifyResult::Success(s2),
                    _ => VerifyResult::Failure
                }
            },
            None => VerifyResult::Error(format!("Rule index {} not found", rule_index))
        }
    }

    fn verify_rules(&self, message: &str, rule_indicies: &Vec<usize>) -> VerifyResult {
        let mut remainings: Vec<String> = vec![message.to_string()];
        for i in rule_indicies {
            let mut new_remainings: Vec<String> = Vec::new();
            for remaining in remainings {
                match self.verify_rule(&remaining, i) {
                    VerifyResult::Success(mut successes) => new_remainings.append(&mut successes),
                    VerifyResult::Failure => (),
                    VerifyResult::Error(error) => return VerifyResult::Error(error)
                }
            }
            if new_remainings.len() == 0 {
                return VerifyResult::Failure;
            } else {
                remainings = new_remainings;
            }
        }
        VerifyResult::Success(remainings)
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
            match rules.verify_root(message) {
                Ok(result) => {
                    println!("Message: {} = {}", message, result);
                    if result {
                        count += 1;
                    }
                },
                Err(error) => println!("Error: {}", error)
            };
        }
        println!("Results: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}