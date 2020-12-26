use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

struct BagRule {
    outer_bag: String,
    inner_bags: HashMap<String, usize>
}

impl BagRule {
    fn contains_bag(&self, bag: &str, list: &Vec<BagRule>) -> bool {
        self.inner_bags.contains_key(bag) || self.inner_bags.iter().any(|bag_pair| list.iter().find(|rule| rule.outer_bag == *bag_pair.0).unwrap().contains_bag(bag, list))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let rules: Vec<BagRule> = text.split("\r\n").map(|line| parse_rule(line)
            .expect(&format!("Error parsing rule: {}",line))).collect();
        let result = process(rules, &"shiny gold");
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_rule(line: &str) -> Result<BagRule,String> {
    lazy_static! {
        static ref NON_EMPTY_RULE: Regex = Regex::new("^([\\w ]+) bags contain (\\d+) ([\\w ]+) bags?(?:, (\\d+) ([\\w ]+) bags?)?(?:, (\\d+) ([\\w ]+) bags?)?(?:, (\\d+) ([\\w ]+) bags?)?\\.$").unwrap(); // NOTE: this will match 1-4 inner bags
        static ref EMPTY_RULE: Regex = Regex::new("^(.+) bags contain no other bags\\.$").unwrap();
    }
    match NON_EMPTY_RULE.captures(line) {
        Some(non_empty_match) => {
            let mut groups = non_empty_match.iter();
            groups.next(); // full match string, eg: light red bags contain 1 bright white bag, 2 muted yellow bags.
            let outer: &str = groups.next().unwrap().unwrap().as_str();
            let mut inners: HashMap<String, usize> = HashMap::new();
            loop {
                let number: usize = match groups.next() {
                    Some(m) => match m {
                        Some(s) => s.as_str().parse().unwrap(),
                        None => continue
                    },
                    None => break
                };
                let bag: &str = groups.next().unwrap().unwrap().as_str();
                inners.insert(bag.to_string(), number);
            }
            Ok(BagRule {
                outer_bag: outer.to_string(),
                inner_bags: inners
            })
        },
        None => match EMPTY_RULE.captures(line) {
            Some(empty_match) => Ok(BagRule {
                outer_bag: empty_match.get(1).unwrap().as_str().to_string(),
                inner_bags: HashMap::new()
            }),
            None => Err("Rule didn't match regex.".to_string())
        }
    }
}

fn process(list: Vec<BagRule>, target_bag: &str) -> usize {
    list.iter().filter(|rule| rule.contains_bag(target_bag, &list)).count()
}