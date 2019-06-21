use crate::risp::expressions::Exp;
use crate::risp::evaluator::eval;
use crate::risp::exceptions::{Exception, ExceptionType};
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
    Do,
    IsInt,
    IsBool,
    IsNil
}

impl Callable for BuiltIn {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Result<Exp, Exception> {
        match self {
            BuiltIn::IsInt => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::Int(_) = arg0 {
                    Ok(Exp::Bool(true))
                } else {
                    Ok(Exp::Bool(false))
                }
            },
            BuiltIn::IsBool => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::Bool(_) = arg0 {
                    Ok(Exp::Bool(true))
                } else {
                    Ok(Exp::Bool(false))
                }
            },
            BuiltIn::IsNil => {
                let arg0 = eval(&args[0], env)?;
                if arg0 == Exp::Nil {
                    Ok(Exp::Bool(true))
                } else {
                    Ok(Exp::Bool(false))
                }
            },
            BuiltIn::Do => {
                let mut result = Exp::Bool(true);
                for arg in args {
                    let arg_value = eval(&arg, env)?;
                    result = arg_value;
                }
                Ok(result)
            }
            BuiltIn::Assert => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::Bool(true) = arg0 {
                    Ok(Exp::Bool(true))
                } else {
                    Err(Exception { etype: ExceptionType::AssertionFailed, message: "Assertion failed".to_owned(), backtrace: vec!() })
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
                        Ok(function)
                    } else {
                        Err(Exception { etype: ExceptionType::ArgumentError, message: "Second arg to defun should be a list of atoms".to_owned(), backtrace: vec!() })
                    }
                } else {
                    Err(Exception { etype: ExceptionType::ArgumentError, message: "First arg to defun should be an atom".to_owned(), backtrace: vec!() })
                }
            }
            BuiltIn::Add => {
                let mut result: i32 = 0;
                for arg in args {
                    let arg_value = eval(&arg, env)?;
                    if let Exp::Int(i) = arg_value {
                        result += i;
                    } else {
                        panic!("+ expected an integer but got {:?}", to_string::to_string(&arg_value));
                    }
                }
                Ok(Exp::Int(result))
            },
            BuiltIn::Subtract => {
                let arg0 =  eval(&args[0], env)?;
                if let Exp::Int(i) = arg0 {
                    let mut result: i32 = i;
                    for arg in &args[1..] {
                        let arg_v = eval(&arg, env)?;
                        if let Exp::Int(i) = arg_v {
                            result -= i;
                        }
                    }
                    Ok(Exp::Int(result))
                } else {
                    panic!("- expected an integer");
                }
            },
            BuiltIn::Inspect => {
                let mut result = Exp::Nil;
                for arg in args {
                    result = eval(&arg, env)?;
                    println!("{}", to_string::to_string(&result));
                }
                Ok(result)
            },
            BuiltIn::Label => {
                if let Exp::Atom(name) = &args[0] {
                    let arg1 = eval(&args[1], env)?;
                    if let Exp::Function(mut function) = arg1 {
                        function.self_name = Some(name.to_owned());
                        Ok(Exp::Function(function))
                    } else {
                        panic!("Second arg to label must yield a function");
                    }
                } else {
                    panic!("First arg to label should be an atom");
                }
            }
            BuiltIn::Def => {
                if let Exp::Atom(name) = &args[0] {
                    let value = eval(&args[1], env)?;
                    env.set(name.clone(), value);
                    Ok(Exp::Bool(true))
                } else {
                    panic!("First arg to def should be an atom");
                }
            },
            BuiltIn::Lambda => {
                if let Exp::List(arg_list) = &args[0] {
                    Ok(Exp::Function(Function { arg_names: arg_list.to_vec(), body_exps: args[1..].to_vec(), self_name: None }))
                } else {
                    panic!("First arg to lambda should be arg list");
                }
            }
            BuiltIn::Quote => return Ok(args[0].clone()),
            BuiltIn::Atom => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::Atom(_) = arg0 {
                    Ok(Exp::Bool(true))
                } else {
                    Ok(Exp::Bool(false))
                }
            },
            BuiltIn::Cond => {
                for curr in 0..(args.len()/2) {
                    let arg_v = eval(&args[curr*2], env)?;
                    if arg_v == Exp::Bool(true) {
                        return eval(&args[curr*2 + 1], env);
                    }
                }
                Ok(Exp::List(vec!()))
            },
            BuiltIn::Cons => {
                let new_head = eval(&args[0], env)?;
                let list = eval(&args[1], env)?;
                if let Exp::List(lv) = list {
                    let mut new_vec = lv.clone();
                    new_vec.insert(0, new_head.clone());
                    Ok(Exp::List(new_vec))
                } else {
                    panic!("cons expected a list");
                }
            },
            BuiltIn::Car => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::List(v) = arg0 {
                    if v.len() > 0 {
                        Ok(v[0].clone())
                    } else {
                        Ok(Exp::List(vec!()))
                    }
                } else {
                    panic!("car expected a list")
                }
            },
            BuiltIn::Cdr => {
                let arg0 = eval(&args[0], env)?;
                if let Exp::List(vec) = arg0 {
                    if vec.len() > 1 {
                        Ok(Exp::List(vec[1..].to_vec()))
                    } else {
                        Ok(Exp::List(vec!()))
                    }
                } else {
                    panic!("cdr expected a list");
                }

            },
            BuiltIn::Eq => {
                if args.len() < 2 {
                    Ok(Exp::Bool(true))
                } else {
                    let first = eval(&args[0], env)?;
                    for arg in &args[1..] {
                        let r = eval(&arg, env)?;
                        if first != r {
                            return Ok(Exp::Bool(false));
                        }
                    }
                    Ok(Exp::Bool(true))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::to_string::display_result;
    use crate::risp::evaluator::eval_all;

    fn parse(code: &str) -> Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    fn run(code: &str) -> String {
        display_result(&eval(&parse(code), &mut Env::new()))
    }

    fn run_all(code: &str) -> String {
        let exps = parser::parse(code);
        let mut env = Env::new();
        let exp = eval_all(&exps, &mut env);
        display_result(&exp)
    }

    #[test]
    fn builtin_int() {
        assert_eq!( "true", run_all("(int? 123)") );
        assert_eq!( "false", run_all("(int? '())") );
    }

    #[test]
    fn builtin_bool() {
        assert_eq!( "true", run_all("(bool? true)") );
        assert_eq!( "false", run_all("(bool? 123)") );
    }

    #[test]
    fn builtin_nil() {
        assert_eq!( "true", run_all("(nil? nil)") );
        assert_eq!( "false", run_all("(nil? false)") );
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
        assert_eq!( "(1 10)", run("( (lambda () (cons 1 '(10))))") );
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
        assert_eq!( "()", run("(car '())") );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!( "(b c)", run("(cdr '(a b c))") );
        assert_eq!( "()", run("(cdr '())") );
    }

    #[test]
    fn eval_eq() {
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq 'abc 'abc)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq 'abc 'def)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq '(a b c) 'def)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq '() '())"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq true true)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq false false)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq true false)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq 12 12)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq 12 -12)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq 12)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)),  eval(&parse("(eq 12 12 12)"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq 12 12 1)"), &mut Env::new()));
    }

    #[test]
    fn eval_eq_works_with_nested_lists() {
        assert_eq!(Ok(Exp::Bool(true)), eval(&parse("(eq '(a b c) '(a b c))"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq '(a b c) '(a b d))"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(true)), eval(&parse("(eq '(a '(1 2 3) c) '(a '(1 2 3) c))"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(eq '(a '(1 2 3) c) '(a '(1 2 4) c))"), &mut Env::new()));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Ok(Exp::Bool(true)), eval(&parse("(atom 'abc))"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(atom '(a b c))"), &mut Env::new()));
        assert_eq!(Ok(Exp::Bool(false)), eval(&parse("(atom '()))"), &mut Env::new()));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Ok(Exp::Int(101)), eval(&parse("'101"), &mut Env::new()));
        assert_eq!(Ok(Exp::Atom("foo".to_owned())), eval(&parse("'foo"), &mut Env::new()));
        assert_eq!(
            Ok(Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            ))),
            eval(&parse("'(a b c)"), &mut Env::new())
        );
    }
}