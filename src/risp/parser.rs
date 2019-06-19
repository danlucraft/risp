use std::str::Chars;
use std::iter::Peekable;
use crate::risp::expressions::Exp;
use regex::Regex;

fn consume_whitespace(chars: &mut Peekable<Chars>) -> i32 {
    let mut c = 0;
    while chars.peek() == Some(&' ') || chars.peek() == Some(&'\n') {
        chars.next();
        c += 1;
    }
    c
}

fn is_identifier_character(ch: char) -> bool {
    return ch != ')' && ch != '(' && ch != ' ' && ch != '\n'
}

fn parse_token(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    let mut s = String::new();
    consume_whitespace(chars);
    let mut ch: Option<char> = chars.peek().cloned();
    while ch.is_some() && is_identifier_character(ch.unwrap()) {
        s.push(ch.unwrap());
        chars.next();
        ch = chars.peek().cloned();
    }
    if s.len() == 0 {
        Err("No token found".to_owned())
    } else {
        let int_literal_re = Regex::new(r"\A[0-9]+\z").unwrap();

        if int_literal_re.is_match(&s) {
            return Ok(Exp::Int(i32::from_str_radix(&s, 10).unwrap()));
        }
        if s == "true" {
            return Ok(Exp::Bool(true));
        }
        if s == "false" {
            return Ok(Exp::Bool(false));
        }
        Ok(Exp::Atom(s))
    }
}

fn parse_inner_list(chars: &mut Peekable<Chars>) -> Result<Vec<Exp>, String> {
    let mut v: Vec<Exp> = vec!();
    loop {
        consume_whitespace(chars);
        match chars.peek() {
            Some(&')') => return Ok(v),
            _ => {
                match parse_expression(chars) {
                    Ok(exp) => v.push(exp),
                    Err(_) => return Ok(v)
                }
            }
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    consume_whitespace(chars);
    match chars.peek() {
        Some(&'(') => {
            chars.next();
            let v = parse_inner_list(chars).unwrap();
            match chars.peek() {
                Some(&')') => {
                    chars.next();
                    Ok(Exp::List(v))
                },
                _ => Err("Expected )".to_owned())
            }
        },
        _ => Err("Expected (".to_owned())
    }
}

pub fn parse_expression(chars: &mut Peekable<Chars>) -> Result<Exp, String> {
    consume_whitespace(chars);
    match chars.peek() {
        Some(&'(') => parse_list(chars),
        _ => parse_token(chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_expressions() {
        assert_eq!(Ok(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::List(vec!(
                    Exp::Atom("+".to_owned()),
                    Exp::Int(1),
                    Exp::Int(2)
                )),
                Exp::Bool(true)
            ))), parse_list(&mut "(a (+ 1 2) true)".chars().peekable()));

        assert_eq!(Ok(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("c".to_owned())
            ))), parse_list(&mut " (  a  c )".chars().peekable()));
    }

    #[test]
    fn parsing_lists() {
        assert_eq!(Ok(Exp::List(vec!())), parse_list(&mut "()".chars().peekable()));
        assert_eq!(Ok(Exp::List(vec!())), parse_list(&mut "  (  )   ".chars().peekable()));
        assert_eq!(Ok(Exp::List(vec!(Exp::Atom("a".to_owned())))), parse_list(&mut "(a)".chars().peekable()));
        assert_eq!(Ok(Exp::List(vec!(Exp::Atom("a".to_owned())))), parse_list(&mut " \n (a\n)".chars().peekable()));
        assert_eq!(Ok(Exp::List(vec!(Exp::Atom("a".to_owned()), Exp::Atom("b".to_owned()), Exp::Atom("c".to_owned())))), parse_list(&mut "(a b c)".chars().peekable()));
    }

    #[test]
    fn parsing_lists_error() {
        assert_eq!(Err("Expected )".to_owned()), parse_list(&mut "(".chars().peekable()));
        assert_eq!(Err("Expected )".to_owned()), parse_list(&mut "(()".chars().peekable()));
        assert_eq!(Err("Expected (".to_owned()), parse_list(&mut ")".chars().peekable()));
    }

    #[test]
    fn parse_literals() {
        assert_eq!(Exp::Int(101), parse_expression(&mut "101".chars().peekable()).unwrap());
        assert_eq!(Exp::Int(1011), parse_expression(&mut "1011".chars().peekable()).unwrap());
        assert_eq!(Exp::Int(99), parse_expression(&mut "99".chars().peekable()).unwrap());
        assert_eq!(Exp::Bool(true), parse_expression(&mut "true".chars().peekable()).unwrap());
        assert_eq!(Exp::Bool(false), parse_expression(&mut "false".chars().peekable()).unwrap());
    }

    #[test]
    fn parsing_atoms() {
        assert_eq!(Ok(Exp::Atom("hello".to_owned())), parse_token(&mut "hello".chars().peekable()));
        assert_eq!(Err("No token found".to_owned()), parse_token(&mut "".chars().peekable()));
        assert_eq!(Ok(Exp::Atom("hello".to_owned())), parse_token(&mut "hello world".chars().peekable()));
        assert_eq!(Ok(Exp::Atom("+".to_owned())), parse_token(&mut "+ 1 2".chars().peekable()));
        assert_eq!(Ok(Exp::Atom("hello".to_owned())), parse_token(&mut "  hello".chars().peekable()));
        assert_eq!(Ok(Exp::Atom("hi".to_owned())), parse_token(&mut "  hi(ho".chars().peekable()));
        assert_eq!(Ok(Exp::Atom("hi".to_owned())), parse_token(&mut "  hi\n".chars().peekable()));
    }
}
