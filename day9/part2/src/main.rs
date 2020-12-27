use std::env;
use std::fs;
use std::slice::Iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let preamble_size: &usize = &args[2].parse().expect("Preamble must be an integer");
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let data: Vec<usize> = text.split("\r\n").map(|line| line.parse()
            .expect(&format!("Error parsing number: {}",line))).collect();
        let result = process(&mut data.iter(), preamble_size).unwrap();
        println!("Result: {}", result);
    } else {
        println!("Please provide 2 arguments: Filename, Preamble Size");
    }
}

fn process(data: &mut Iter<usize>, preamble_size: &usize) -> Result<usize,String> {
    let mut preamble: Vec<&usize> = Vec::new();
    // fill preamble
    for _i in 0..*preamble_size {
        preamble.push(data.next().unwrap());
    }
    // check data
    while let Some(value) = data.next() {
        if !check_sum(&preamble, value) {
            return Ok(value.clone()); // found mismatched value
        }
        preamble.remove(0);
        preamble.push(value);
    }
    // failed if no errors found
    Err("No mismatched data found.".to_string())
}

fn check_sum(list: &Vec<&usize>, target_value: &usize) -> bool {
    for (index1, value1) in list.iter().enumerate() {
        for value2 in list[(index1+1)..].iter() {
            if **value1 != **value2 && **value1 + **value2 == *target_value {
                return true; // 2 values in list sum to target
            }
        }
    }
    return false; // no sum found
}