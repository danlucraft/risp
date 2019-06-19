use crate::risp::expressions::Exp;
use crate::risp::environment::Env;
use crate::risp::evaluator::eval;
use crate::risp::to_string;

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
    Label,
    Inspect,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    arg_names: Vec<Exp>, // atoms
    body_exps: Vec<Exp>,
    self_name: Option<String> // any exps
}

impl Callable for Function {
    fn call(&self, args: Vec<Exp>, mut env: &mut Env) -> Exp {
        let mut arg_values: Vec<Exp> = vec!();
        for arg in args {
            arg_values.push(eval(&arg, &mut env))
        }
        let mut function_env = Env::new_with_parent(&mut env);
        for (i, arg_name) in self.arg_names.iter().enumerate() {
            if let Exp::Atom(arg_name1) = arg_name {
                function_env.set(arg_name1.to_string(), arg_values[i].clone());
            } else {
                panic!("Arg list contained a non-atom");
            }
        }
        if let Some(name) = &self.self_name {
            function_env.set(name.to_string(), Exp::Function(self.clone()));
        }
        for exp in &self.body_exps[0..(self.body_exps.len()-1)] {
            eval(&exp, &mut function_env);
        }
        eval(&self.body_exps[self.body_exps.len()-1], &mut function_env)
    }
}

impl Callable for BuiltIn {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Exp {
        match self {
            BuiltIn::Inspect => {
                for arg in args {
                    println!("{}", to_string::to_string(&eval(&arg, env)));
                }
                return Exp::Bool(true)
            },
            BuiltIn::Label => {
                if let Exp::Atom(name) = &args[0] {
                    if let Exp::Function(mut function) = eval(&args[1], env) {
                        function.self_name = Some(name.to_owned());
                        return Exp::Function(function);
                    } else {
                        panic!("Second arg to label must yield a function");
                    }
                } else {
                    panic!("First arg to label should be an atom");
                }
            }
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
                    return Exp::Function(Function { arg_names: arg_list.to_vec(), body_exps: args[1..].to_vec(), self_name: None })
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
                let new_head = eval(&args[0], env);
                if let Exp::List(lv) = eval(&args[1], env) {
                    let mut new_vec = lv.clone();
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
                    panic!("cdr expected a list");
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