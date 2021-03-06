use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

struct AdapterSet {
    numbers: HashSet<usize>,
    cached_combinations: HashMap<(usize,usize),usize>
}

impl AdapterSet {
    fn count_combinations(&mut self, from: usize, to: usize) -> usize {
        match self.cached_combinations.get(&(from,to)) {
            Some(result) => *result,
            None => {
                if !self.numbers.contains(&from) && from != 0 {
                    0
                } else if from+3 == to {
                    1
                } else {
                    let result = self.count_combinations(from+1, to) + self.count_combinations(from+2, to) + self.count_combinations(from+3, to);
                    self.cached_combinations.insert((from,to),result);
                    result
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: HashSet<usize> = text.split("\r\n").map(|line| line.parse()
            .expect(&format!("Error parsing number: {}",line))).collect();
        let target = numbers.iter().max().unwrap() + 3;
        let mut adapters = AdapterSet {
            numbers,
            cached_combinations: HashMap::new()
        };
        println!("Device joltage: {}", target);
        let result = adapters.count_combinations(0, target);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}