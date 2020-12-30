use std::collections::HashSet;
use std::env;
use std::fs;

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

fn count_combinations(adapters: &HashSet<usize>, from: usize, to: usize) -> usize {
    let mut count: usize = 0;
    //println!("Finding combinations from {} to {}", from, to);
    if adapters.contains(&(from+1)) {
        //println!("Adapters contains {}", from+1);
        count += count_combinations(adapters, from+1, to);
    }
    if adapters.contains(&(from+2)) {
        //println!("Adapters contains {}", from+2);
        count += count_combinations(adapters, from+2, to);
    }
    if adapters.contains(&(from+3)) {
        //println!("Adapters contains {}", from+3);
        count += count_combinations(adapters, from+3, to);
    }
    if from+3 == to {
        //println!("Final adapter from {} to {}", from, to);
        count += 1;
    }
    //println!("Resulting combinations from {} to {}: {}", from, to, count);
    return count;
}