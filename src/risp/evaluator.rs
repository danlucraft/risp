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
            _       => {
                if let Some(value) = env.get(a.to_string()) {
                    value
                } else {
                    panic!("Can't resolve {}", a)
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
            eval(&parse("( (lambda () (cons 1 (quote (10)))) 2)"), &mut Env::new())
        );
    }

    #[test]
    fn eval_lambda_with_args() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Int(2),
                Exp::Int(1),
            )),
            eval(&parse("( (lambda (x) (cons x (quote (1)))) 2)"), &mut Env::new())
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
            eval(&parse("( (lambda (a b c) (cons (eq a b) (cons c (quote (hi))))) true true 101)"), &mut Env::new())
        );
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("z".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned()),
            )),
            eval(&parse("( (lambda (x y) (cons x (cdr y))) (quote z) (quote (a b c)))"), &mut Env::new())
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
            eval(&parse("( (lambda (f) (f (quote (b c)))) (lambda (x) (cons (quote a) x)))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_cond() {
        assert_eq!(
            Exp::Atom("b".to_owned()),
            eval(&parse("(cond (eq true false) (quote a) (eq false false) (quote b))"), &mut Env::new())
        );

        assert_eq!(
            Exp::Atom("c".to_owned()),
            eval(&parse("(cond (eq true false) (quote a) (eq false true) (quote b) true (quote c))"), &mut Env::new())
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
            eval(&parse("(cons (quote a) (quote (b c)))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_car() {
        assert_eq!(
            Exp::Atom("a".to_owned()),
            eval(&parse("(car (quote (a b c)))"), &mut Env::new())
        );
        assert_eq!(
            Exp::Int(1),
            eval(&parse("(car (quote (1 2 3)))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_cdr() {
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(cdr (quote (a b c)))"), &mut Env::new())
        );
        assert_eq!(
            Exp::List(vec!()),
            eval(&parse("(cdr (quote ()))"), &mut Env::new())
        );
    }

    #[test]
    fn eval_eq() {
        assert_eq!(Exp::Bool(true), eval(&parse("(eq (quote abc) (quote abc))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq (quote abc) (quote def))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq (quote (a b c)) (quote def))"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq (quote ()) (quote ()))"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq true true)"), &mut Env::new()));
        assert_eq!(Exp::Bool(true), eval(&parse("(eq false false)"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(eq true false)"), &mut Env::new()));
    }

    #[test]
    fn eval_atom() {
        assert_eq!(Exp::Bool(true), eval(&parse("(atom (quote abc))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom (quote (quote a b c)))"), &mut Env::new()));
        assert_eq!(Exp::Bool(false), eval(&parse("(atom (quote ()))"), &mut Env::new()));
    }

    #[test]
    fn eval_quote() {
        assert_eq!(Exp::Int(101), eval(&parse("(quote 101)"), &mut Env::new()));
        assert_eq!(Exp::Atom("foo".to_owned()), eval(&parse("(quote foo)"), &mut Env::new()));
        assert_eq!(
            Exp::List(vec!(
                Exp::Atom("a".to_owned()),
                Exp::Atom("b".to_owned()),
                Exp::Atom("c".to_owned())
            )),
            eval(&parse("(quote (a b c))"), &mut Env::new())
        );
    }
}