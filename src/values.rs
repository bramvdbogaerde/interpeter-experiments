use std::{cell::RefCell, rc::Rc};
use crate::{ast::Expr, env::Env};

pub enum Val {  
    Number(usize),
    Proc(Rc<Env>, Vec<Expr>, Vec<String>),
    Native(Box<dyn Fn(Vec<Exp>) -> Exp>),
    Bool(bool),
    Nil
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Val::*;
        match self {
            Number(n) => write!(f, "Number({})", n),
            Proc(..) => write!(f, "Proc()"),
            Native(..) => write!(f, "Native()"),
            Bool(b) => write!(f, "Bool({})", b),
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

pub type Exp = Rc<RefCell<Val>>;
