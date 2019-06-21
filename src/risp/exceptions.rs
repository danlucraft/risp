#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exception {
    ArgumentError(String),
    SyntaxError(String),
    UncallableCalled(String),
    UnknownSymbol(String),
    AssertionFailed(String)
}
