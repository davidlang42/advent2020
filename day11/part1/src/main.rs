use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

enum Seat {
    Occupied,
    Empty,
    None
}

impl Seat {
    fn from_char(c: &char) -> Self {
        match *c {
            '#' => Seat::Occupied,
            'L' => Seat::Empty,
            _ => Seat::None
        }
    }

    fn to_char(&self) -> char {
        match self {
            Seat::Occupied => '#',
            Seat::Empty => 'L',
            Seat::None => '.'
        }
    }

    fn is_occupied(&self) -> bool {
        match self {
            Seat::Occupied => true,
            _ => false
        }
    }
}

struct SeatingMap {
    seats: Vec<Vec<Seat>>,
    step: usize
}

impl SeatingMap {
    fn next_step(&mut self) -> bool { // returns true if any changes
        let mut changes = false;
        let mut new_seats: Vec<Vec<Seat>> = Vec::new();
        for (row_index, row) in self.seats.iter().enumerate() {
            let mut new_row: Vec<Seat> = Vec::new();
            for (col_index, seat) in row.iter().enumerate() {
                new_row.push(match seat {
                    Seat::Occupied => {
                        if self.count_surrounding_seats(row_index, col_index) >= 4 {
                            changes = true;
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    },
                    Seat::Empty => {
                        if self.count_surrounding_seats(row_index, col_index) == 0 {
                            changes = true;
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    },
                    Seat::None => Seat::None
                });
            }
            new_seats.push(new_row);
        }
        self.seats = new_seats;
        self.step += 1;
        changes
    }

    fn count_surrounding_seats(&self, row: usize, col: usize) -> usize {
        let r = row as isize;
        let c = col as isize;
        let adjacent_seats = [
            self.get_seat(r-1, c-1),
            self.get_seat(r-1, c),
            self.get_seat(r-1, c+1),
            self.get_seat(r, c-1),
            self.get_seat(r, c+1),
            self.get_seat(r+1, c-1),
            self.get_seat(r+1, c),
            self.get_seat(r+1, c+1),
        ];
        adjacent_seats.iter().filter(|seat| seat.is_occupied()).count()
    }

    fn get_seat(&self, row_index: isize, col_index: isize) -> &Seat {
        if row_index < 0 || col_index < 0 {
            &Seat::None
        } else {
            match self.seats.get(row_index as usize) {
                Some(row) => match row.get(col_index as usize) {
                    Some(seat) => seat,
                    None => &Seat::None
                },
                None => &Seat::None
            }
        }
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.iter().map(|row| row.iter().filter(|seat| seat.is_occupied()).count()).sum()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in self.seats.iter() {
            for seat in row.iter() {
                s.push(seat.to_char());
            }
            s.push_str(NEW_LINE);
        }
        s
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let seats: Vec<Vec<Seat>> = text.split(NEW_LINE).map(|s| s.chars().map(|c| Seat::from_char(&c)).collect()).collect();
        let mut state = SeatingMap { seats, step: 0 };
        //println!("Step {}:\r\n{}", state.step, state.to_string());
        while state.next_step() {
            //println!("Step {}:\r\n{}", state.step, state.to_string());
        }
        println!("Result: {}", state.count_occupied_seats());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}