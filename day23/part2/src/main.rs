use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt;

/*
SPEED:
^^^^^^
VecDeque= ~0.5 seconds per 100
*/

const NUMBER_OF_CUPS: usize = 1000000;

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
        for i in (cups.iter().max().unwrap()+1)..=NUMBER_OF_CUPS {
            cups.push_back(i);
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
            if m % 100 == 0 {
                println!("Move {}", m+1)
            }
            //println!("Before move {}: {}", m+1, circle);
            let current = circle.read_current();
            circle.move_next(); // so we dont take the current
            let taken_cups = circle.take_cups(3);
            //println!("Taken: {:?}", taken_cups);
            let destination = circle.find_next_lowest(&current);
            //println!("Destination: {}", destination);
            circle.move_to_value(destination);
            circle.move_next(); // so we insert after destination
            circle.place_cups(taken_cups);
            circle.move_to_value(current); // back to initial current cup
            circle.move_next(); // next cup for next round
            //println!("");
        }
        //println!("Final: {}", circle);
        circle.move_to_value(1);
        circle.move_next(); // dont read 1
        let n1 = circle.read_current();
        circle.move_next();
        let n2 = circle.read_current();
        println!("Next 2 cups: {}, {}", n1, n2);
        println!("Product: {}", n1 * n2);
    } else {
        println!("Please provide 2 arguments: Filename, Moves");
    }
}