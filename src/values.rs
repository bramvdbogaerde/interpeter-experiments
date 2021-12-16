use std::{cell::RefCell, rc::Rc};
use crate::{ast::Expr, env::Env};

pub enum Val {  
    Number(usize),
    Proc(Rc<Env>, Vec<Expr>, Vec<String>),
    Native(String, Box<dyn Fn(Vec<Exp>) -> Exp>),
    Bool(bool),
    Str(String),
    Nil
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Val::*;
        match self {
            Number(n) => write!(f, "{}", n),
            Proc(..) => write!(f, "<#:procedure>"),
            Native(name, ..) => write!(f, "<#:native {}>", name),
            Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Str(b) => write!(f, "{}", b),
            Nil => write!(f, "nil")
        }
    }
}

impl Val {
    pub fn wrap(v: Val) -> Exp {
        Rc::new(RefCell::new(v))
    }

    pub fn is_true(v: Exp) -> bool {
        match *v.borrow() {
            Val::Bool(b) => b,
            _ => true
        }
    }
}

pub fn print_exp(v: Exp) -> String {
    format!("{:?}", *v.borrow())
}

pub type Exp = Rc<RefCell<Val>>;
