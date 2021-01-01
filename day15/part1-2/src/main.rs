use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let starting_numbers: Vec<usize> = args[1].split(",").map(|s| s.parse().unwrap()).collect();
        let steps: usize = args[2].parse().unwrap();
        let result = process(&starting_numbers, steps);
        println!("Result: {}", result);
    } else {
        println!("Please provide 2 arguments: Input, Steps");
    }
}

fn process(starting_numbers: &Vec<usize>, steps: usize) -> usize {
    let mut previous_steps: HashMap<usize,usize> = HashMap::new();
    let mut last_number: usize = 0;
    for (step, number) in starting_numbers.iter().enumerate() {
        previous_steps.insert(*number,step);
        last_number = *number;
        //println!("Step {}: {}", step+1, last_number);
    }
    previous_steps.remove(&last_number);
    for step in starting_numbers.len()..steps {
        let next_number: usize = match previous_steps.get(&last_number) {
            Some(previous_step) => step - previous_step - 1,
            None => 0
        };
        previous_steps.insert(last_number, step-1);
        last_number = next_number;
        //println!("Step {}: {}", step+1, last_number);
    }
    last_number
}