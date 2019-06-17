use crate::risp::parser;
use regex::Regex;

#[derive(Debug,PartialEq,Eq)]
pub enum Value<'a> {
    Exp(&'a parser::Exp),
    Int(i32),
    Bool(bool)
}

pub fn eval<'a>(exp: &parser::Exp) -> Value {
    let int_literal_re = Regex::new(r"\A[0-9]+\z").unwrap();
    match exp {
        parser::Exp::List(v) => {
            let first = &v[0];
            if let parser::Exp::Atom(a) = first {
                match a.as_ref() {
                    "quote" => return Value::Exp(&v[1]),
                    "atom" => {
                        if let Value::Exp(parser::Exp::Atom(_)) = eval(&v[1]) {
                            return Value::Bool(true)
                        } else {
                            return Value::Bool(false)
                        }
                    },
                    _ => {
                        println!("Don't know how yet");
                    }
                }
            } else {
                println!("Not quote")
            }
        },
        parser::Exp::Atom(a) => {
            if int_literal_re.is_match(a) {
                return Value::Int(i32::from_str_radix(a, 10).unwrap())

            }
            if a == "true" {
                return Value::Bool(true)
            }
            if a == "false" {
                return Value::Bool(false)
            }
        }
    }
    return Value::Int(101);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(code: &str) -> parser::Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    #[test]
    fn eval_literals() {
        assert_eq!(Value::Int(101), eval(&parse("101")));
        assert_eq!(Value::Int(1011), eval(&parse("1011")));
        assert_eq!(Value::Int(99), eval(&parse("99")));
        assert_eq!(Value::Bool(true), eval(&parse("true")));
        assert_eq!(Value::Bool(false), eval(&parse("false")));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Value::Bool(true), eval(&parse("(atom (quote abc))")));
        assert_eq!(Value::Bool(false), eval(&parse("(atom (quote (quote a b c)))")));
        assert_eq!(Value::Bool(false), eval(&parse("(atom (quote ()))")));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Value::Exp(&parser::Exp::Atom("101".to_owned())), eval(&parse("(quote 101)")));
        assert_eq!(Value::Exp(&parser::Exp::Atom("foo".to_owned())), eval(&parse("(quote foo)")));
        assert_eq!(Value::Exp(
            &parser::Exp::List(vec!(
                parser::Exp::Atom("a".to_owned()),
                parser::Exp::Atom("b".to_owned()),
                parser::Exp::Atom("c".to_owned())
            ))),
            eval(&parse("(quote (a b c))"))
        );
    }
}