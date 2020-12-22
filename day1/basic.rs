use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<u16> = text.split("\r\n").map(|s| s.parse()
           .expect(&format!("Error parsing number {}", s))).collect();
        process(numbers);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<u16>) {
    for num in list {
        println!("Num: {}", num);
    }
}