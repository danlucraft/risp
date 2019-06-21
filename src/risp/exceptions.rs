use crate::risp::expressions::Exp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exception {
    ArgumentError(String, Vec<Exp>),
    SyntaxError(String, Vec<Exp>),
    UncallableCalled(String, Vec<Exp>),
    UnknownSymbol(String, Vec<Exp>),
    AssertionFailed(String, Vec<Exp>)
}
