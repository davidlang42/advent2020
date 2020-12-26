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
    executed: bool
}

impl Instruction {
    fn execute(&mut self, state: &mut ExecutionState) -> bool {
        if self.executed {
            return false; // failed because command already executed
        } else {
            match self.command {
                InstructionType::ACC => {
                    state.accumulator += self.argument;
                    state.program_counter += 1;
                },
                InstructionType::JMP => {
                    let new_pc = state.program_counter as isize + self.argument;
                    if new_pc >= 0 {
                        state.program_counter = new_pc as usize;
                    } else {
                        return false; // failed because pc < 0
                    }
                },
                InstructionType::NOP => {
                    state.program_counter += 1;
                }
            }
            self.executed = true;
            return true;
        }
    }
}

struct ExecutionState {
    instructions: Vec<Instruction>,
    program_counter: usize,
    accumulator: isize
}

impl ExecutionState {
    fn execute_step(&mut self) -> bool {
        self.instructions[self.program_counter].execute(&mut self)
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
            accumulator: 0
        };
        let result = process(&mut state);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn parse_instruction(line: &str) -> Result<Instruction,String> {
    let parts: Vec<&str> = line.split(" ").collect();
    let com = match parts[0] {
        "NOP" => InstructionType::NOP,
        "ACC" => InstructionType::ACC,
        "JMP" => InstructionType::JMP,
        _ => return Err(format!("Command not recognised: {}", parts[0]))
    };
    let arg: isize = parts[1].replace("+","").parse().expect(&format!("Argument not integer: {}", parts[1]));
    Ok(Instruction {
        command: com,
        argument: arg,
        executed: false
    })
}

fn process(state: &mut ExecutionState) -> isize {
    while state.execute_step() { }
    state.accumulator
}