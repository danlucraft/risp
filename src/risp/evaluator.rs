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