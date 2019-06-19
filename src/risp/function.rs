use crate::risp::expressions::Exp;
use crate::risp::environment::Env;
use crate::risp::evaluator::eval;

pub trait Callable {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Exp;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltIn {
    Quote,
    Atom,
    Eq,
    Car,
    Cdr,
    Cons,
    Cond,
    Lambda,
    Def,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    args: Vec<Exp>,     // atoms
    body_exps: Vec<Exp> // any exps
}

impl Callable for Function {
    fn call(&self, _args: Vec<Exp>, env: &mut Env) -> Exp {
        for exp in &self.body_exps[0..(self.body_exps.len()-1)] {
            eval(&exp, env);
        }
        eval(&self.body_exps[self.body_exps.len()-1], env)
    }
}

impl Callable for BuiltIn {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Exp {
        match self {
            BuiltIn::Def => {
                if let Exp::Atom(name) = &args[0] {
                    let value = eval(&args[1], env);
                    env.set(name.clone(), value);
                    return Exp::Bool(true);
                } else {
                    panic!("First arg to def should be an atom");
                }
            },
            BuiltIn::Lambda => {
                if let Exp::List(arg_list) = &args[0] {
                    return Exp::Function(Function { args: arg_list.to_vec(), body_exps: args[1..].to_vec() })
                } else {
                    panic!("First arg to lambda should be arg list");
                }
            }
            BuiltIn::Quote => return args[0].clone(),
            BuiltIn::Atom => {
                if let Exp::Atom(_) = eval(&args[0], env) {
                    return Exp::Bool(true)
                } else {
                    return Exp::Bool(false)
                }
            },
            BuiltIn::Cond => {
                for curr in 0..(args.len()/2) {
                    if eval(&args[curr*2], env) == Exp::Bool(true) {
                        return eval(&args[curr*2 + 1], env);
                    }
                }
                return Exp::List(vec!())
            },
            BuiltIn::Cons => {
                if let Exp::List(lv) = eval(&args[1], env) {
                    let mut new_vec = lv.clone();
                    let new_head = eval(&args[0], env);
                    new_vec.insert(0, new_head.clone());
                    return Exp::List(new_vec);
                } else {
                    panic!("cons expected a list");
                }
            },
            BuiltIn::Car => {
                if let Exp::List(v) = eval(&args[0], env) {
                    return v[0].clone();
                } else {
                    panic!("car expected a list");
                }
            },
            BuiltIn::Cdr => {
                if let Exp::List(vec) = eval(&args[0], env) {
                    if vec.len() > 1 {
                        return Exp::List(vec[1..].to_vec());
                    } else {
                        return Exp::List(vec!());
                    }
                } else {
                    panic!("car expected a list");
                }

            }
            BuiltIn::Eq => {
                let l = eval(&args[0], env);
                let r = eval(&args[1], env);
                if let Exp::Atom(x) = l {
                    if let Exp::Atom(y) = r {
                        if x == y {
                            return Exp::Bool(true);
                        }
                    }
                } else if let Exp::List(x) = eval(&args[0], env) {
                    if let Exp::List(y) = eval(&args[1], env) {
                        if x.len() == 0 && y.len() == 0 {
                            return Exp::Bool(true);
                        }
                    }
                } else if let Exp::Bool(xb) = eval(&args[0], env) {
                    if let Exp::Bool(yb) = eval(&args[1], env) {
                        if xb == yb {
                            return Exp::Bool(true);
                        }
                    }
                }
                return Exp::Bool(false);
            }
        }
    }
}