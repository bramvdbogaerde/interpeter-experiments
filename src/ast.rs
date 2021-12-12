
#[derive(Clone, Debug)]
pub enum Expr {
    Apply {
        operator: Box<Expr>, 
        operands: Vec<Expr>
    },

    Ident(String),

    Num(usize),

    Letrec {
        name: String,
        binding: Box<Expr>,
        body: Box<Expr>,
    },

    Lambda {
        arguments: Vec<String>,
        body: Vec<Expr>,
    },

    If {
        cond: Box<Expr>,
        cnsq: Box<Expr>,
        alt: Box<Expr>
    }
}
