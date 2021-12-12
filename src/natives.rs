use crate::values::{Val, Exp};


pub fn minus(operands: Vec<Exp>) -> Exp {
    let op1 = operands[0].borrow();
    let op2 = operands[1].borrow();
    match (&*op1, &*op2) {
        (Val::Number(x), Val::Number(y)) => Val::wrap(Val::Number(x-y)),
        _ => panic!("unsupported operands"),
    }
}

pub fn plus(operands: Vec<Exp>) -> Exp {
    let op1 = operands[0].borrow();
    let op2 = operands[1].borrow();
    match (&*op1, &*op2) {
        (Val::Number(x), Val::Number(y)) => Val::wrap(Val::Number(x+y)),
        _ => panic!("unsupported operands"),
    }
}

pub fn times(operands: Vec<Exp>) -> Exp {
    let op1 = operands[0].borrow();
    let op2 = operands[1].borrow();
    match (&*op1, &*op2) {
        (Val::Number(x), Val::Number(y)) => Val::wrap(Val::Number(x*y)),
        _ => panic!("unsupported operands"),
    }
}

pub fn equals(operands: Vec<Exp>) -> Exp {
    let op1 = operands[0].borrow();
    let op2 = operands[1].borrow();
    match (&*op1, &*op2) {
        (Val::Number(x), Val::Number(y)) => Val::wrap(Val::Bool(x == y)),
        _ => panic!("unsupported operands"),
    }
}
