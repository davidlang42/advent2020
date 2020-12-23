use std::env;
use std::fs;

fn wrapped_value(row: &Vec<bool>, column_index: usize) -> bool {
    row[column_index % row.len()]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let filename = &args[1];
        let right: &usize = &args[2].parse().expect("Right argument should be int");
        let down: &usize = &args[3].parse().expect("Down argument should be int");
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let rows: Vec<Vec<bool>> = text.split("\r\n").map(|s| s.chars().map(|c| c == '#').collect()).collect();
        let result = process(rows, right, down);
        println!("Result: {}", result);
    } else {
        println!("Please provide 3 arguments: Filename, Right, Down");
    }
}

fn process(list: Vec<Vec<bool>>, right: &usize, down: &usize) -> usize {
    let mut count: usize = 0;
    let mut column_index: usize = 0;
    for (row_index, row) in list.iter().enumerate() {
        if row_index % down == 0 {
            if wrapped_value(row,column_index) {
                count += 1;
            }
            column_index += right;
        }
    }
    count
}
