use std::collections::VecDeque;
use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt;

/*
Note: this took ~3hrs to run
*/

const NUMBER_OF_CUPS: usize = 1000000;

struct CupCircle {
    cups: VecDeque<usize>,
    index: usize
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
            cups,
            index: 0
        })
    }
}

impl fmt::Display for CupCircle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cup) in self.cups.iter().enumerate() {
            if i == self.index {
                write!(f, "({})", cup)?
            } else {
                write!(f, " {} ", cup)?
            }
        }
        fmt::Result::Ok(())
    }
}

impl CupCircle {
    fn read_current(&self) -> usize {
        self.cups[self.index]
    }

    fn move_next(&mut self) {
        self.index += 1;
        if self.index == self.cups.len() {
            self.index = 0;
        }
    }

    fn find_value(&mut self, value: &usize) -> usize {
        for (i, cup) in self.cups.iter().enumerate() {
            if *cup == *value {
                return i;
            }
        }
        panic!(format!("Value not found: {}", value))
    }

    fn take_cups(&mut self, after_index: usize, number: usize) -> Vec<usize> {
        let mut taken = Vec::new();
        let mut at_index = after_index + 1;
        for _ in 0..number {
            if at_index == self.cups.len() {
                at_index = 0;
            }
            taken.push(self.cups.remove(at_index).unwrap());
            if at_index < self.index {
                self.index -= 1;
            }
        }
        taken
    }

    fn place_cups(&mut self, after_index: usize, cups: Vec<usize>) {
        if after_index < self.index {
            self.index += cups.len();
        }
        let at_index = after_index + 1;
        for cup in cups.into_iter().rev() {
            self.cups.insert(at_index, cup);
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
            if m % 1000 == 0 {
                println!("Move {}", m+1)
            }
            //println!("Before move {}: {}", m+1, circle);
            let current_value = circle.read_current();
            let taken_cups = circle.take_cups(circle.index, 3);
            //println!("Taken: {:?}", taken_cups);
            let mut destination_value: usize = current_value - 1;
            if destination_value == 0 {
                destination_value = NUMBER_OF_CUPS;
            }
            while taken_cups.contains(&destination_value) {
                destination_value -= 1;
                if destination_value == 0 {
                    destination_value = NUMBER_OF_CUPS;
                }
            }
            //println!("Destination: {}", destination_value);
            let destination_index = circle.find_value(&destination_value);
            circle.place_cups(destination_index, taken_cups);
            circle.move_next(); // next cup for next round
            //println!("");
        }
        //println!("Final: {}", circle);
        let index_of_one = circle.find_value(&1);
        circle.index = index_of_one;
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