use crate::risp::evaluator;
use crate::risp::parser;

pub fn to_string_exp(exp: &parser::Exp) -> String {
    match exp {
        parser::Exp::Atom(a) => a.clone(),
        parser::Exp::List(v) => {
            let mut result = String::new();
            result.push_str("(");
            for (pos, sub_exp) in v.iter().enumerate() {
                result.push_str(&to_string_exp(sub_exp));
                if pos < v.len() - 1 {
                    result.push(' ');
                }
            }
            result.push_str(")");
            result
        }
    }
}

pub fn to_string(value: evaluator::Value) -> String {
    match value {
        evaluator::Value::Bool(true) => "true".to_owned(),
        evaluator::Value::Bool(false) => "false".to_owned(),
        evaluator::Value::Int(i) => i.to_string(),
        evaluator::Value::Exp(exp) => to_string_exp(&exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(code: &str) -> parser::Exp {
        parser::parse_expression(&mut code.chars().peekable()).unwrap()
    }

    #[test]
    fn test_to_string_list() {
        assert_eq!("(a b c)".to_owned(), to_string(evaluator::Value::Exp(parse("(a b c)"))));
        assert_eq!("(a (* 2 3) c)".to_owned(), to_string(evaluator::Value::Exp(parse("(a (* 2 3 ) c)"))));
    }

    #[test]
    fn test_to_string_atom() {
        assert_eq!("abc".to_owned(), to_string(evaluator::Value::Exp(parse("abc"))));
    }

    #[test]
    fn test_to_string_boolean() {
        assert_eq!("true".to_owned(), to_string(evaluator::Value::Bool(true)));
        assert_eq!("false".to_owned(), to_string(evaluator::Value::Bool(false)));
    }

    #[test]
    fn test_to_string_int() {
        assert_eq!("104".to_owned(), to_string(evaluator::Value::Int(104)));
        assert_eq!("999".to_owned(), to_string(evaluator::Value::Int(999)));
    }


}