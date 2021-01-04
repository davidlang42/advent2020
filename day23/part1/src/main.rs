use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt;

struct CupCircle {
    cups: VecDeque<usize>,
}

impl FromStr for CupCircle {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut cups: VecDeque<usize> = VecDeque::new();
        for c in line.chars() {
            cups.push_back(match c.to_digit(10) {
                Some(digit) => digit as usize,
                None => return Err(format!("Not a number: {}", c))
            });
        }
        Ok(CupCircle {
            cups
        })
    }
}

impl fmt::Display for CupCircle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cups)
    }
}

impl CupCircle {
    fn read_current(&self) -> usize {
        *self.cups.front().unwrap()
    }

    fn move_next(&mut self) {
        let first = self.cups.pop_front().unwrap();
        self.cups.push_back(first);
    }

    fn move_to_value(&mut self, value: usize) {
        while self.read_current() != value {
            self.move_next();
        }
    }

    fn take_cups(&mut self, number: usize) -> Vec<usize> {
        let mut taken = Vec::new();
        for _ in 0..number {
            taken.push(self.cups.pop_front().unwrap());
        }
        taken
    }

    fn place_cups(&mut self, cups: Vec<usize>) {
        for cup in cups.into_iter().rev() {
            self.cups.push_front(cup);
        }
    }

    fn find_next_lowest(&self, lower_than: &usize) -> usize { // wrap to highest if nothing lower
        let mut lower_numbers: Vec<&usize> = self.cups.iter().filter(|c| *c < lower_than).collect();
        lower_numbers.sort();
        match lower_numbers.last() {
            Some(next_lowest) => **next_lowest,
            None => *self.cups.iter().max().unwrap()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut circle: CupCircle = text.parse().expect("Error parsing cups");
        let moves: usize = args[2].parse().expect("Error parsing moves");
        for m in 0..moves {
            println!("Before move {}: {}", m+1, circle);
            let current = circle.read_current();
            circle.move_next(); // so we dont take the current
            let taken_cups = circle.take_cups(3);
            println!("Taken: {:?}", taken_cups);
            let destination = circle.find_next_lowest(&current);
            println!("Destination: {}", destination);
            circle.move_to_value(destination);
            circle.move_next(); // so we insert after destination
            circle.place_cups(taken_cups);
            println!("");
        }
        println!("Final: {}", circle);
    } else {
        println!("Please provide 2 arguments: Filename, Moves");
    }
}