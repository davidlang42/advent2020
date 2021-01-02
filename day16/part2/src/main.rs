use std::collections::HashMap;
use std::num::ParseIntError;
use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;

const NEW_LINE: &str = "\r\n";
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

#[derive(Clone)]
struct Rule {
    field: String,
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
                field: rule_match.get(1).unwrap().as_str().to_string(),
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
        (value >= self.min1 && value <= self.max1) || (value >= self.min2 && value <= self.max2)
    }
}

#[derive(Clone, Debug)]
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
        let mut any_errors = false;
        for value in self.values.iter() {
            if !rules.iter().any(|r| r.verify(*value)) {
                errors += value;
                any_errors = true;
            }
        }
        if any_errors {
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
        let my_ticket: Ticket = sections[1].split(NEW_LINE).skip(1).next().unwrap().parse().unwrap();
        // println!("My ticket: {:?}", my_ticket);
        let tickets: Vec<Ticket> = sections[2].split(NEW_LINE).skip(1).map(|line| line.parse().unwrap()).collect();
        // println!("Total nearby tickets: {}", tickets.len());
        let valid: Vec<&Ticket> = tickets.iter().filter(|ticket| ticket.find_errors(&rules).is_none()).collect();
        // println!("Valid nearby tickets: {}", valid.len());
        let field_values: Vec<Vec<usize>> = transpose(valid.iter().map(|t| t.values.clone()).collect::<Vec<Vec<usize>>>());
        let ordered_rules = match_fields(&field_values, &rules);
        let mut result: usize = 1;
        for (index, rule) in ordered_rules {
            // println!("Field: {}, My Ticket Value: {}", rule.field, my_ticket.values[index]);
            if rule.field.starts_with("departure") {
                result *= my_ticket.values[index];
            }
        }
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn match_fields(field_values: &Vec<Vec<usize>>, rules: &Vec<Rule>) -> HashMap<usize,Rule> {
    let mut remaining_rules = rules.clone();
    let mut ordered_rules: HashMap<usize,Rule> = HashMap::new();
    let mut changes: bool = true;
    while remaining_rules.len() > 0 && changes {
        changes = false;
        // println!("Matching fields: {} remaining", remaining_rules.len());
        for (index, values) in field_values.iter().enumerate() {
            if !ordered_rules.contains_key(&index) {
                let candidate_indices: Vec<usize> = remaining_rules.iter().enumerate().filter(|(_i,r)| values.iter().all(|v| r.verify(*v))).map(|(i,_r)| i).collect();
                if candidate_indices.len() == 1 {
                    changes = true;
                    ordered_rules.insert(index, remaining_rules.remove(candidate_indices[0]));
                }
            }
        }
    }
    if remaining_rules.len() > 0 {
        println!("Failed: {} fields not matched: {:?}", remaining_rules.len(), remaining_rules.iter().map(|r| r.field.to_string()).collect::<Vec<String>>());
    }
    ordered_rules
}

//source: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}