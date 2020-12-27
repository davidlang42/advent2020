use std::collections::HashSet;
use std::env;
use std::fs;

enum InstructionType {
    NOP,
    ACC,
    JMP
}

struct Instruction {
    command: InstructionType,
    argument: isize,
}

struct ExecutionState {
    instructions: Vec<Instruction>,
    visited: HashSet<usize>,
    program_counter: usize,
    accumulator: isize
}

impl ExecutionState {
    fn execute_step(&mut self) -> bool {
        if self.visited.contains(&self.program_counter) {
            return false; // failed because command already executed
        } else {
            self.visited.insert(self.program_counter);
            match self.instructions[self.program_counter].command {
                InstructionType::ACC => {
                    self.accumulator += self.instructions[self.program_counter].argument;
                    self.program_counter += 1;
                },
                InstructionType::JMP => {
                    let new_pc: isize = self.program_counter as isize + self.instructions[self.program_counter].argument;
                    if new_pc < 0 {
                        return false; // failed because jumped to before start of program
                    } else {
                        self.program_counter = new_pc as usize;
                    }
                },
                InstructionType::NOP => {
                    self.program_counter += 1;
                }
            }
            return true;
        }
    }

    fn execute_to_completion(&mut self) -> Result<isize,String> {
        while self.execute_step() {
            if self.is_complete() {
                return Ok(self.accumulator);
            }
        }
        Err("Program did not complete".to_string())
    }

    fn is_complete(&self) -> bool {
        self.program_counter == self.instructions.len()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split("\r\n").map(|line| parse_instruction(line)
            .expect(&format!("Error parsing instruction: {}",line))).collect();
        let mut state = ExecutionState {
            instructions: instructions,
            program_counter: 0,
            accumulator: 0,
            visited: HashSet::new()
        };
        match state.execute_to_completion() {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}\nAccumuator: {}", error, state.accumulator)
        }
        //TODO try toggling the nop/jmp instructions one at a time and re-executing until it succeeds
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_instruction(line: &str) -> Result<Instruction,String> {
    let parts: Vec<&str> = line.split(" ").collect();
    let com = match parts[0] {
        "nop" => InstructionType::NOP,
        "acc" => InstructionType::ACC,
        "jmp" => InstructionType::JMP,
        _ => return Err(format!("Command not recognised: {}", parts[0]))
    };
    let arg: isize = parts[1].replace("+","").parse().expect(&format!("Argument not integer: {}", parts[1]));
    Ok(Instruction {
        command: com,
        argument: arg
    })
}