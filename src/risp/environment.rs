use crate::risp::expressions::Exp;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env<'a> {
    bindings: HashMap<String, Exp>,
    parent: Option<&'a Env<'a>>
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env { bindings: HashMap::new(), parent: None }
    }

    pub fn new_with_parent(parent: &'a Env) -> Env<'a> {
        Env { bindings: HashMap::new(), parent: Some(parent) }
    }

    pub fn set(&mut self, key: String, value: Exp) {
        self.bindings.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<Exp> {
        if let Some(opt_value) = self.bindings.get(&key) {
            Some(opt_value.clone())
        } else if let Some(parent_env) = self.parent {
            parent_env.get(key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risp::parser;
    use crate::risp::evaluator::eval;
    use crate::risp::to_string::display_result;

    #[test]
    fn test_label() {
        assert_eq!(
            "(a m (a m c) d)",
            display_result(&eval(&parser::parse(r#"
            (
                (label subst (lambda (x y z)
                               (cond (eq z '()) '()
                                     (atom z)   (cond (eq z y) x 
                                                      true     z)
                                     true       (cons (subst x y (car z))
                                                      (subst x y (cdr z))))))
                'm 'b '(a b (a b c) d)
            )
            "#)[0], &mut Env::new()))
        )

    }

    #[test]
    fn test_parent_getting() {
        let mut parent = Env::new();
        parent.set("p".to_string(), Exp::Int(101));
        let mut child1 = Env::new_with_parent(&mut parent);
        child1.set("c1".to_string(), Exp::Int(202));
        let mut child2 = Env::new_with_parent(&mut child1);
        child2.set("c2".to_string(), Exp::Int(303));
        assert_eq!(Some(Exp::Int(303)), child2.get("c2".to_string()));
        assert_eq!(Some(Exp::Int(202)), child2.get("c1".to_string()));
        assert_eq!(Some(Exp::Int(101)), child2.get("p".to_string()));
        assert_eq!(None, child2.get("qqq".to_string()));
    }

    #[test]
    fn test_def() {
        let mut env = Env::new();
        let exp = &parser::parse("(def num 101)")[0];
        eval(exp, &mut env).ok();
        assert_eq!(Some(Exp::Int(101)), env.get("num".to_string()));
    }

    #[test]
    fn test_resolving() {
        let mut env = Env::new();
        let exp = &parser::parse("(def num 101)")[0];
        eval(exp, &mut env).ok();
        let exp2 = &parser::parse("num")[0];
        assert_eq!(Ok(Exp::Int(101)), eval(exp2, &mut env));
    }
}