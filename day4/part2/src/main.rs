use std::env;
use std::fs;
use std::collections::HashMap;

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
    match passport.get("byr") {
        None => return false,
        Some(byr) => if !validate_int(byr,1920,2002) {
            return false
        }
    }
    match passport.get("iyr") {
        None => return false,
        Some(iyr) => if !validate_int(iyr,2010,2020) {
            return false
        }
    }
    match passport.get("eyr") {
        None => return false,
        Some(eyr) => if !validate_int(eyr,2020,2030) {
            return false
        }
    }
    match passport.get("hgt") {
        None => return false,
        Some(hgt) => if !(validate_cm(hgt,150,193) || validate_in(hgt,59,76)) {
            return false
        }
    }
    match passport.get("hcl") {
        None => return false,
        Some(hcl) => if !validate_hex(hcl,6)  {
            return false
        }
    }
    match passport.get("ecl") {
        None => return false,
        Some(ecl) => if !validate_color(ecl)  {
            return false
        }
    }
    match passport.get("pid") {
        None => return false,
        Some(pid) => if !validate_digits(pid,9)  {
            return false
        }
    }
    return true;
}

fn validate_int(s: &str, min: isize, max: isize) -> bool {
    match s.parse::<isize>() {
        Err(_) => false,
        Ok(i) => i >= min && i <= max
    }
}

fn validate_cm(s: &str, min: isize, max :isize) -> bool {
    s.ends_with("cm") && validate_int(&s[..s.len()-2],min,max)
}

fn validate_in(s: &str, min: isize, max :isize) -> bool {
    s.ends_with("in") && validate_int(&s[..s.len()-2],min,max)
}

const HEX_CHARS: [char; 16]= ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
fn validate_hex(s: &str, digits: usize) -> bool {
    s.len() == digits + 1 && s.starts_with("#") && validate_chars(&s[1..], HEX_CHARS.to_vec())
}

const COLOR_STRINGS: [&str; 7] = ["amb","blu","brn","gry","grn","hzl","oth"];
fn validate_color(s: &str) -> bool {
    COLOR_STRINGS.iter().any(|option| s == *option)
}

const DIGIT_CHARS: [char; 10]= ['0','1','2','3','4','5','6','7','8','9'];
fn validate_digits(s: &str, digits: usize) -> bool {
    s.len() == digits && validate_chars(s, DIGIT_CHARS.to_vec())
}

fn validate_chars(s: &str, valid_chars: Vec<char>) -> bool {
    s.chars().all(|c| valid_chars.iter().any(|valid_char| c == *valid_char))
}