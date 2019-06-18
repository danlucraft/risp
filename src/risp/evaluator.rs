use crate::risp::parser;
use regex::Regex;

#[derive(Debug,PartialEq,Eq)]
pub enum Value {
    Exp(parser::Exp),
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
                    "quote" => return Value::Exp(v[1].clone()),
                    "atom" => {
                        if let Value::Exp(parser::Exp::Atom(_)) = eval(&v[1]) {
                            return Value::Bool(true)
                        } else {
                            return Value::Bool(false)
                        }
                    },
                    "car" => {
                        if let Value::Exp(list) = eval(&v[1]) {
                            if let parser::Exp::List(v) = &list {
                                return Value::Exp(v[0].clone());
                            }
                        } else {
                            panic!("car expected a list");
                        }
                    }
                    "cdr" => {
                        if let Value::Exp(list) = eval(&v[1]) {
                            if let parser::Exp::List(v) = &list {
                                if v.len() > 1 {
                                    return Value::Exp(parser::Exp::List(v[1..].to_vec()));
                                } else {
                                    return Value::Exp(parser::Exp::List(vec!()));
                                }
                            }
                        } else {
                            panic!("car expected a list");
                        }

                    }
                    "eq" => {
                        if let Value::Exp(x_atom) = eval(&v[1]) {
                            if let parser::Exp::Atom(x) = x_atom {
                                if let Value::Exp(y_atom) = eval(&v[2]) {
                                    if let parser::Exp::Atom(y) = y_atom {
                                        if x == y {
                                            return Value::Bool(true);
                                        }
                                    }
                                }
                            }
                        }

                        if let Value::Exp(x_list) = eval(&v[1]) {
                            if let parser::Exp::List(x) = x_list {
                                if let Value::Exp(y_list) = eval(&v[2]) {
                                    if let parser::Exp::List(y) = y_list {
                                        if x.len() == 0 && y.len() == 0 {
                                            return Value::Bool(true);
                                        }
                                    }
                                }
                            }
                        }

                        if let Value::Bool(xb) = eval(&v[1]) {
                            if let Value::Bool(yb) = eval(&v[2]) {
                                if xb == yb {
                                    return Value::Bool(true);
                                }
                            }
                        }
                        return Value::Bool(false);
                    }
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
    fn eval_car() {
        assert_eq!(
            Value::Exp(parser::Exp::Atom("a".to_owned())),
            eval(&parse("(car (quote (a b c)))"))
        );
        assert_eq!(
            Value::Exp(parser::Exp::Atom("1".to_owned())),
            eval(&parse("(car (quote (1 2 3)))"))
        );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!(
            Value::Exp(
                parser::Exp::List(vec!(
                    parser::Exp::Atom("b".to_owned()),
                    parser::Exp::Atom("c".to_owned())
                ))
            ),
            eval(&parse("(cdr (quote (a b c)))"))
        );
        assert_eq!(
            Value::Exp(
                parser::Exp::List(vec!())
            ),
            eval(&parse("(cdr (quote ()))"))
        );
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
    fn eval_eq() {
        assert_eq!(Value::Bool(true), eval(&parse("(eq (quote abc) (quote abc))")));
        assert_eq!(Value::Bool(false), eval(&parse("(eq (quote abc) (quote def))")));
        assert_eq!(Value::Bool(false), eval(&parse("(eq (quote (a b c)) (quote def))")));
        assert_eq!(Value::Bool(true), eval(&parse("(eq (quote ()) (quote ()))")));
        assert_eq!(Value::Bool(true), eval(&parse("(eq true true)")));
        assert_eq!(Value::Bool(true), eval(&parse("(eq false false)")));
        assert_eq!(Value::Bool(false), eval(&parse("(eq true false)")));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Value::Bool(true), eval(&parse("(atom (quote abc))")));
        assert_eq!(Value::Bool(false), eval(&parse("(atom (quote (quote a b c)))")));
        assert_eq!(Value::Bool(false), eval(&parse("(atom (quote ()))")));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Value::Exp(parser::Exp::Atom("101".to_owned())), eval(&parse("(quote 101)")));
        assert_eq!(Value::Exp(parser::Exp::Atom("foo".to_owned())), eval(&parse("(quote foo)")));
        assert_eq!(Value::Exp(
            parser::Exp::List(vec!(
                parser::Exp::Atom("a".to_owned()),
                parser::Exp::Atom("b".to_owned()),
                parser::Exp::Atom("c".to_owned())
            ))),
            eval(&parse("(quote (a b c))"))
        );
    }
}