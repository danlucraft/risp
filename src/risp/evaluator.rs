use crate::risp::expressions::Exp;
use crate::risp::function::*;
use crate::risp::environment::Env;
use crate::risp::builtins::BuiltIn;

pub fn eval_all(exps: &Vec<Exp>, env: &mut Env) -> Exp {
    let mut result = Exp::Bool(true);
    for exp in exps {
        result = eval(exp, env);
    }
    result
}

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
            "+"      => Exp::BuiltIn(BuiltIn::Add),
            "-"      => Exp::BuiltIn(BuiltIn::Subtract),
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

    fn run_all(code: &str) -> String {
        let exps = parser::parse(code);
        let mut env = Env::new();
        let exp = eval_all(&exps, &mut env);
        to_string(&exp)
    }

    #[test]
    fn eval_many() {
        assert_eq!( "123", run_all("(def foo 123) foo") );
        assert_eq!( "(123 999)", run_all("(def foo 123) (def bar 999) (cons foo (cons bar '()))") );
    }

}