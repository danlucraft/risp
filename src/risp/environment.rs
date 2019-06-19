use crate::risp::expressions::Exp;
use std::collections::HashMap;

pub struct Env<'a> {
    pub bindings: HashMap<String, Exp>,
    parent: Option<&'a Env<'a>>
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env { bindings: HashMap::new(), parent: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::evaluator::eval;

    #[test]
    fn test_def() {
        let mut env = Env::new();
        let exp = parser::parse("(def num 101)").unwrap();
        eval(&exp, &mut env);
        assert_eq!(Some(&Exp::Int(101)), env.bindings.get("num"));
    }

    #[test]
    fn test_resolving() {
        let mut env = Env::new();
        let exp = parser::parse("(def num 101)").unwrap();
        eval(&exp, &mut env);
        let exp2 = parser::parse("num").unwrap();
        assert_eq!(Exp::Int(101), eval(&exp2, &mut env));
    }
}