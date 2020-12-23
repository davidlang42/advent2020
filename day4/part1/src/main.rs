use std::env;
use std::fs;
use std::collections::HashMap;

const REQUIRED_KEYS: [&str; 7] = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

fn parse_passport(text: &str) -> HashMap<String,String> {
    let mut passport = HashMap::new();
    for pair in text.split("\r\n").flat_map(|line| line.split(" ")) {
        let key_value: Vec<&str> = pair.split(":").collect();
        passport.insert(key_value[0].to_string(), key_value[1].to_string());
    }
    return passport;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let passports: Vec<HashMap<String, String>> = text.split("\r\n\r\n").map(|s| parse_passport(s)).collect();
        let result = process(passports);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<HashMap<String, String>>) -> usize {
    list.iter().filter(|passport| validate(passport)).count()
}

fn validate(passport: &HashMap<String,String>) -> bool {
    REQUIRED_KEYS.iter().all(|key| passport.contains_key(*key))
}