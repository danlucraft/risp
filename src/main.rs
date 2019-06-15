use std::io::{stdin, Write, stdout};

#[derive(Debug)]
enum Exp {
    Atom(String),
    List(Vec<Exp>)
}

fn read_line() -> String {
    print!(">> ");
    stdout().flush().expect("couldn't flush stdout");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}

fn parse_expression(line: &String) -> Result<Exp, String> {
    for ch in line.chars() {
        if ch == '(' {
            println!("Spotted list start");
        } else {
            return Err("Malformed expression".to_owned())
        }
    }
    Ok(Exp::Atom("asd".to_owned()))
}

fn main() {
    let line = read_line();
    println!("{}", line);
    let exp = parse_expression(&line);
    println!("{:?}", exp);
}


