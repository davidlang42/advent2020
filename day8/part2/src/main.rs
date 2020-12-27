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
            accumulator: 0
        };
        match state.execute_to_completion() {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}\nAccumuator: {}", error, state.accumulator)
        }
        //TODO try toggling the nop/jmp instructions one at a time and re-executing until it succeeds
        // (this requires changing ExecutionState to only need a mutable reference to instructions rather than ownership)
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