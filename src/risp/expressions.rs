use crate::risp::function;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Atom(String),
    List(Vec<Exp>),
    Int(i32),
    Bool(bool),
    BuiltIn(function::BuiltIn),
    Function(function::Function)
}
