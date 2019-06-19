use crate::risp::expressions::Exp;
use crate::risp::function::*;
use crate::risp::environment::Env;

pub fn eval<'a>(exp: &Exp, env: &mut Env) -> Exp {
    match exp {
        Exp::Int(_) => exp.clone(),
        Exp::Bool(_) => exp.clone(),
        Exp::Function(_) => exp.clone(),
        Exp::Atom(a) => match a.as_ref() {
            "quote" => Exp::BuiltIn(BuiltIn::Quote),
            "atom"  => Exp::BuiltIn(BuiltIn::Atom),
            "eq"    => Exp::BuiltIn(BuiltIn::Eq),
            "car"   => Exp::BuiltIn(BuiltIn::Car),
            "cdr"   => Exp::BuiltIn(BuiltIn::Cdr),
            "cons"  => Exp::BuiltIn(BuiltIn::Cons),
            "cond"  => Exp::BuiltIn(BuiltIn::Cond),
            "lambda" => Exp::BuiltIn(BuiltIn::Lambda),
            "def"    => Exp::BuiltIn(BuiltIn::Def),
            "label"  => Exp::BuiltIn(BuiltIn::Label),
            "prn"    => Exp::BuiltIn(BuiltIn::Inspect),
            _       => {
                if let Some(value) = env.get(a.to_string()) {
                    value
                } else {
                    panic!("Can't resolve atom '{}'", a)
                }
            }
        },
        Exp::BuiltIn(_) => panic!("Don't know how to evaluate a built in"),
        Exp::List(v) => {
            let first = eval(&v[0], env);
            match first {
                Exp::BuiltIn(builtin) => builtin.call(v[1..].to_vec(), env),
                Exp::Function(function) => function.call(v[1..].to_vec(), env),
                _ => panic!("Only know how to execute builtins currently")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;

    fn parse(code: &str) -> Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    #[test]
    fn eval_lambda() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Int(1),
                Exp::Int(10),
            )),
            eval(&parse("( (lambda () (cons 1 '(10))) 2)"), &mut Env::new())
        );
    }

    #[test]
    fn eval_lambda_with_args() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Int(2),
                Exp::Int(1),
            )),
            eval(&parse("( (lambda (x) (cons x '(1))) 2)"), &mut Env::new())
        );
    }

    #[test]
    fn eval_lambda_with_multiple_args() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Bool(true),
                Exp::Int(101),
                Exp::Atom("hi".to_owned()),
            )),
            eval(&parse("( (lambda (a b c) (cons (eq a b) (cons c '(hi)))) true true 101)"), &mut Env::new())
        );
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("z".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned()),
            )),
            eval(&parse("( (lambda (x y) (cons x (cdr y))) 'z '(a b c))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_lambda_with_lambda_parameter() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned()),
            )),
            eval(&parse("( (lambda (f) (f '(b c))) (lambda (x) (cons 'a x)))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_cond() {
        assert_eq!(
            Exp::Atom("b".to_owned()),
            eval(&parse("(cond (eq true false) 'a (eq false false) 'b)"), &mut Env::new())
        );

        assert_eq!(
            Exp::Atom("c".to_owned()),
            eval(&parse("(cond (eq true false) 'a (eq false true) 'b true 'c)"), &mut Env::new())
        );
    }

    #[test]
    fn eval_cons() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(cons 'a '(b c))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_car() {
        assert_eq!(
            Exp::Atom("a".to_owned()),
            eval(&parse("(car '(a b c))"), &mut Env::new())
        );
        assert_eq!(
            Exp::Int(1),
            eval(&parse("(car '(1 2 3))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(cdr '(a b c))"), &mut Env::new())
        );
        assert_eq!(
            Exp::List(vec!()),
            eval(&parse("(cdr '())"), &mut Env::new())
        );
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