use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let groups: Vec<Vec<HashSet<char>>> = text.split("\r\n\r\n")
            .map(|g| g.split("\r\n").map(|s| parse_person(s)).collect()).collect();
        let result = process(groups);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

const ALLOWED_CHARS: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
fn parse_person(line: &str) -> HashSet<char> {
    let mut person = HashSet::<char>::new();
    for c in line.chars() {
        if ALLOWED_CHARS.contains(&c) {
            person.insert(c);
        }
    }
    return person;
}

fn process(list: Vec<Vec<HashSet<char>>>) -> usize {
    list.iter().map(|g| ALLOWED_CHARS.iter().filter(|c| g.iter().all(|p| p.contains(c))).count()).sum()
}