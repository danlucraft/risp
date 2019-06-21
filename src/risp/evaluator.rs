use crate::risp::expressions::Exp;
use crate::risp::function::*;
use crate::risp::environment::Env;
use crate::risp::builtins::BuiltIn;
use crate::risp::exceptions::Exception;

pub fn eval_all(exps: &Vec<Exp>, env: &mut Env) -> Result<Exp, Exception> {
    let mut value_result = Ok(Exp::Bool(true));
    for exp in exps {
        match eval(exp, env) {
            Ok(exp) => { value_result = Ok(exp) },
            Err(exc) => { return Err(exc) }
        }
    }
    value_result
}

pub fn eval<'a>(exp: &Exp, env: &mut Env) -> Result<Exp, Exception> {
    match exp {
        Exp::Atom(a) => match a.as_ref() {
            "quote"   => Ok(Exp::BuiltIn(BuiltIn::Quote)),
            "atom"    => Ok(Exp::BuiltIn(BuiltIn::Atom)),
            "eq"      => Ok(Exp::BuiltIn(BuiltIn::Eq)),
            "car"     => Ok(Exp::BuiltIn(BuiltIn::Car)),
            "cdr"     => Ok(Exp::BuiltIn(BuiltIn::Cdr)),
            "cons"    => Ok(Exp::BuiltIn(BuiltIn::Cons)),
            "cond"    => Ok(Exp::BuiltIn(BuiltIn::Cond)),
            "lambda"  => Ok(Exp::BuiltIn(BuiltIn::Lambda)),
            "def"     => Ok(Exp::BuiltIn(BuiltIn::Def)),
            "label"   => Ok(Exp::BuiltIn(BuiltIn::Label)),
            "prn"     => Ok(Exp::BuiltIn(BuiltIn::Inspect)),
            "+"       => Ok(Exp::BuiltIn(BuiltIn::Add)),
            "-"       => Ok(Exp::BuiltIn(BuiltIn::Subtract)),
            "defun"   => Ok(Exp::BuiltIn(BuiltIn::Defun)),
            "assert!" => Ok(Exp::BuiltIn(BuiltIn::Assert)),
            "do"      => Ok(Exp::BuiltIn(BuiltIn::Do)),
            "int?"    => Ok(Exp::BuiltIn(BuiltIn::IsInt)),
            "bool?"   => Ok(Exp::BuiltIn(BuiltIn::IsBool)),
            "nil?"    => Ok(Exp::BuiltIn(BuiltIn::IsNil)),
            _       => {
                if let Some(value) = env.get(a.to_string()) {
                    Ok(value)
                } else {
                    Err(Exception::UnknownSymbol(a.to_string()))
                }
            }
        },
        Exp::List(v) => {
            let first = eval(&v[0], env);
            match first {
                Ok(first_exp) =>
                    match first_exp {
                        Exp::BuiltIn(builtin) => builtin.call(v[1..].to_vec(), env),
                        Exp::Function(function) => function.call(v[1..].to_vec(), env),
                        Exp::Atom(a) => Err(Exception::UncallableCalled(a.to_string())),
                        Exp::Int(a)  => Err(Exception::UncallableCalled(a.to_string())),
                        Exp::Bool(a) => Err(Exception::UncallableCalled(a.to_string())),
                        Exp::Nil     => Err(Exception::UncallableCalled("nil".to_owned())),
                        _            => Err(Exception::UncallableCalled("unknown".to_owned()))
                    },
                Err(_) => {
                    first
                }
            }
        },
        _ => Ok(exp.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::to_string::display_result;

    fn run_all(code: &str) -> String {
        let exps = parser::parse(code);
        let mut env = Env::new();
        let exp = eval_all(&exps, &mut env);
        display_result(&exp)
    }

    #[test]
    fn eval_many() {
        assert_eq!( "123", run_all("(def foo 123) foo") );
        assert_eq!( "(123 999)", run_all("(def foo 123) (def bar 999) (cons foo (cons bar '()))") );
    }

    #[test]
    fn exception_uncallable_things() {
        assert_eq!( "Exception: UncallableCalled(\"1\")", run_all("(1 2 3)"));
        assert_eq!( "Exception: UncallableCalled(\"nil\")", run_all("(nil 2 3)"));
        assert_eq!( "Exception: UncallableCalled(\"true\")", run_all("(true 2 3)"));
    }

    #[test]
    fn exception_unknown_symbol() {
        assert_eq!( "Exception: UnknownSymbol(\"a\")", run_all("(a 2 3)"));
    }

    #[test]
    fn exception_deep() {
        assert_eq!( "Exception: UncallableCalled(\"10\")", run_all("(cons 10 (10 2 3))"));
    }
}
