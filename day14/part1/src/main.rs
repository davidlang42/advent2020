use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;

const NEW_LINE: &str = "\r\n";
const BITS: usize = 36;

#[derive(Copy, Clone)]
enum BitMask {
    Override(bool),
    Passthrough()
}

struct MachineState {
    mask: [BitMask; BITS],
    memory: HashMap<usize,usize>
}

impl MachineState {
    fn read_address(&self, address: usize) -> &usize {
        match self.memory.get(&address) {
            Some(value) => value,
            None => &0
        }
    }

    fn write_address(&mut self, address: &usize, decimal: &usize) {
        let mut binary: [bool; BITS] = decimal_to_binary(decimal);
        for i in 0..BITS {
            match self.mask[i] {
                BitMask::Override(bit) => binary[i] = bit,
                BitMask::Passthrough() => ()
            }
        }
        self.memory.insert(*address, binary_to_decimal(&binary));
    }

    fn set_mask(&mut self, new_mask: &str) {
        for i in 0..BITS {

        }
    }
}

fn decimal_to_binary(decimal: &usize) -> [bool; BITS] {
    let mut remaining: usize = *decimal;
    let mut binary = [false; BITS];
    let mut bit_value = 2_usize.pow(BITS as u32 - 1);
    for i in 0..BITS {
        if remaining >= bit_value {
            binary[i] = true;
            remaining -= bit_value;
        }
        bit_value /= 2;
    }
    binary
}

fn binary_to_decimal(binary: &[bool; BITS]) -> usize {
    let mut decimal: usize = 0;
    for (power, bit) in binary.iter().rev().enumerate() {
        if *bit {
            decimal += 2_usize.pow(power as u32);
        }
    }
    decimal
}

enum Instruction {
    WriteAddress(usize,usize),
    SetMask(String)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref WRITE_ADDRESS: Regex = Regex::new("^mem\\[(\\d*)\\] = (\\d*)$").unwrap();
            static ref SET_MASK: Regex = Regex::new("^mask = ([01X]*)$").unwrap();
        }
        match WRITE_ADDRESS.captures(line) {
            Some(mem_match) => {
                let address: usize = mem_match.get(1).unwrap().as_str().parse().expect("This regex should only return a number");
                let value: usize = mem_match.get(2).unwrap().as_str().parse().expect("This regex should only return a number");
                Ok(Instruction::WriteAddress(address, value))
            },
            None => match SET_MASK.captures(line) {
                Some(mask_match) => Ok(Instruction::SetMask(mask_match.get(1).unwrap().as_str().to_string())),
                None => Err(format!("Did not match regex: {}", line))
            }
        }
    }
}

impl Instruction {
    fn run(&self, state: &mut MachineState) {
        match self {
            Instruction::SetMask(mask) => state.set_mask(mask),
            Instruction::WriteAddress(address, value) => state.write_address(address, value)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split(NEW_LINE).map(|s| s.parse().unwrap()).collect();
        let mut state = MachineState {
            mask: [BitMask::Passthrough(); BITS],
            memory: HashMap::new()
        };
        for instruction in instructions {
            instruction.run(&mut state);
        }
        let sum: usize = state.memory.iter().map(|(_,value)| *value).sum();
        println!("Result: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
