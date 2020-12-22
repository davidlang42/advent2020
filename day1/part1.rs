use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<u32> = text.split("\r\n").map(|s| s.parse()
           .expect(&format!("Error parsing number {}", s))).collect();
        let result = process(numbers).unwrap();
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<u32>) -> Result<u32, String> {
    for (index1, value1) in list.iter().enumerate() {
        for value2 in list[(index1+1)..].iter() {
            if value1 + value2 == 2020 {
                return Ok(value1 * value2);
            }
        }
    }
    return Err(format!("No number pair sum to 2020."));
}