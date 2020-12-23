use std::env;
use std::fs;

fn wrapped_value(row: Vec<bool>, column_index: usize) -> bool {
    row[column_index % row.len()]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let rows: Vec<Vec<bool>> = text.split("\r\n").map(|s| s.chars().map(|c| c == '#').collect()).collect();
        let result = process(rows);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn process(list: Vec<Vec<bool>>) -> usize {
    let mut count: usize = 0;
    let mut column_index: usize = 0;
    for row in list {
        if wrapped_value(row,column_index) {
            count += 1;
        }
        column_index += 3;
    }
    count
}
