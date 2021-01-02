use std::str::Chars;
use std::env;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::char::ParseCharError;

const NEW_LINE: &str = "\r\n";

#[derive(Debug)]
enum Expression<'a> {
    Number(isize),
    Addition(&'a Expression<'a>,&'a Expression<'a>),
    Multiplication(&'a Expression<'a>,&'a Expression<'a>)
}

impl Expression<'_> {
    fn evaluate(&self) -> isize {
        match self {
            Expression::Number(number) => *number,
            Expression::Addition(a,b) => a.evaluate() + b.evaluate(),
            Expression::Multiplication(a,b) => a.evaluate() * b.evaluate(),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Char(ParseCharError),
    Other(String)
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl From<ParseCharError> for ParseError {
    fn from(e: ParseCharError) -> Self {
        ParseError::Char(e)
    }
}

impl FromStr for Expression<'_> {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut remaining = line.chars();
        let a: Expression = read_expression(&mut remaining).parse()?;
        match remaining.next() {
            Some(c) => {
                remaining.next(); // space
                match c {
                    '+' => Ok(Expression::Addition(a, remaining.as_str().parse()?)),
                    '*' => Ok(Expression::Multiplication(a, remaining.as_str().parse()?)),
                }
            },
            None => Ok(a)
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
        let expressions: Vec<Expression> = text.split(NEW_LINE).map(|s| s.parse()).collect();
        for expression in expressions {
            println!("{:?} = {}", expression, expression.evaluate());
        }
        let sum = expressions.iter().map(|exp| exp.evaluate()).sum();
        println!("Sum of results: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}