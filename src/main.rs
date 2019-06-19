use std::io::{stdin, Write, stdout};

mod risp;
use risp::{parser, evaluator, to_string};
use risp::environment::Env;

fn read_line() -> String {
    print!(">> ");
    stdout().flush().expect("couldn't flush stdout");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let mut env = Env::new();
    loop {
        let line = read_line();
        let exp = parser::parse_expression(&mut line.chars().peekable()).unwrap();
        let result = evaluator::eval(&exp, &mut env);
        println!("{}", to_string::to_string(&result));
    }
}
