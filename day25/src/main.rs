use std::fs;
use std::env;

const NEW_LINE: &str = "\r\n";
const SUBJECT_DIVIDER: usize = 20201227;
const HANDSHAKE_INITIAL: usize = 7;

struct SubjectNumber {
    subject_number: usize,
    current_value: usize
}

impl SubjectNumber {
    fn transform(&mut self) {
        self.current_value = (self.current_value * self.subject_number) % SUBJECT_DIVIDER;
    }

    fn transform_loop(&mut self, loop_size: usize) {
        for _ in 0..loop_size {
            self.transform();
        }
    }

    fn new(subject_number: usize) -> Self {
        SubjectNumber {
            subject_number,
            current_value: 1
        }
    }
}

struct Handshake {
    card_pub_key: usize,
    door_pub_key: usize
}

impl Handshake {
    fn find_encryption_key(self) -> usize {
        let mut subject_pub = SubjectNumber::new(HANDSHAKE_INITIAL);
        let mut loop_size: usize = 0;
        loop {
            subject_pub.transform();
            loop_size += 1;
            if subject_pub.current_value == self.card_pub_key {
                let mut subject_enc = SubjectNumber::new(self.door_pub_key);
                subject_enc.transform_loop(loop_size);
                return subject_enc.current_value;
            }
            if subject_pub.current_value == self.door_pub_key {
                let mut subject_enc = SubjectNumber::new(self.card_pub_key);
                subject_enc.transform_loop(loop_size);
                return subject_enc.current_value;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let pub_keys: Vec<usize> = text.split(NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing public key: {}", s))).collect();
        assert_eq!(pub_keys.len(), 2);
        let handshake = Handshake {
            card_pub_key: pub_keys[0],
            door_pub_key: pub_keys[1]
        };
        let encryption_key = handshake.find_encryption_key();
        println!("Result: {}", encryption_key);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}