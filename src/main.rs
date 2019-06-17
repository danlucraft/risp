use std::io::{stdin, Write, stdout};

mod risp;
use risp::{parser, evaluator};

fn read_line() -> String {
    print!(">> ");
    stdout().flush().expect("couldn't flush stdout");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let line = read_line();
    let exp = parser::parse_expression(&mut line.chars().peekable()).unwrap();
    println!("{:?}", exp);
    let result = evaluator::eval(&exp);
    println!("{:?}", result);
}


