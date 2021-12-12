use crate::ast::Expr;
use crate::env::Env; 
use crate::values::{Exp, Val};
use std::rc::Rc;


fn eval_sequence(expressions: Vec<Expr>, env: Rc<Env>) -> Exp {
    for expression in expressions[0..expressions.len()-1].iter() {
        eval(expression.clone(), env.clone());
    }

    eval(expressions[expressions.len()-1].clone(), env)
}

fn apply(operator: Exp, operands: Vec<Exp>) -> Exp {
    match &*operator.borrow() {
        Val::Proc(lexical_env, body, arguments) =>  {
            // add the arguments to the lexical env 
            let mut extended_env = lexical_env.clone();
            for (name, operand) in arguments.iter().zip(operands) {
                extended_env = Env::extend((*name).clone(), operand, extended_env.clone());
            }

            let result = eval_sequence(body.clone(), extended_env);
            result
        },
        Val::Native(f) => f(operands),
        _ => panic!("unsupported apply")

    }
}

pub fn eval(exp: Expr, env: Rc<Env>) -> Exp {
    use Expr::*;
    match exp {
        Ident(name) => env.lookup(name),
        Num(n) => Val::wrap(Val::Number(n)),
        //Begin { expressions } => eval_sequence(expressions, env),
        If { cond, cnsq, alt } =>  {
            let cond_value = eval(*cond, env.clone());
            if Val::is_true(cond_value) {
                eval(*cnsq, env.clone())
            } else {
                eval(*alt, env.clone())
            }
        },
        Letrec { name, binding, body } =>  {
            // define is equal to a letrec, so we extend the env first
            let extended_env = Env::extend(name.clone(), Val::wrap(Val::Nil), env);
            let binding_eval = eval(*binding, extended_env.clone());
            extended_env.update(name, binding_eval.clone());
            // evaluate the body in the extended env
            let res = eval(*body, extended_env);
            // no tail call optimization yet
            res
        },
        Apply { operator, operands } =>  {
            // first evaluate the operator
            let operator_eval = eval(*operator, env.clone());
            // then the operands 
            let operands_eval = operands.iter().map(|operand| eval(operand.clone(),  env.clone())).collect::<Vec<Exp>>();
            apply(operator_eval, operands_eval)

        },

        Lambda { arguments, body } => 
            Val::wrap(Val::Proc(env.clone(), body, arguments)),

    }
    }

