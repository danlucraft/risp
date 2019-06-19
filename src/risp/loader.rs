use std::fs;

use crate::risp::parser;
use crate::risp::evaluator;
use crate::risp::environment::Env;
use crate::risp::expressions::Exp;

pub fn eval_file(path: String, env: &mut Env) -> Exp {
    let file: String = String::from_utf8(fs::read(path).unwrap()).unwrap();
    eval_code(&file, env)
}

pub fn eval_code(code: &String, env: &mut Env) -> Exp {
    let exps = parser::parse(code);
    let result = evaluator::eval_all(&exps, env);
    result
}