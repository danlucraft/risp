use crate::risp::expressions::Exp;
use crate::risp::exceptions::Exception;
use crate::risp::environment::Env;
use crate::risp::evaluator::eval;

pub trait Callable {
    fn call(&self, args: Vec<Exp>, env: &mut Env) -> Result<Exp, Exception>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub arg_names: Vec<Exp>, // atoms
    pub body_exps: Vec<Exp>,
    pub self_name: Option<String> // any exps
}

impl Callable for Function {
    fn call(&self, args: Vec<Exp>, mut env: &mut Env) -> Result<Exp, Exception> {
        if args.len() != self.arg_names.len() {
            panic!("function {:?} expected {} args but received {}", self.self_name, self.arg_names.len(), args.len());
        }
        let mut arg_values: Vec<Exp> = vec!();
        for arg in args {
            arg_values.push(eval(&arg, &mut env)?)
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
            eval(&exp, &mut function_env)?;
        }
        eval(&self.body_exps[self.body_exps.len()-1], &mut function_env)
    }
}
