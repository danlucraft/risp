use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Exp {
    Atom(String),
    List(Vec<Exp>)
}

fn consume_whitespace(chars: &mut Peekable<Chars>) -> i32 {
    let mut c = 0;
    while chars.peek() == Some(&' ') {
        chars.next();
        c += 1;
    }
    c
}

fn parse_atom(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    let mut s = String::new();
    consume_whitespace(chars);
    let mut ch: Option<char> = chars.peek().cloned();
    while ch.is_some() && ch != Some(')') && ch != Some(' ') {
        s.push(ch.unwrap());
        chars.next();
        ch = chars.peek().cloned();
    }
    Ok(Exp::Atom(s))
}

fn parse_inner_list(chars: &mut Peekable<Chars>) -> Result<Vec<Exp>, String> {
    let mut v: Vec<Exp> = vec!();
    loop {
        consume_whitespace(chars);
        let ch: Option<&char> = chars.peek();
        if ch == Some(&')') {
            return Ok(v)
        } else {
            match parse_expression(chars) {
                Ok(exp) => v.push(exp),
                Err(_) => return Ok(v)
            }
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    consume_whitespace(chars);
    let ch: Option<&char> = chars.peek();
    if ch == Some(&'(') {
        chars.next();
        let v = parse_inner_list(chars).unwrap();
        if chars.peek() == Some(&')') {
            chars.next();
            return Ok(Exp::List(v));
        } else {
            return Err("Expected )".to_owned())
        }
    } else {
        return Err("Expected (".to_owned())
    }
}

pub fn parse_expression(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    consume_whitespace(chars);
    let ch: Option<&char> = chars.peek();
    if ch == Some(&'(') {
        return parse_list(chars);
    } else {
        return parse_atom(chars)
    }
}
