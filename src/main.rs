mod ast;
mod values;
mod env;
mod examples;
mod natives;
mod evaluator;
mod parser;
mod errors;

use std::io::{self, BufRead, Write};

use values::{Exp, Val, print_exp};
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
        initial_env = Env::extend(native.0.to_string(), Val::wrap(Val::Native(native.0.to_string(), native.1)), initial_env.clone())
    }

    // REPL
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("minischeme> ");
    let _ = stdout.flush();
    for line in stdin.lock().lines() {
       let expr = crate::parser::parse(&line.as_ref().unwrap()).unwrap();
        let result = evaluator::cps::eval(expr, initial_env.clone());
        println!("{}", print_exp(result));
        print!("minischeme> ");
        let _ = stdout.flush();
     
    }
}
