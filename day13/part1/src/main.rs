use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

struct BusTime {
    bus: usize,
    timestamp: usize,
    wait: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let lines: Vec<&str> = text.split(NEW_LINE).collect();
        let earliest_time: usize = lines[0].parse().expect("First line should be the earliest timestamp");
        let values: Vec<&str> = lines[1].split(",").collect();
        let buses: Vec<usize> = values.iter().map(|s| s.parse()).filter(|o| o.is_ok()).map(|s| s.unwrap()).collect();
        let result = process(&buses, &earliest_time);
        println!("Bus: {}, Time: {}, Wait: {}", result.bus, result.timestamp, result.wait);
        println!("Result: {}", result.bus * result.wait);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: &Vec<usize>, minimum: &usize) -> BusTime {
    let mut timestamp = *minimum;
    loop {
        for bus in list {
            if timestamp % bus == 0 {
                return BusTime {
                    bus: *bus,
                    timestamp,
                    wait: timestamp - minimum
                }
            }
        }
        timestamp += 1;
    }
}