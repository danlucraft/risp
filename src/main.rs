use std::io::{stdin, Write, stdout};

fn read_line() -> String {
    print!(">> ");
    stdout().flush().expect("couldn't flush stdout");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let line = read_line();
    println!("{}", line)
}
