use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let start: usize = args[2].parse().unwrap();
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let lines: Vec<&str> = text.split(NEW_LINE).collect();
        //let earliest_time: usize = lines[0].parse().expect("First line should be the earliest timestamp");
        let values: Vec<&str> = lines[1].split(",").collect();
        let possible_buses: Vec<Option<usize>> = values.iter().map(|s| s.parse().ok()).collect();
        let mut timestamp = start;
        while !verify(&possible_buses, &timestamp) {
            timestamp += 1;
            if timestamp % 100000000 == 0 {
                println!("{}",timestamp);
            }
        }
        println!("Result: {}", timestamp);
    } else {
        println!("Please provide 2 arguments: Filename, Starting point");
    }
}

fn verify(list: &Vec<Option<usize>>, start: &usize) -> bool {
    for (offset, possible_bus) in list.iter().enumerate() {
        match possible_bus {
            Some(bus) => {
                if (start + offset) % bus != 0 {
                    return false;
                }
            },
            None => ()
        }
    }
    true
}