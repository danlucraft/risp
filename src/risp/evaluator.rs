use crate::risp::parser;
use crate::risp::expressions::Exp;

pub fn eval<'a>(exp: &Exp) -> Exp {
    match exp {
        Exp::Int(_) => exp.clone(),
        Exp::Bool(_) => exp.clone(),
        Exp::Atom(_) => panic!("Don't know how to resolve an atom yet"),
        Exp::List(v) => {
            let first = &v[0];
            if let Exp::Atom(a) = first {
                match a.as_ref() {
                    "quote" => return v[1].clone(),
                    "atom" => {
                        if let Exp::Atom(_) = eval(&v[1]) {
                            return Exp::Bool(true)
                        } else {
                            return Exp::Bool(false)
                        }
                    },
                    "cons" => {
                        if let Exp::List(lv) = eval(&v[2]) {
                            let mut new_vec = lv.clone();
                            let new_head = eval(&v[1]);
                            new_vec.insert(0, new_head);
                            return Exp::List(new_vec);
                        } else {
                            panic!("cons expected a list");
                        }
                    },
                    "car" => {
                        if let Exp::List(v) = eval(&v[1]) {
                            return v[0].clone();
                        } else {
                            panic!("car expected a list");
                        }
                    },
                    "cdr" => {
                        if let Exp::List(vec) = eval(&v[1]) {
                            if vec.len() > 1 {
                                return Exp::List(vec[1..].to_vec());
                            } else {
                                return Exp::List(vec!());
                            }
                        } else {
                            panic!("car expected a list");
                        }

                    }
                    "eq" => {
                        if let Exp::Atom(x) = eval(&v[1]) {
                            if let Exp::Atom(y) = eval(&v[2]) {
                                if x == y {
                                    return Exp::Bool(true);
                                }
                            }
                        }

                        if let Exp::List(x) = eval(&v[1]) {
                            if let Exp::List(y) = eval(&v[2]) {
                                if x.len() == 0 && y.len() == 0 {
                                    return Exp::Bool(true);
                                }
                            }
                        }

                        if let Exp::Bool(xb) = eval(&v[1]) {
                            if let Exp::Bool(yb) = eval(&v[2]) {
                                if xb == yb {
                                    return Exp::Bool(true);
                                }
                            }
                        }
                        return Exp::Bool(false);
                    }
                    _ => {
                        panic!("Don't know how yet");
                    }
                }
            } else {
                panic!("Not quote")
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(code: &str) -> Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    #[test]
    fn eval_cons() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(cons (quote a) (quote (b c)))"))
        );
    }

    #[test]
    fn eval_car() {
        assert_eq!(
            Exp::Atom("a".to_owned()),
            eval(&parse("(car (quote (a b c)))"))
        );
        assert_eq!(
            Exp::Int(1),
            eval(&parse("(car (quote (1 2 3)))"))
        );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(cdr (quote (a b c)))"))
        );
        assert_eq!(
            Exp::List(vec!()),
            eval(&parse("(cdr (quote ()))"))
        );
    }

    #[test]
    fn eval_eq() {
        assert_eq!(Exp::Bool(true), eval(&parse("(eq (quote abc) (quote abc))")));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq (quote abc) (quote def))")));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq (quote (a b c)) (quote def))")));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq (quote ()) (quote ()))")));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq true true)")));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq false false)")));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq true false)")));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Exp::Bool(true), eval(&parse("(atom (quote abc))")));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom (quote (quote a b c)))")));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom (quote ()))")));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Exp::Int(101), eval(&parse("(quote 101)")));
        assert_eq!(Exp::Atom("foo".to_owned()), eval(&parse("(quote foo)")));
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(quote (a b c))"))
        );
    }
}