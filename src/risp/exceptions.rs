use crate::risp::expressions::Exp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exception {
    pub etype: ExceptionType,
    pub message: String,
    pub backtrace: Vec<Exp>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExceptionType {
    ArgumentError,
    SyntaxError,
    UncallableCalled,
    UnknownSymbol,
    AssertionFailed,
}
