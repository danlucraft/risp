use crate::risp::expressions::Exp;

pub fn to_string(value: &Exp) -> String {
    match value {
        Exp::Bool(true) => "true".to_owned(),
        Exp::Bool(false) => "false".to_owned(),
        Exp::Nil => "nil".to_owned(),
        Exp::Int(i) => i.to_string(),
        Exp::Atom(a) => a.clone(),
        Exp::BuiltIn(_) => "#BuiltIn".to_owned(),
        Exp::Function(_) => "#Function".to_owned(),
        Exp::List(v) => {
            let mut result = String::new();
            result.push_str("(");
            for (pos, sub_exp) in v.iter().enumerate() {
                result.push_str(&to_string(sub_exp));
                if pos < v.len() - 1 {
                    result.push(' ');
                }
            }
            result.push_str(")");
            result
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
    fn test_to_string_list() {
        assert_eq!("(a b c)".to_owned(), to_string(&parse("(a b c)")));
        assert_eq!("(a (* 2 3) c)".to_owned(), to_string(&parse("(a (* 2 3 ) c)")));
    }

    #[test]
    fn test_to_string_atom() {
        assert_eq!("abc".to_owned(), to_string(&parse("abc")));
    }

    #[test]
    fn test_to_string_boolean() {
        assert_eq!("true".to_owned(), to_string(&Exp::Bool(true)));
        assert_eq!("false".to_owned(), to_string(&Exp::Bool(false)));
    }

    #[test]
    fn test_to_string_int() {
        assert_eq!("104".to_owned(), to_string(&Exp::Int(104)));
        assert_eq!("999".to_owned(), to_string(&Exp::Int(999)));
    }


}