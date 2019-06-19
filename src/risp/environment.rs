use crate::risp::expressions::Exp;
use std::collections::HashMap;

pub struct Env<'a> {
    bindings: HashMap<String, Exp>,
    parent: Option<&'a Env<'a>>
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env { bindings: HashMap::new(), parent: None }
    }
}