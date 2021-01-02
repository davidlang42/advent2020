use std::str::Chars;
use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

const DIGITS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

fn evaluate(expression: &str) -> Result<isize,String> {
    if expression.chars().all(|c| DIGITS.contains(&c)) {
        // parse number
        println!("Evaluating number: {}", expression);
        match expression.parse() {
            Ok(number) => Ok(number),
            Err(_) => Err(format!("Not a number: {}", expression))
        }
    } else {
        // parse expression
        println!("Evaluating expression: {}", expression);
        let mut remaining = expression.chars();
        let first_operand = read_expression(&mut remaining);
        println!("First operand: {}", first_operand);
        match remaining.next() {
            Some(operator) => {
                println!("Operator: {}", operator);
                remaining.next(); // consume space
                let second_operand = remaining.as_str();
                println!("Second operand: {}", second_operand);
                match operator {
                    '+' => Ok(evaluate(&first_operand)? + evaluate(second_operand)?),
                    '*' => Ok(evaluate(&first_operand)? * evaluate(second_operand)?),
                    _ => Err(format!("Operator not found: {}", operator))
                }
            },
            None => evaluate(&first_operand)
        }
    }
}

fn read_expression(iter: &mut Chars) -> String { // reads digits until a space, or everything inside (brackets)
    let mut brackets: usize = 0;
    let mut s = String::new();
    while let Some(c) = iter.next() {
        if c == ' ' && brackets == 0 {
            break;
        } else if c == '(' {
            brackets += 1;
            if brackets == 1 {
                continue; // dont include the first open bracket
            }
        } else if c == ')' {
            brackets -= 1;
            if brackets == 0 {
                continue; // dont include the last close bracket
            }
        }
        s.push(c);
    }
    s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let expressions: Vec<&str> = text.split(NEW_LINE).collect();
        let mut sum: isize = 0;
        for expression in expressions {
            let result = evaluate(expression).unwrap();
            println!("{} = {}", expression, result);
            sum += result;
        }
        println!("Sum of results: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}