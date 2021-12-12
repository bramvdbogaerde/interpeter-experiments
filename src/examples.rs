use crate::ast::Expr;


pub fn sum() -> Expr {
    Expr::Letrec {
        name: "sum".to_string(), 
        binding: Box::new(Expr::Lambda {
            arguments: vec![format!("x")], 
            body: vec![
                Expr::If {
                    cond: Box::new(Expr::Apply {
                        operator: Box::new(Expr::Ident("=".to_string())),
                        operands: vec![
                            Expr::Ident("x".to_string()),
                            Expr::Num(0)
                        ]
                    }
                    ),
                    cnsq: Box::new(Expr::Num(1)),
                    alt: Box::new(Expr::Apply {
                        operator: Box::new(Expr::Ident("+".to_string())),
                        operands: vec![
                            Expr::Ident("x".to_string()),
                            Expr::Apply {
                                operator: Box::new(Expr::Ident("sum".to_string())),
                                operands: vec![
                                    Expr::Apply {
                                        operator: Box::new(Expr::Ident("-".to_string())),
                                        operands: vec![
                                            Expr::Ident("x".to_string()),
                                            Expr::Num(1)
                                        ]
                                    }
                                ]
                            }
                        ]
                    })
                }
            ]
        }),
        body:      Box::new(Expr::Apply {
            operator: Box::new(Expr::Ident("sum".to_string())),
            operands: vec![
                Expr::Num(10000)
            ]
        })
    }
}


pub fn sum_iter() -> Expr {
    Expr::Letrec {
        name: "sum".to_string(), 
        binding: Box::new(Expr::Lambda {
            arguments: vec![format!("x"), format!("res")], 
            body: vec![
                Expr::If {
                    cond: Box::new(Expr::Apply {
                        operator: Box::new(Expr::Ident("=".to_string())),
                        operands: vec![
                            Expr::Ident("x".to_string()),
                            Expr::Num(0)
                        ]
                    }
                    ),
                    cnsq: Box::new(Expr::Ident("res".to_string())),
                    alt: Box::new(Expr::Apply {
                                operator: Box::new(Expr::Ident("sum".to_string())),
                                operands: vec![
                                    Expr::Apply {
                                        operator: Box::new(Expr::Ident("-".to_string())),
                                        operands: vec![
                                            Expr::Ident("x".to_string()),
                                            Expr::Num(1)
                                        ]
                                    },
                                    Expr::Apply {
                                        operator: Box::new(Expr::Ident("+".to_string())),
                                        operands: vec![
                                            Expr::Ident("x".to_string()),
                                            Expr::Ident("res".to_string())
                                        ]
                                    }
                                ]
                            })
                    }
            ]
        }),
        body: Box::new(Expr::Apply {
            operator: Box::new(Expr::Ident("sum".to_string())),
            operands: vec![
                Expr::Num(10000),
                Expr::Num(0)
            ]
        })
    }
}
