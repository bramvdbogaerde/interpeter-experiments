use crate::ast::Expr;


pub fn factorial() -> Expr {
    Expr::Letrec {
        name: "fac".to_string(), 
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
                        operator: Box::new(Expr::Ident("*".to_string())),
                        operands: vec![
                            Expr::Ident("x".to_string()),
                            Expr::Apply {
                                operator: Box::new(Expr::Ident("fac".to_string())),
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
            operator: Box::new(Expr::Ident("fac".to_string())),
            operands: vec![
                Expr::Num(5)
            ]
        })
    }
}
