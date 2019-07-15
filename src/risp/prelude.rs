use crate::risp::loader;
use crate::risp::environment::Env;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prelude_no_errors() {
        let mut env = Env::new();
        let result = loader::eval_file("lisp/prelude.lisp".to_owned(), &mut env);
        assert_eq!(result.is_ok(), true);
    }
}
