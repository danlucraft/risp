use crate::risp::expressions::Exp;
use crate::risp::evaluator::eval;
use crate::risp::environment::Env;
use crate::risp::to_string;
use crate::risp::function::{Callable, Function};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltIn {
    Quote,
    Atom,
    Eq,
    Car,
    Cdr,
    Cons,
    Cond,
    Lambda,
    Def,
    Label,
    Inspect,
}

impl Callable for BuiltIn {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Exp {
        match self {
            BuiltIn::Inspect => {
                for arg in args {
                    println!("{}", to_string::to_string(&eval(&arg, env)));
                }
                return Exp::Bool(true)
            },
            BuiltIn::Label => {
                if let Exp::Atom(name) = &args[0] {
                    if let Exp::Function(mut function) = eval(&args[1], env) {
                        function.self_name = Some(name.to_owned());
                        return Exp::Function(function);
                    } else {
                        panic!("Second arg to label must yield a function");
                    }
                } else {
                    panic!("First arg to label should be an atom");
                }
            }
            BuiltIn::Def => {
                if let Exp::Atom(name) = &args[0] {
                    let value = eval(&args[1], env);
                    env.set(name.clone(), value);
                    return Exp::Bool(true);
                } else {
                    panic!("First arg to def should be an atom");
                }
            },
            BuiltIn::Lambda => {
                if let Exp::List(arg_list) = &args[0] {
                    return Exp::Function(Function { arg_names: arg_list.to_vec(), body_exps: args[1..].to_vec(), self_name: None })
                } else {
                    panic!("First arg to lambda should be arg list");
                }
            }
            BuiltIn::Quote => return args[0].clone(),
            BuiltIn::Atom => {
                if let Exp::Atom(_) = eval(&args[0], env) {
                    return Exp::Bool(true)
                } else {
                    return Exp::Bool(false)
                }
            },
            BuiltIn::Cond => {
                for curr in 0..(args.len()/2) {
                    if eval(&args[curr*2], env) == Exp::Bool(true) {
                        return eval(&args[curr*2 + 1], env);
                    }
                }
                return Exp::List(vec!())
            },
            BuiltIn::Cons => {
                let new_head = eval(&args[0], env);
                if let Exp::List(lv) = eval(&args[1], env) {
                    let mut new_vec = lv.clone();
                    new_vec.insert(0, new_head.clone());
                    return Exp::List(new_vec);
                } else {
                    panic!("cons expected a list");
                }
            },
            BuiltIn::Car => {
                if let Exp::List(v) = eval(&args[0], env) {
                    return v[0].clone();
                } else {
                    panic!("car expected a list");
                }
            },
            BuiltIn::Cdr => {
                if let Exp::List(vec) = eval(&args[0], env) {
                    if vec.len() > 1 {
                        return Exp::List(vec[1..].to_vec());
                    } else {
                        return Exp::List(vec!());
                    }
                } else {
                    panic!("cdr expected a list");
                }

            }
            BuiltIn::Eq => {
                let l = eval(&args[0], env);
                let r = eval(&args[1], env);
                if let Exp::Atom(x) = l {
                    if let Exp::Atom(y) = r {
                        if x == y {
                            return Exp::Bool(true);
                        }
                    }
                } else if let Exp::List(x) = eval(&args[0], env) {
                    if let Exp::List(y) = eval(&args[1], env) {
                        if x.len() == 0 && y.len() == 0 {
                            return Exp::Bool(true);
                        }
                    }
                } else if let Exp::Bool(xb) = eval(&args[0], env) {
                    if let Exp::Bool(yb) = eval(&args[1], env) {
                        if xb == yb {
                            return Exp::Bool(true);
                        }
                    }
                }
                return Exp::Bool(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::to_string::to_string;

    fn parse(code: &str) -> Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    fn run(code: &str) -> String {
        to_string(&eval(&parse(code), &mut Env::new()))
    }

    #[test]
    fn eval_lambda() {
        assert_eq!( "(1 10)", run("( (lambda () (cons 1 '(10))) 2)") );
    }

    #[test]
    fn eval_lambda_with_args() {
        assert_eq!( "(2 1)", run("( (lambda (x) (cons x '(1))) 2)") );
    }

    #[test]
    fn eval_lambda_with_multiple_args() {
        assert_eq!( "(true 101 hi)", run("( (lambda (a b c) (cons (eq a b) (cons c '(hi)))) true true 101)") );
        assert_eq!( "(z b c)", run("( (lambda (x y) (cons x (cdr y))) 'z '(a b c))") );
    }

    #[test]
    fn eval_lambda_with_lambda_parameter() {
        assert_eq!( "(a b c)", run("( (lambda (f) (f '(b c))) (lambda (x) (cons 'a x)))") );
    }

    #[test]
    fn eval_cond() {
        assert_eq!( "b", run("(cond (eq true false) 'a (eq false false) 'b)") );
        assert_eq!( "c", run("(cond (eq true false) 'a (eq false true) 'b true 'c)") );
    }

    #[test]
    fn eval_cons() {
        assert_eq!( "(a b c)", run("(cons 'a '(b c))") );
    }

    #[test]
    fn eval_car() {
        assert_eq!( "a", run("(car '(a b c))") );
        assert_eq!( "1", run("(car '(1 2 3))") );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!( "(b c)", run("(cdr '(a b c))") );
        assert_eq!( "()", run("(cdr '())") );
    }

    #[test]
    fn eval_eq() {
        assert_eq!(Exp::Bool(true), eval(&parse("(eq 'abc 'abc)"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq 'abc 'def)"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq '(a b c) 'def)"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq '() '())"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq true true)"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq false false)"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq true false)"), &mut Env::new()));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Exp::Bool(true), eval(&parse("(atom 'abc))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom '(a b c))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom '()))"), &mut Env::new()));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Exp::Int(101), eval(&parse("'101"), &mut Env::new()));
        assert_eq!(Exp::Atom("foo".to_owned()), eval(&parse("'foo"), &mut Env::new()));
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("'(a b c)"), &mut Env::new())
        );
    }
}