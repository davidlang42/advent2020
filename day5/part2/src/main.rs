use std::env;
use std::fs;

fn parse_seat(line: &str) -> Result<usize,String> {
    if line.len() != 10 {
        return Err(format!("Length was {} instead of 10", line.len()));
    }
    let mut digit_value: usize = 2_usize.pow(10-1);
    let mut value: usize = 0;
    for digit in line.chars() {
        if digit == 'B' || digit == 'R' {
            value += digit_value;
        }
        digit_value /= 2;
    }
    Ok(value)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let seats: Vec<usize> = text.split("\r\n").map(|s| parse_seat(s)
            .expect(&format!("Error parsing seat {}", s))).collect();
        let result = process(seats);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<usize>) -> usize {
    let min = list.iter().min().unwrap();
    let max = list.iter().max().unwrap();
    let mut seat = min + 1;
    while seat < *max && list.contains(&seat) {
        seat += 1;
    }
    seat
}