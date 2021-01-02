use std::str::Chars;
use std::env;
use std::fs;

const NEW_LINE: &str = "\r\n";

const DIGITS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

fn evaluate(expression: &str) -> Result<isize,String> {
    if expression.chars().all(|c| DIGITS.contains(&c)) {
        // parse number
        //println!("Evaluating number: {}", expression);
        match expression.parse() {
            Ok(number) => Ok(number),
            Err(_) => Err(format!("Not a number: {}", expression))
        }
    } else {
        // parse operation
        //println!("Evaluating operation: {}", expression);
        let mut remaining = expression.chars();
        let first_operand = read_expression(&mut remaining);
        let mut result: isize = evaluate(&first_operand)?;
        while let Some(operator) = remaining.next() {
            assert_eq!(remaining.next().unwrap(),' '); // consume space
            let next_operand = read_expression(&mut remaining);
            match operator {
                '+' => result += evaluate(&next_operand)?,
                '*' => result *= evaluate(&next_operand)?,
                _ => return Err(format!("Invalid operator: {}", operator))
            }
        }
        Ok(result)
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