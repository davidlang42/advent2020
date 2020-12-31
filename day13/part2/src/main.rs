use std::env;
use std::fs;
use num::integer::lcm;

const NEW_LINE: &str = "\r\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let lines: Vec<&str> = text.split(NEW_LINE).collect();
        //let earliest_time: usize = lines[0].parse().expect("First line should be the earliest timestamp");
        let values: Vec<&str> = lines[1].split(",").collect();
        let possible_buses: Vec<Option<usize>> = values.iter().map(|s| s.parse().ok()).collect();
        let result = process(&possible_buses);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: &Vec<Option<usize>>) -> usize {
    let mut timestamp: usize = 0;
    let mut offset: usize = 0;
    let mut increment: usize = 1;
    for possible_bus in list {
        match possible_bus {
            Some(bus) => {
                //println!("Looking for bus {} at timestamp {} + offset {}", bus, timestamp, offset);
                loop {
                    if (timestamp + offset) % bus == 0 {
                        increment = lcm(increment, *bus);
                        //println!("Found bus {} at timestamp {} + offset {} (now increment by {})", bus, timestamp, offset, increment);
                        break;
                    } else {
                        timestamp += increment;
                    }
                }
            },
            None => ()
        }
        offset += 1;
    }
    timestamp
}