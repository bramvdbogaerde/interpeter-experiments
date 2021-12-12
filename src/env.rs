use crate::values::{Val, Exp};
use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
pub enum Env {
    Empty,
    NonEmpty {
        name: String, 
        value: RefCell<Exp>,
        next: Rc<Env>
    }
}

impl Env {
    pub fn empty() -> Rc<Env> {
        Rc::new(Env::Empty)
    }

    pub fn extend(name: String, value: Exp, next: Rc<Env>) -> Rc<Env> {
        Rc::new(Env::NonEmpty {
            name, 
            value: RefCell::new(value),
            next 
        })
    }

    pub fn update(&self, lookup_name: String, new_value: Exp) {
        match self {
            Env::Empty => panic!("binding not found {}", lookup_name),
            Env::NonEmpty { name, value, next } => 
                if lookup_name == *name {
                    *value.borrow_mut() = new_value;
                } else {
                    next.lookup(lookup_name);
                }
        }
    }

    pub fn lookup(&self, lookup_name: String) -> Exp {
        match self {
            Env::Empty => panic!("binding not found {}", lookup_name),
            Env::NonEmpty { name, value, next } => 
                if lookup_name == *name {
                    (*value.borrow()).clone()
                } else {
                    next.lookup(lookup_name)
                }
        }
    }
}

pub type EnvRef = Rc<Env>;
