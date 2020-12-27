use std::env;
use std::fs;
use std::slice::Iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let target_sum: &usize = &args[2].parse().expect("Target Sum must be an integer");
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<usize> = text.split("\r\n").map(|line| line.parse()
            .expect(&format!("Error parsing number: {}",line))).collect();
        let mut data = numbers.iter();
        let result = process(&mut data, target_sum);
        let min = result.iter().min().unwrap();
        let max = result.iter().max().unwrap();
        println!("Result: {}", **min + **max);
    } else {
        println!("Please provide 2 arguments: Filename, Target Sum");
    }
}

fn process<'a>(data: &'a mut Iter<usize>, target_sum: &usize) -> Vec<&'a usize> {
    let mut selection: Vec<&usize> = Vec::new();
    let mut sum: usize = 0;
    while !(sum == *target_sum && selection.len() > 1) {
        if sum > *target_sum {
            let remove_value = selection.remove(0);
            sum -= remove_value;
        } else {
            let add_value = data.next().unwrap();
            sum += add_value;
            selection.push(add_value);
        }
    }
    selection
}