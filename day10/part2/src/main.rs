use std::collections::HashSet;
use std::env;
use std::fs;
//use cached::proc_macro::cached;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let target: usize = args[2].parse().unwrap();
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: HashSet<usize> = text.split("\r\n").map(|line| line.parse()
            .expect(&format!("Error parsing number: {}",line))).collect();
        //println!("Numbers: {:?}", numbers);
        let result = count_combinations(&numbers, 0, target);
        println!("Result: {}", result);
    } else {
        println!("Please provide 2 arguments: Filename, Device Joltage");
    }
}

//#[cached]
fn count_combinations(adapters: &HashSet<usize>, from: usize, to: usize) -> usize {
    if !adapters.contains(&from) && from != 0 {
        0
    } else if from+3 == to {
        1
    } else {
        count_combinations(adapters, from+1, to) + count_combinations(adapters, from+2, to) + count_combinations(adapters, from+3, to)
    }
}