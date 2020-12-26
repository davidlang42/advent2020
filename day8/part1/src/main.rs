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
    fn execute(&mut self) -> (bool, isize, isize) { // success, delta program_counter, delta accumulator
        if self.executed {
            return (false, 0, 0); // failed because command already executed
        } else {
            self.executed = true;
            match self.command {
                InstructionType::ACC => (true, 1, self.argument),
                InstructionType::JMP => (true, self.argument, 0),
                InstructionType::NOP => (true, 1, 0)
            }
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
        let (success, delta_pc, delta_acc) = self.instructions[self.program_counter].execute();
        let new_pc: isize = self.program_counter as isize + delta_pc;
        if new_pc < 0 {
            false
        } else {
            self.program_counter = new_pc as usize;
            self.accumulator += delta_acc;
            success
        }
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
        "nop" => InstructionType::NOP,
        "acc" => InstructionType::ACC,
        "jmp" => InstructionType::JMP,
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