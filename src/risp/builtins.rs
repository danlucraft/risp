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
    Add,
    Subtract,
    Defun,
    Assert,
    Do
}

impl Callable for BuiltIn {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Exp {
        match self {
            BuiltIn::Do => {
                let mut result = Exp::Bool(true);
                for arg in args {
                    result = eval(&arg, env);
                }
                result
            }
            BuiltIn::Assert => {
                if let Exp::Bool(true) = eval(&args[0], env) {
                    return Exp::Bool(true);
                } else {
                    panic!("Assertion failed");
                }
            },
            BuiltIn::Defun => {
                if let Exp::Atom(name) = &args[0] {
                    if let Exp::List(arg_list) = &args[1] {
                        let function = Exp::Function(Function { 
                            arg_names: arg_list.to_vec(), 
                            body_exps: args[2..].to_vec(), 
                            self_name: Some(name.to_string())
                        });
                        env.set(name.to_string(), function.clone());
                        function
                    } else {
                        panic!("Second arg to defun should be a list of atoms");
                    }
                } else {
                    panic!("First arg to defun should be an atom");
                }
            }
            BuiltIn::Add => {
                let mut result: i32 = 0;
                for arg in args {
                    if let Exp::Int(i) = eval(&arg, env) {
                        result += i;
                    } else {
                        panic!("+ expected an integer");
                    }
                }
                Exp::Int(result)
            },
            BuiltIn::Subtract => {
                if let Exp::Int(i) = eval(&args[0], env) {
                    let mut result: i32 = i;
                    for arg in &args[1..] {
                        if let Exp::Int(i) = eval(&arg, env) {
                            result -= i;
                        }
                    }
                    Exp::Int(result)
                } else {
                    panic!("- expected an integer");
                }
            },
            BuiltIn::Inspect => {
                for arg in args {
                    println!("{}", to_string::to_string(&eval(&arg, env)));
                }
                Exp::Bool(true)
            },
            BuiltIn::Label => {
                if let Exp::Atom(name) = &args[0] {
                    if let Exp::Function(mut function) = eval(&args[1], env) {
                        function.self_name = Some(name.to_owned());
                        Exp::Function(function)
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
                    Exp::Bool(true)
                } else {
                    panic!("First arg to def should be an atom");
                }
            },
            BuiltIn::Lambda => {
                if let Exp::List(arg_list) = &args[0] {
                    Exp::Function(Function { arg_names: arg_list.to_vec(), body_exps: args[1..].to_vec(), self_name: None })
                } else {
                    panic!("First arg to lambda should be arg list");
                }
            }
            BuiltIn::Quote => return args[0].clone(),
            BuiltIn::Atom => {
                if let Exp::Atom(_) = eval(&args[0], env) {
                    Exp::Bool(true)
                } else {
                    Exp::Bool(false)
                }
            },
            BuiltIn::Cond => {
                for curr in 0..(args.len()/2) {
                    if eval(&args[curr*2], env) == Exp::Bool(true) {
                        return eval(&args[curr*2 + 1], env);
                    }
                }
                Exp::List(vec!())
            },
            BuiltIn::Cons => {
                let new_head = eval(&args[0], env);
                if let Exp::List(lv) = eval(&args[1], env) {
                    let mut new_vec = lv.clone();
                    new_vec.insert(0, new_head.clone());
                    Exp::List(new_vec)
                } else {
                    panic!("cons expected a list");
                }
            },
            BuiltIn::Car => {
                if let Exp::List(v) = eval(&args[0], env) {
                    if v.len() > 0 {
                        v[0].clone()
                    } else {
                        Exp::Nil
                    }
                } else {
                    panic!("car expected a list")
                }
            },
            BuiltIn::Cdr => {
                if let Exp::List(vec) = eval(&args[0], env) {
                    if vec.len() > 1 {
                        Exp::List(vec[1..].to_vec())
                    } else {
                        Exp::List(vec!())
                    }
                } else {
                    panic!("cdr expected a list");
                }

            },
            BuiltIn::Eq => {
                let l = eval(&args[0], env);
                let r = eval(&args[1], env);
                Exp::Bool(l == r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::to_string::to_string;
    use crate::risp::evaluator::eval_all;

    fn parse(code: &str) -> Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    fn run(code: &str) -> String {
        to_string(&eval(&parse(code), &mut Env::new()))
    }

    fn run_all(code: &str) -> String {
        let exps = parser::parse(code);
        let mut env = Env::new();
        let exp = eval_all(&exps, &mut env);
        to_string(&exp)
    }

    #[test]
    fn builtin_do() {
        assert_eq!( "123", run_all("(do (def foo 123) foo)") );
    }

    #[test]
    fn builtin_assert() {
        assert_eq!( "true", run_all("(assert! true)") );
    }

    #[test]
    fn builtin_defun() {
        assert_eq!( "404", run_all("(defun add4 (x) (+ x 4)) (add4 400)") );
    }

    #[test]
    fn eval_addition_subtraction() {
        assert_eq!( "5", run("(+ 1 4)") );
        assert_eq!( "7", run("(- 11 4)") );
        assert_eq!( "112", run("(+ 1 4 7 100)") );
        assert_eq!( "-3", run("(- 11 4 10)") );
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
    fn eval_car_empty_list() {
        assert_eq!( "nil", run("(car '())") );
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
        assert_eq!(Exp::Bool(true), eval(&parse("(eq 12 12)"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq 12 -12)"), &mut Env::new()));
    }

    #[test]
    fn eval_eq_works_with_nested_lists() {
        assert_eq!(Exp::Bool(true), eval(&parse("(eq '(a b c) '(a b c))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq '(a b c) '(a b d))"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq '(a '(1 2 3) c) '(a '(1 2 3) c))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq '(a '(1 2 3) c) '(a '(1 2 4) c))"), &mut Env::new()));
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