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

    #[test]
    fn test_and() {
        assert_eq!("true", run_with_prelude("(and true true)"));
        assert_eq!("false", run_with_prelude("(and true false)"));
        assert_eq!("true", run_with_prelude("(and (eq 1 1) (eq true true))"));
        assert_eq!("false", run_with_prelude("(and (eq 1 2) (eq true true))"));
    }

    #[test]
    fn test_not() {
        assert_eq!("false", run_with_prelude("(not true)"));
        assert_eq!("true", run_with_prelude("(not false)"));
        assert_eq!("true", run_with_prelude("(not 1)"));
    }

    #[test]
    fn test_append() {
        assert_eq!("(1 2 3 4)", run_with_prelude("(append '(1 2) '(3 4))"));
        assert_eq!("(3 4)", run_with_prelude("(append '() '(3 4))"));
        assert_eq!("(1 2)", run_with_prelude("(append '(1 2) '())"));
    }

    // #[test]
    // fn test_list() {
    //     assert_eq!("((1 2) (3 4))", run_with_prelude("(zip '(1 2) '(3 4))"));
    // }

    // #[test]
    // fn test_zip() {
    //     assert_eq!("((1 2) (3 4))", run_with_prelude("(zip '(1 2) '(3 4))"));
    // }
}
