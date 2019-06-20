#[cfg(test)]
mod tests {
    use crate::risp::parser;
    use crate::risp::to_string::to_string;
    use crate::risp::evaluator::eval;
    use crate::risp::environment::Env;
    use crate::risp::loader;

    fn run_with_prelude(code: &str) -> String {
        let mut env = Env::new();
        loader::eval_file("lisp/prelude.lisp".to_owned(), &mut env);
        let exp = parser::parse_expression(&mut code.chars().peekable()).unwrap();
        to_string(&eval(&exp, &mut env))
    }

    #[test]
    fn test_null() {
        assert_eq!("true", run_with_prelude("(null? '())"));
        assert_eq!("false", run_with_prelude("(null? 123)"));
    }

    #[test]
    fn test_nth() {
        assert_eq!("1", run_with_prelude("(nth 0 '(1 2 3))"));
        assert_eq!("2", run_with_prelude("(nth 1 '(1 2 3))"));
        assert_eq!("3", run_with_prelude("(nth 2 '(1 2 3))"));
    }
}
