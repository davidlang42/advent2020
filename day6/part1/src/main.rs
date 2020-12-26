use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let groups: Vec<HashSet<char>> = text.split("\r\n\r\n").map(|s| parse_group(s)).collect();
        let result = process(groups);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

const ALLOWED_CHARS: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
fn parse_group(line: &str) -> HashSet<char> {
    let mut group = HashSet::<char>::new();
    for c in line.chars() {
        if ALLOWED_CHARS.contains(&c) {
            group.insert(c);
        }
    }
    return group;
}

fn process(list: Vec<HashSet<char>>) -> usize {
    list.iter().map(|group| group.len()).sum()
}