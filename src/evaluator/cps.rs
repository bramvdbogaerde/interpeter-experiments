/// CPS style evaluator.
/// This could be improved by using Rc's as well for the AST, since it is often cloned to satisfy
/// the borrowing and ownership rules of Rust.


use crate::{values::{Val, Exp}, env::{EnvRef, Env}, ast::Expr};
use std::{cell::RefCell, rc::Rc};



#[derive(Clone)]
enum ERes {
    Value(Exp),
    Stop(Exp)
}

trait Continuation  {
    fn apply(self: Box<Self>, vlu: Exp) -> ERes;
}

impl<F> Continuation for F 
    where F: FnOnce(Exp) -> ERes {
        fn apply(self: Box<Self>, vlu: Exp) -> ERes {
            self(vlu)
        }
}

type ContinuationRef = Box<dyn Continuation>;

pub struct Evaluator {
    continuations: RefCell<Vec<ContinuationRef>>,
}

type EvRef = Rc<Evaluator>;

impl Evaluator {
    fn push_continuation(&self, cnt: ContinuationRef) {
        //println!("DEBUG: size of continuation stack {}", self.continuations.borrow().len());
        self.continuations.borrow_mut().push(cnt);
    }
    
    fn continue_with(&self, v: Exp) -> ERes {
        let cnt = self.continuations.borrow_mut().pop().expect("At least one continuation before stop");

        //println!("DEBUG: size of continuation stack {}", self.continuations.borrow().len());
        cnt.apply(v)
    }

    fn apply(ev: EvRef, operator: Exp, operands: Vec<Exp>) -> ERes {
        match &*operator.borrow() {
            Val::Proc(lexical_env, body, arguments) =>  {
                // add the arguments to the lexical env 
                let mut extended_env = lexical_env.clone();
                for (name, operand) in arguments.iter().zip(operands) {
                    extended_env = Env::extend((*name).clone(), operand, extended_env.clone());
                }

                let result = Evaluator::eval_sequence(ev, body.clone(), extended_env, 0);
                result
            },
            Val::Native(_, f) => ERes::Value(f(operands)),
            _ => panic!("unsupported apply")
        }
    }

    fn eval_sequence(ev: EvRef, sequence: Vec<Expr>, env: EnvRef, position: usize) -> ERes {
        if position < sequence.len()-1 {
            let continue_env = env.clone();
            let first_expression = sequence[position].clone();
            let inner_ev = ev.clone();
            ev.push_continuation(Box::new(move |_value|{
                Evaluator::eval_sequence(inner_ev, sequence, continue_env, position+1)
            }));

            ev.eval(first_expression, env)
        } else {
            ev.eval(sequence[position].clone(), env)
        }
    }

    fn evaluate_operands(ev: EvRef, operator: Exp, operands: Vec<Expr>, mut operand_values: Vec<Exp>, env: EnvRef, position: usize) -> ERes {
        if position >= operands.len() {
            Evaluator::apply(ev, operator, operand_values)
        } else {
            let operand = operands[position].clone();
            let continue_env = env.clone();
            let inner_ev = ev.clone();
            ev.push_continuation(Box::new(move |operand| {
                operand_values.push(operand);
                Evaluator::evaluate_operands(inner_ev, operator, operands, operand_values, continue_env, position+1)
            }));

            ev.eval(operand, env)
        }
    }

    fn eval(ev: EvRef, exp: Expr, env: EnvRef) -> ERes {
        use Expr::*;
        match exp {
            Ident(name) => ERes::Value(env.lookup(name.to_string())), 
            Str(s) => ERes::Value(Val::wrap(Val::Str(s))),
            Bool(b) => ERes::Value(Val::wrap(Val::Bool(b))),
            Nil => ERes::Value(Val::wrap(Val::Nil)),
            Num(n) => ERes::Value(Val::wrap(Val::Number(n))), 
            If { cond, cnsq, alt } =>  {
                let continue_env = env.clone();
                let inner_ev = ev.clone();
                let k = Box::new(move |value| {
                    if Val::is_true(value) {
                        inner_ev.eval(*cnsq, continue_env)
                    } else {
                        inner_ev.eval(*alt, continue_env)
                    }
                });

                ev.push_continuation(k);

                ev.eval(*cond, env)
            },

            Letrec { name, binding, body } =>  {
                let extended_env = Env::extend(name.clone(), Val::wrap(Val::Nil), env);
                let continue_env = extended_env.clone();
                let inner_ev = ev.clone();
                let k = Box::new(move |value| {
                    continue_env.update(name, value);
                    inner_ev.eval(*body, continue_env)
                });


                ev.push_continuation(k);

                ev.eval(*binding, extended_env)
            },

            Apply { operator, operands } => {
                let continue_env = env.clone();
                let inner_ev = ev.clone();
                ev.push_continuation(Box::new(move |operator_value| {
                    Evaluator::evaluate_operands(inner_ev, operator_value, operands, Vec::new(), continue_env, 0)
                }));


                ev.eval(*operator, env)
            }

            Lambda { arguments, body } => 
                ERes::Value(Val::wrap(Val::Proc(env.clone(), body, arguments))),
            
        }
    }


    pub  fn new() -> Evaluator {
        Evaluator { continuations: RefCell::new(Vec::new()) }
    }
}

trait EvRefOps {
    fn eval(&self, exp: Expr, env: EnvRef) -> ERes;
}

impl EvRefOps for Rc<Evaluator> {
    fn eval(&self, exp: Expr, env: EnvRef) -> ERes {
        Evaluator::eval(self.clone(), exp, env)
    }

}

struct HaltContinuation;

impl Continuation for HaltContinuation {
    fn apply(self: Box<Self>, vlu: Exp) -> ERes {
        ERes::Stop(vlu)
    }
}
pub fn eval(ex: Expr, env: EnvRef) -> Exp { 
    let evaluator = Rc::new(Evaluator::new());
    evaluator.push_continuation(Box::new(HaltContinuation));
    let mut result = Evaluator::eval(evaluator.clone(), ex, env); 
    loop {
        use ERes::*;
        match result {
            Value(v) => 
                result = evaluator.continue_with(v),
            Stop(v) => break v
        }
    }
}
