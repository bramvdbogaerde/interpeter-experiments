mod ast;
mod values;
mod env;
mod examples;
mod natives;
mod evaluator;


use values::{Exp, Val};
use natives::*;
use env::Env;

fn main() {
    let natives: Vec<(&'static str, Box<dyn Fn(Vec<Exp>) -> Exp>)> = vec![
        ("-", Box::new(minus)),
        ("*", Box::new(times)),
        ("=", Box::new(equals)),
        ("+", Box::new(plus))
    ];

    let mut initial_env = Env::empty();
    for native in natives {
        initial_env = Env::extend(native.0.to_string(), Val::wrap(Val::Native(native.1)), initial_env.clone())
    }

    let factorial_program = examples::factorial();
    let result = evaluator::recursive::eval(factorial_program.clone(), initial_env.clone());
    println!("Result (recursive eval) is {:?}", result);
    let result2 = evaluator::cps::eval(factorial_program, initial_env);
    println!("Result (cps eval) {:?}", result2);

}
