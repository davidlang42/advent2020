use std::env;
use std::fs;

struct JoltageDifferences {
    diff1: usize,
    diff2: usize,
    diff3: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut numbers: Vec<usize> = text.split("\r\n").map(|line| line.parse()
            .expect(&format!("Error parsing number: {}",line))).collect();
        let result: JoltageDifferences = process(&mut numbers).unwrap();
        println!("Diffs: {},{},{}", result.diff1, result.diff2, result.diff3);
        println!("Result: {}", result.diff1 * result.diff3);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: &mut Vec<usize>) -> Result<JoltageDifferences, String> {
    list.sort();
    let mut joltages = list.iter();
    let mut last_joltage: &usize = &0;
    let mut result = JoltageDifferences {
        diff1: 0,
        diff2: 0,
        diff3: 0
    };
    while let Some(next_joltage) = joltages.next() {
        match *next_joltage - *last_joltage {
            1 => result.diff1 += 1,
            2 => result.diff2 += 1,
            3 => result.diff3 += 1,
            0 => return Err("You have 2 adapters with the same joltage".to_string()),
            _ => return Err("You have adapters with a joltage difference greater than 3".to_string())
        }
        last_joltage = next_joltage;
    }
    result.diff3 += 1; // always final diff of 3 between highest adapter and your device
    Ok(result)
}