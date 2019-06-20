use crate::risp::function;
use crate::risp::builtins;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Atom(String),
    List(Vec<Exp>),
    Int(i32),
    Bool(bool),
    Nil,
    BuiltIn(builtins::BuiltIn),
    Function(function::Function)
}
