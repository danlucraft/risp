use crate::risp::evaluator;

pub fn to_string(value: evaluator::Value) -> String {
    "foo".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!("foo".to_owned(), to_string(evaluator::Value::Bool(true)));
    }
}